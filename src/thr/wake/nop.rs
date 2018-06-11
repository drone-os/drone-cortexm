use core::ptr;
use futures::task::{UnsafeWake, Waker};

#[derive(Clone)]
pub(in thr) struct WakeNop(());

impl WakeNop {
  #[inline(always)]
  pub(in thr) fn new() -> Self {
    WakeNop(())
  }

  #[inline(always)]
  pub(in thr) fn into_waker(self) -> Waker {
    unsafe { Waker::new(ptr::null::<WakeNop>() as *const UnsafeWake) }
  }
}

unsafe impl UnsafeWake for WakeNop {
  unsafe fn clone_raw(&self) -> Waker {
    WakeNop::new().into_waker()
  }

  unsafe fn drop_raw(&self) {}

  unsafe fn wake(&self) {}
}
