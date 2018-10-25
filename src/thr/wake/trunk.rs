use core::ptr;
use cpu;
use futures::task::{UnsafeWake, Waker};

#[derive(Clone)]
pub(in thr) struct WakeTrunk(());

impl WakeTrunk {
  #[inline(always)]
  pub(in thr) fn new() -> Self {
    WakeTrunk(())
  }

  #[inline(always)]
  pub(in thr) fn wait() {
    cpu::wait_for_event();
  }

  #[inline(always)]
  pub(in thr) fn into_waker(self) -> Waker {
    unsafe { Waker::new(ptr::null::<WakeTrunk>() as *const UnsafeWake) }
  }
}

unsafe impl UnsafeWake for WakeTrunk {
  unsafe fn clone_raw(&self) -> Waker {
    WakeTrunk::new().into_waker()
  }

  unsafe fn drop_raw(&self) {}

  unsafe fn wake(&self) {
    cpu::send_event();
  }
}
