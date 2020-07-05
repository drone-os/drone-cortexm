#![cfg_attr(feature = "std", allow(unreachable_code, unused_variables, unused_mut))]

use super::{Data, ProcData, Yielder};
use crate::{
    fib::{Fiber, FiberRoot, FiberState},
    sv::Switch,
};
use ::alloc::alloc;
use core::{
    alloc::Layout,
    cmp::max,
    marker::{PhantomData, Unpin},
    mem::{align_of, size_of},
    pin::Pin,
};

/// Stackful fiber for [`FnMut`] closure.
///
/// Can be created with [`fib::new_proc`](crate::fib::new_proc),
/// [`fib::new_proc_unchecked`](crate::fib::new_proc_unchecked),
/// [`fib::new_proc_unprivileged`](crate::fib::new_proc_unprivileged),
/// [`fib::new_proc_unprivileged_unchecked`](crate::fib::new_proc_unprivileged_unchecked).
pub struct FiberProc<Sv, I, Y, R, F>
where
    Sv: Switch<ProcData<I, Y, R>>,
    F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
    F: Send + 'static,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    stack_bottom: *mut u8,
    stack_ptr: *const u8,
    stack_size: usize,
    _f: PhantomData<*const F>,
    _sv: PhantomData<*const Sv>,
    _input: PhantomData<*const I>,
    _yield: PhantomData<*const Y>,
    _return: PhantomData<*const R>,
}

impl<Sv, I, Y, R, F> FiberProc<Sv, I, Y, R, F>
where
    Sv: Switch<ProcData<I, Y, R>>,
    F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
    F: Send + 'static,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    pub(super) unsafe fn new(stack_size: usize, unprivileged: bool, unchecked: bool, f: F) -> Self {
        if !unchecked {
            #[cfg(feature = "memory-protection-unit")]
            mpu::check();
        }
        let stack_bottom = alloc::alloc(layout(stack_size));
        if stack_bottom.is_null() {
            panic!("Stack allocation failure");
        }
        let stack_ptr = Self::stack_init(stack_bottom, stack_size, unprivileged, unchecked, f);
        Self {
            stack_bottom,
            stack_ptr,
            stack_size,
            _f: PhantomData,
            _sv: PhantomData,
            _input: PhantomData,
            _yield: PhantomData,
            _return: PhantomData,
        }
    }

    unsafe fn stack_init(
        stack_bottom: *mut u8,
        stack_size: usize,
        unprivileged: bool,
        unchecked: bool,
        f: F,
    ) -> *const u8 {
        assert!(
            stack_size
                >= size_of::<ProcData<I, Y, R>>()
                    + (align_of::<ProcData<I, Y, R>>() - 1)
                    + size_of::<F>()
                    + (align_of::<F>() - 1)
                    + 4
                    + 16
                    + 2
                    + guard_size(unchecked),
            "insufficient stack size",
        );
        let stack_ptr = stack_bottom.add(stack_size);
        let data_ptr = Self::stack_reserve::<ProcData<I, Y, R>>(stack_ptr);
        let fn_ptr = Self::stack_reserve::<F>(data_ptr) as *mut F;
        fn_ptr.write(f);
        let mut stack_ptr = fn_ptr as *mut u32;
        // Align the stack to double word.
        if (stack_ptr as usize).trailing_zeros() < 3 {
            stack_ptr = stack_ptr.sub(1);
        }
        // xPSR
        stack_ptr = stack_ptr.sub(1);
        stack_ptr.write(0x0100_0000);
        // PC
        stack_ptr = stack_ptr.sub(1);
        stack_ptr.write(Self::handler as usize as u32);
        // LR, R12, R3, R2
        stack_ptr = stack_ptr.sub(4);
        stack_ptr.write_bytes(0, 4);
        // R1
        stack_ptr = stack_ptr.sub(1);
        stack_ptr.write(data_ptr as u32);
        // R0
        stack_ptr = stack_ptr.sub(1);
        stack_ptr.write(fn_ptr as u32);
        // LR
        stack_ptr = stack_ptr.sub(1);
        stack_ptr.write(0xFFFF_FFFD);
        // R11, R10, R9, R8, R7, R6, R5, R4
        stack_ptr = stack_ptr.sub(8);
        stack_ptr.write_bytes(0, 8);
        // CONTROL
        stack_ptr = stack_ptr.sub(1);
        stack_ptr.write(if unprivileged { 0b11 } else { 0b10 });
        // MPU CONFIG
        stack_ptr = stack_ptr.sub(1);
        stack_ptr.write(mpu_config(unchecked, stack_bottom));
        stack_ptr as *const u8
    }

    unsafe fn stack_reserve<T>(mut stack_ptr: *mut u8) -> *mut u8 {
        if size_of::<T>() != 0 {
            let align = max(align_of::<T>(), 4);
            stack_ptr = stack_ptr.sub(size_of::<T>());
            stack_ptr = stack_ptr.sub((stack_ptr as usize) & (align - 1));
        }
        stack_ptr
    }

    unsafe extern "C" fn handler(fn_ptr: *mut F, mut data_ptr: *mut ProcData<I, Y, R>) {
        let yielder = Yielder::new();
        let input = data_ptr.read().into_input();
        let output = Data::from_output(FiberState::Complete((*fn_ptr)(input, yielder)));
        data_ptr.write(output);
        Sv::switch_back(&mut data_ptr);
    }

    unsafe fn data_ptr(&mut self) -> *mut ProcData<I, Y, R> {
        let data_size = size_of::<ProcData<I, Y, R>>();
        self.stack_bottom.add(self.stack_size - data_size) as _
    }
}

impl<Sv, I, Y, R, F> Drop for FiberProc<Sv, I, Y, R, F>
where
    Sv: Switch<ProcData<I, Y, R>>,
    F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
    F: Send + 'static,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    fn drop(&mut self) {
        unsafe { alloc::dealloc(self.stack_bottom, layout(self.stack_size)) };
    }
}

impl<Sv, I, Y, R, F> Fiber for FiberProc<Sv, I, Y, R, F>
where
    Sv: Switch<ProcData<I, Y, R>>,
    F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
    F: Send + 'static,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    type Input = I;
    type Return = R;
    type Yield = Y;

    fn resume(mut self: Pin<&mut Self>, input: I) -> FiberState<Y, R> {
        #[cfg(feature = "std")]
        return unimplemented!();
        unsafe {
            let data_ptr = self.data_ptr();
            data_ptr.write(Data::from_input(input));
            Sv::switch_context(data_ptr, &mut self.stack_ptr);
            data_ptr.read().into_output()
        }
    }
}

#[allow(clippy::unused_unit)]
impl<Sv, F> FiberRoot for FiberProc<Sv, (), (), (), F>
where
    Sv: Switch<ProcData<(), (), ()>>,
    F: FnMut((), Yielder<Sv, (), (), ()>) -> (),
    F: Send + 'static,
{
    fn advance(self: Pin<&mut Self>) -> bool {
        match self.resume(()) {
            FiberState::Yielded(()) => true,
            FiberState::Complete(()) => false,
        }
    }
}

unsafe impl<Sv, I, Y, R, F> Send for FiberProc<Sv, I, Y, R, F>
where
    Sv: Switch<ProcData<I, Y, R>>,
    F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
    F: Send + 'static,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
}

impl<Sv, I, Y, R, F> Unpin for FiberProc<Sv, I, Y, R, F>
where
    Sv: Switch<ProcData<I, Y, R>>,
    F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
    F: Send + 'static,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
}

fn guard_size(unchecked: bool) -> usize {
    if !unchecked {
        #[cfg(feature = "memory-protection-unit")]
        return mpu::guard_size();
    }
    1
}

#[allow(unused_variables)]
unsafe fn mpu_config(unchecked: bool, stack_bottom: *mut u8) -> u32 {
    if !unchecked {
        #[cfg(feature = "memory-protection-unit")]
        return mpu::config(stack_bottom);
    }
    0
}

unsafe fn layout(stack_size: usize) -> Layout {
    Layout::from_size_align_unchecked(stack_size, 1)
}

#[cfg(feature = "memory-protection-unit")]
mod mpu {
    use crate::{map::reg::mpu, reg::prelude::*};
    use drone_core::{bitfield::Bitfield, token::Token};

    const GUARD_SIZE: u32 = 5;

    pub(super) fn check() {
        #[cfg(feature = "std")]
        return;
        if unsafe { mpu::Type::<Srt>::take().load().dregion() == 0 } {
            panic!("MPU not present");
        }
    }

    pub(super) fn guard_size() -> usize {
        1 + (1 << GUARD_SIZE + 1) + (1 << GUARD_SIZE + 1) - 1
    }

    #[allow(clippy::cast_ptr_alignment)]
    pub(super) unsafe fn config(mut guard_ptr: *mut u8) -> u32 {
        let rbar_bits = |region, addr| {
            mpu::Rbar::<Srt>::take()
                .default()
                .write_addr(addr >> 5)
                .set_valid()
                .write_region(region)
                .val()
                .bits()
        };
        let rasr_bits = || {
            mpu::Rasr::<Srt>::take()
                .default()
                .write_ap(0b000)
                .write_size(GUARD_SIZE)
                .set_enable()
                .val()
                .bits()
        };
        if (guard_ptr as usize).trailing_zeros() <= GUARD_SIZE {
            guard_ptr = guard_ptr
                .add((1 << GUARD_SIZE + 1) - ((guard_ptr as usize) & (1 << GUARD_SIZE + 1) - 1));
        }
        let mut table_ptr = guard_ptr as *mut u32;
        table_ptr.write(rbar_bits(0, guard_ptr as u32));
        table_ptr = table_ptr.add(1);
        table_ptr.write(rasr_bits());
        table_ptr = table_ptr.add(1);
        for i in 1..8 {
            table_ptr.write(rbar_bits(i, 0));
            table_ptr = table_ptr.add(1);
            table_ptr.write(0);
            table_ptr = table_ptr.add(1);
        }
        table_ptr.sub(16) as u32
    }
}
