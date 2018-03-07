use alloc::heap::{Alloc, Heap, Layout};
use core::cmp::max;
use core::marker::PhantomData;
use core::mem::{align_of, forget, size_of};
use fib::{Fiber, FiberRoot, FiberState};
use sv::Switch;
use thr::prelude::*;

/// A stackful fiber.
pub struct FiberCont<Sv, I, Y, R, F>
where
  Sv: Switch<DataCont<I, Y, R>>,
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

/// A communication channel for [`FiberCont`](FiberCont).
#[derive(Clone, Copy)]
pub struct Yielder<Sv, I, Y, R>
where
  Sv: Switch<DataCont<I, Y, R>>,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  _sv: PhantomData<*const Sv>,
  _input: PhantomData<*const I>,
  _yield: PhantomData<*const Y>,
  _return: PhantomData<*const R>,
}

#[allow(unions_with_drop_fields)]
pub union Data<I, O> {
  input: I,
  output: O,
}

type DataCont<I, Y, R> = Data<I, FiberState<Y, R>>;

impl<Sv, I, Y, R, F> FiberCont<Sv, I, Y, R, F>
where
  Sv: Switch<DataCont<I, Y, R>>,
  F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  unsafe fn stack_init(stack_ptr: *mut u8, f: F) -> *const u8 {
    unsafe fn reserve<T>(mut stack_ptr: *mut u8) -> *mut u8 {
      if size_of::<T>() != 0 {
        let align = max(align_of::<T>(), 4);
        stack_ptr = stack_ptr.sub(size_of::<T>());
        stack_ptr = stack_ptr.sub((stack_ptr as usize) & (align - 1));
      }
      stack_ptr
    }
    let data_ptr = reserve::<DataCont<I, Y, R>>(stack_ptr);
    let fn_ptr = reserve::<F>(data_ptr) as *mut F;
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
    // R11, R10, R9, R8, R7, R6, R5, R4
    stack_ptr = stack_ptr.sub(8);
    stack_ptr.write_bytes(0, 8);
    stack_ptr as *const u8
  }

  unsafe extern "C" fn handler(
    fn_ptr: *mut F,
    mut data_ptr: *mut DataCont<I, Y, R>,
  ) {
    let input = data_ptr.read().input;
    let yielder = Yielder::new();
    let output = FiberState::Complete((*fn_ptr)(input, yielder));
    data_ptr.write(Data { output });
    Sv::switch_back(&mut data_ptr);
  }

  unsafe fn data_ptr(&self) -> *mut DataCont<I, Y, R> {
    let data_size = size_of::<DataCont<I, Y, R>>();
    self.stack_top.add(self.stack_size - data_size) as _
  }
}

impl<Sv, I, Y, R, F> Fiber for FiberCont<Sv, I, Y, R, F>
where
  Sv: Switch<DataCont<I, Y, R>>,
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

impl<Sv, F> FiberRoot for FiberCont<Sv, (), (), (), F>
where
  Sv: Switch<DataCont<(), (), ()>>,
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

impl<Sv, I, Y, R, F> Drop for FiberCont<Sv, I, Y, R, F>
where
  Sv: Switch<DataCont<I, Y, R>>,
  F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  fn drop(&mut self) {
    unsafe { Heap.dealloc(self.stack_top, layout(self.stack_size)) };
  }
}

unsafe impl<Sv, I, Y, R, F> Send for FiberCont<Sv, I, Y, R, F>
where
  Sv: Switch<DataCont<I, Y, R>>,
  F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
}

#[cfg_attr(feature = "clippy", allow(new_without_default_derive))]
impl<Sv, I, Y, R> Yielder<Sv, I, Y, R>
where
  Sv: Switch<DataCont<I, Y, R>>,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  /// Creates a new `Yielder`. Normally one should use the yielder provided to
  /// continuation as argument.
  ///
  /// # Safety
  ///
  /// `I` and `O` types must match the enclosing fiber.
  #[inline(always)]
  pub unsafe fn new() -> Self {
    Self {
      _sv: PhantomData,
      _input: PhantomData,
      _yield: PhantomData,
      _return: PhantomData,
    }
  }

  /// Yields from the enclosing stackful fiber.
  pub fn cont_yield(&self, output: Y) -> I {
    unsafe {
      let output = FiberState::Yielded(output);
      let mut data = Data { output };
      let mut data_ptr = &mut data as *mut _;
      Sv::switch_back(&mut data_ptr);
      forget(data);
      data_ptr.read().input
    }
  }
}

/// Creates a new stackful fiber.
pub fn new_cont<Sv, I, Y, R, F>(
  stack_size: usize,
  f: F,
) -> FiberCont<Sv, I, Y, R, F>
where
  Sv: Switch<DataCont<I, Y, R>>,
  F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  unsafe {
    let stack_top: *mut u8 = Heap
      .alloc(layout(stack_size))
      .unwrap_or_else(|err| Heap.oom(err));
    let stack_ptr = FiberCont::stack_init(stack_top.add(stack_size), f);
    FiberCont {
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

/// Spawns a new stackful fiber on the given `thr`.
pub fn spawn_cont<T, U, F>(thr: T, stack_size: usize, mut f: F)
where
  T: ThrToken<U>,
  U: ThrTag,
  F: FnMut(Yielder<<T::Thr as Thread>::Sv, (), (), ()>),
  F: Send + 'static,
  <T::Thr as Thread>::Sv: Switch<DataCont<(), (), ()>>,
{
  thr
    .as_ref()
    .fib_chain()
    .add(new_cont(stack_size, move |(), yielder| f(yielder)))
}

unsafe fn layout(stack_size: usize) -> Layout {
  Layout::from_size_align_unchecked(stack_size, 1)
}
