use core::marker::PhantomData;
use futures::task::{UnsafeWake, Waker};

#[derive(Clone)]
pub(in thr) struct WakeNop(PhantomData<()>);

impl WakeNop {
  pub(in thr) fn new() -> Self {
    WakeNop(PhantomData)
  }

  pub(in thr) fn waker(self) -> Waker {
    unsafe { Waker::new(&self) }
  }
}

unsafe impl UnsafeWake for WakeNop {
  #[inline(always)]
  unsafe fn clone_raw(&self) -> Waker {
    (*self).clone().waker()
  }

  #[inline(always)]
  unsafe fn drop_raw(&self) {}

  #[inline(always)]
  unsafe fn wake(&self) {}
}
