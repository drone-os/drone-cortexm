use super::{Data, StackData, Yielder};
use alloc::heap::{Alloc, Heap, Layout};
use core::cmp::max;
use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use fib::{Fiber, FiberRoot, FiberState};
use sv::Switch;

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
  stack_top: *mut u8,
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
  pub(super) fn new(stack_size: usize, unprivileged: bool, f: F) -> Self {
    unsafe {
      let stack_top = alloc(stack_size);
      let stack_ptr = Self::stack_init(stack_top, stack_size, unprivileged, f);
      Self {
        stack_top,
        stack_ptr,
        stack_size,
        _f: PhantomData,
        _sv: PhantomData,
        _input: PhantomData,
        _yield: PhantomData,
        _return: PhantomData,
      }
    }
  }

  unsafe fn stack_init(
    stack_top: *mut u8,
    stack_size: usize,
    unprivileged: bool,
    f: F,
  ) -> *const u8 {
    assert!(
      stack_size
        >= size_of::<StackData<I, Y, R>>()
          + (align_of::<StackData<I, Y, R>>() - 1) + size_of::<F>()
          + (align_of::<F>() - 1) + 4 + 16 + 2,
      "insufficient stack size",
    );
    let stack_ptr = stack_top.add(stack_size);
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
    self.stack_top.add(self.stack_size - data_size) as _
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
    unsafe { dealloc(self.stack_top, self.stack_size) };
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
  Heap
    .alloc(layout(stack_size))
    .unwrap_or_else(|err| Heap.oom(err))
}

unsafe fn dealloc(stack_top: *mut u8, stack_size: usize) {
  Heap.dealloc(stack_top, layout(stack_size));
}

unsafe fn layout(stack_size: usize) -> Layout {
  Layout::from_size_align_unchecked(stack_size, 1)
}
