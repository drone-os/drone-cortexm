use super::{Data, StackData, Yielder};
use alloc::alloc::Global;
use core::alloc::{GlobalAlloc, Layout, Opaque};
use core::cmp::max;
use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use drone_core::bitfield::Bitfield;
use fib::{Fiber, FiberRoot, FiberState};
use reg::mpu;
use reg::prelude::*;
use sv::Switch;

const GUARD_SIZE: u32 = 5;

/// A stackful fiber.
pub struct FiberStack<Sv, I, Y, R, F>
where
  Sv: Switch<StackData<I, Y, R>>,
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

impl<Sv, I, Y, R, F> FiberStack<Sv, I, Y, R, F>
where
  Sv: Switch<StackData<I, Y, R>>,
  F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  pub(super) unsafe fn new(
    stack_size: usize,
    unprivileged: bool,
    unchecked: bool,
    f: F,
  ) -> Self {
    if !unchecked && mpu::Type::<Srt>::new().load().dregion() == 0 {
      panic!("MPU not present");
    }
    let stack_bottom = alloc(stack_size);
    let stack_ptr =
      Self::stack_init(stack_bottom, stack_size, unprivileged, unchecked, f);
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
        >= size_of::<StackData<I, Y, R>>()
          + (align_of::<StackData<I, Y, R>>() - 1) + size_of::<F>()
          + (align_of::<F>() - 1) + 4 + 16 + 2 + if unchecked {
          1
        } else {
          1 + (1 << GUARD_SIZE + 1) + (1 << GUARD_SIZE + 1) - 1
        },
      "insufficient stack size",
    );
    let stack_ptr = stack_bottom.add(stack_size);
    let data_ptr = Self::stack_reserve::<StackData<I, Y, R>>(stack_ptr);
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
    stack_ptr.write(Self::handler as u32);
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
    stack_ptr.write(if unchecked {
      0
    } else {
      Self::mpu_config(stack_bottom)
    });
    stack_ptr as *const u8
  }

  #[cfg_attr(feature = "clippy", allow(cast_ptr_alignment))]
  unsafe fn mpu_config(mut guard_ptr: *mut u8) -> u32 {
    let rbar = mpu::Rbar::<Srt>::new();
    let rasr = mpu::Rasr::<Srt>::new();
    let rbar_bits = |region, addr| {
      rbar
        .default()
        .write_addr(addr >> 5)
        .set_valid()
        .write_region(region)
        .val()
        .bits()
    };
    if (guard_ptr as usize).trailing_zeros() <= GUARD_SIZE {
      guard_ptr = guard_ptr.add(
        (1 << GUARD_SIZE + 1)
          - ((guard_ptr as usize) & (1 << GUARD_SIZE + 1) - 1),
      );
    }
    let mut table_ptr = guard_ptr as *mut u32;
    table_ptr.write(rbar_bits(0, guard_ptr as u32));
    table_ptr = table_ptr.add(1);
    table_ptr.write(
      rasr
        .default()
        .write_ap(0b000)
        .write_size(GUARD_SIZE)
        .set_enable()
        .val()
        .bits(),
    );
    table_ptr = table_ptr.add(1);
    for i in 1..8 {
      table_ptr.write(rbar_bits(i, 0));
      table_ptr = table_ptr.add(1);
      table_ptr.write(0);
      table_ptr = table_ptr.add(1);
    }
    table_ptr.sub(16) as u32
  }

  unsafe fn stack_reserve<T>(mut stack_ptr: *mut u8) -> *mut u8 {
    if size_of::<T>() != 0 {
      let align = max(align_of::<T>(), 4);
      stack_ptr = stack_ptr.sub(size_of::<T>());
      stack_ptr = stack_ptr.sub((stack_ptr as usize) & (align - 1));
    }
    stack_ptr
  }

  unsafe extern "C" fn handler(
    fn_ptr: *mut F,
    mut data_ptr: *mut StackData<I, Y, R>,
  ) {
    let input = data_ptr.read().input;
    let yielder = Yielder::new();
    let output = FiberState::Complete((*fn_ptr)(input, yielder));
    data_ptr.write(Data { output });
    Sv::switch_back(&mut data_ptr);
  }

  unsafe fn data_ptr(&self) -> *mut StackData<I, Y, R> {
    let data_size = size_of::<StackData<I, Y, R>>();
    self.stack_bottom.add(self.stack_size - data_size) as _
  }
}

impl<Sv, I, Y, R, F> Drop for FiberStack<Sv, I, Y, R, F>
where
  Sv: Switch<StackData<I, Y, R>>,
  F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  fn drop(&mut self) {
    unsafe { dealloc(self.stack_bottom, self.stack_size) };
  }
}

impl<Sv, I, Y, R, F> Fiber for FiberStack<Sv, I, Y, R, F>
where
  Sv: Switch<StackData<I, Y, R>>,
  F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  type Input = I;
  type Yield = Y;
  type Return = R;

  fn resume(&mut self, input: I) -> FiberState<Y, R> {
    unsafe {
      let data_ptr = self.data_ptr();
      data_ptr.write(Data { input });
      Sv::switch_context(data_ptr, &mut self.stack_ptr);
      data_ptr.read().output
    }
  }
}

impl<Sv, F> FiberRoot for FiberStack<Sv, (), (), (), F>
where
  Sv: Switch<StackData<(), (), ()>>,
  F: FnMut((), Yielder<Sv, (), (), ()>) -> (),
  F: Send + 'static,
{
  fn advance(&mut self) -> bool {
    match self.resume(()) {
      FiberState::Yielded(()) => true,
      FiberState::Complete(()) => false,
    }
  }
}

unsafe impl<Sv, I, Y, R, F> Send for FiberStack<Sv, I, Y, R, F>
where
  Sv: Switch<StackData<I, Y, R>>,
  F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
}

unsafe fn alloc(stack_size: usize) -> *mut u8 {
  Global.alloc(layout(stack_size)) as *mut u8
}

unsafe fn dealloc(stack_bottom: *mut u8, stack_size: usize) {
  Global.dealloc(stack_bottom as *mut Opaque, layout(stack_size));
}

unsafe fn layout(stack_size: usize) -> Layout {
  Layout::from_size_align_unchecked(stack_size, 1)
}
