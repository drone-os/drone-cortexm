use crate::cpu;
use core::{
  ptr::NonNull,
  task::{LocalWaker, UnsafeWake, Waker},
};

pub struct WakeTrunk(());

impl WakeTrunk {
  #[inline(always)]
  pub fn new() -> Self {
    Self(())
  }

  #[inline(always)]
  pub fn wait() {
    cpu::wait_for_event();
  }

  #[inline]
  pub fn into_local_waker(self) -> LocalWaker {
    unsafe { LocalWaker::new(NonNull::<Self>::dangling()) }
  }
}

unsafe impl UnsafeWake for WakeTrunk {
  #[inline]
  unsafe fn clone_raw(&self) -> Waker {
    WakeTrunk::new().into_local_waker().into_waker()
  }

  #[inline]
  unsafe fn drop_raw(&self) {}

  #[inline]
  unsafe fn wake(&self) {
    cpu::send_event();
  }
}
