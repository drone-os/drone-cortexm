use crate::cpu;
use core::ptr;
use futures::task::{UnsafeWake, Waker};

#[derive(Clone)]
pub(in crate::thr) struct WakeTrunk(());

impl WakeTrunk {
  #[inline(always)]
  pub(in crate::thr) fn new() -> Self {
    Self(())
  }

  #[inline(always)]
  pub(in crate::thr) fn wait() {
    cpu::wait_for_event();
  }

  #[inline]
  pub(in crate::thr) fn into_waker(self) -> Waker {
    unsafe { Waker::new(ptr::null::<Self>() as *const UnsafeWake) }
  }
}

unsafe impl UnsafeWake for WakeTrunk {
  unsafe fn clone_raw(&self) -> Waker {
    Self::new().into_waker()
  }

  unsafe fn drop_raw(&self) {}

  unsafe fn wake(&self) {
    cpu::send_event();
  }
}
