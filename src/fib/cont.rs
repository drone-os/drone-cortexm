use alloc::heap::{Alloc, Heap, Layout};
use core::marker::PhantomData;
use core::mem;
use fib::{Fiber, FiberRoot, FiberState};
use thr::prelude::*;

/// A stackful fiber.
pub struct FiberCont<F, I, Y, R>
where
  F: FnMut(I, Yielder<Y, I>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  stack_top: *mut u8,
  stack_ptr: *const u32,
  stack_size: usize,
  _f: PhantomData<*const F>,
  _input: PhantomData<*const I>,
  _yield: PhantomData<*const Y>,
  _return: PhantomData<*const R>,
}

/// A communication channel for [`FiberCont`](FiberCont).
#[derive(Clone, Copy)]
pub struct Yielder<I, O>
where
  I: Send + 'static,
  O: Send + 'static,
{
  _input: PhantomData<*const I>,
  _output: PhantomData<*const O>,
}

impl<F, I, Y, R> FiberCont<F, I, Y, R>
where
  F: FnMut(I, Yielder<Y, I>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  unsafe fn stack_init(stack_ptr: *mut u8, f: F) -> *const u32 {
    let mut stack_ptr = stack_ptr as *mut F;
    stack_ptr = stack_ptr.sub(1);
    stack_ptr.write(f);
    let mut stack_ptr = stack_ptr as *mut u32;
    // xPSR
    stack_ptr = stack_ptr.sub(1);
    stack_ptr.write(0x0100_0000);
    // PC
    stack_ptr = stack_ptr.sub(1);
    stack_ptr.write(&Self::handler as *const _ as u32);
    // LR, R12, R3, R2, R1, R0, R11, R10, R9, R8, R7, R6, R5, R4
    stack_ptr = stack_ptr.sub(14);
    stack_ptr.write_bytes(0, 14);
    stack_ptr as *const u32
  }

  #[naked]
  unsafe extern "C" fn handler() {
    let f: *mut F;
    asm!("mov $0, sp" : "=r"(f));
    match f.as_mut() {
      None => mem::unreachable(),
      Some(f) => {
        let input = ::core::mem::uninitialized::<I>();
        let yielder = Yielder::new();
        let output = f(input, yielder);
        cont_return(output, true);
      }
    }
  }
}

impl<F, I, Y, R> Fiber for FiberCont<F, I, Y, R>
where
  F: FnMut(I, Yielder<Y, I>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  type Input = ();
  type Yield = Y;
  type Return = R;

  fn resume(&mut self, _input: ()) -> FiberState<Y, R> {
    unsafe {
      cont_resume();
      cont_continue()
    }
  }
}

impl<F> FiberRoot for FiberCont<F, (), (), ()>
where
  F: FnMut((), Yielder<(), ()>),
  F: Send + 'static,
{
  fn advance(&mut self) -> bool {
    match self.resume(()) {
      FiberState::Yielded(()) => true,
      FiberState::Complete(()) => false,
    }
  }
}

impl<F, I, Y, R> Drop for FiberCont<F, I, Y, R>
where
  F: FnMut(I, Yielder<Y, I>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  fn drop(&mut self) {
    unsafe { Heap.dealloc(self.stack_top, layout(self.stack_size)) };
  }
}

unsafe impl<F, I, Y, R> Send for FiberCont<F, I, Y, R>
where
  F: FnMut(I, Yielder<Y, I>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
}

#[cfg_attr(feature = "clippy", allow(new_without_default_derive))]
impl<I, O> Yielder<I, O>
where
  I: Send + 'static,
  O: Send + 'static,
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
      _input: PhantomData,
      _output: PhantomData,
    }
  }

  /// Yields from the enclosing stackful fiber.
  #[inline(always)]
  pub fn cont_yield(input: I) -> O {
    unsafe { cont_return(input, false) };
  }
}

/// Creates a new stackful fiber.
pub fn new_cont<F, I, Y, R>(stack_size: usize, f: F) -> FiberCont<F, I, Y, R>
where
  F: FnMut(I, Yielder<Y, I>) -> R,
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
      _input: PhantomData,
      _yield: PhantomData,
      _return: PhantomData,
    }
  }
}

/// Spawns a new stackful fiber on the given `thr`.
#[inline(always)]
pub fn spawn_cont<T, U, F>(thr: T, stack_size: usize, f: F)
where
  T: AsRef<U>,
  U: Thread,
  F: FnMut(Yielder<(), ()>),
  F: Send + 'static,
{
  thr
    .as_ref()
    .fib_chain()
    .add(new_cont(stack_size, |(), yielder| f(yielder)))
}

unsafe fn cont_resume() {
  asm!("
    push {r4-r11}
    svc 0
  " :
    :
    :
    : "volatile");
}

unsafe fn cont_return<T>(value: T, complete: bool)
where
  T: Send + 'static,
{
  // TODO
  asm!("svc 1" :::: "volatile");
}

unsafe fn cont_continue<Y, R>() -> FiberState<Y, R>
where
  Y: Send + 'static,
  R: Send + 'static,
{
  asm!("
    pop {r4-r11}
  " :
    :
    :
    : "volatile");
  // TODO
  FiberState::Complete(::core::mem::uninitialized::<R>())
}

unsafe fn layout(stack_size: usize) -> Layout {
  Layout::from_size_align_unchecked(stack_size, 1)
}
