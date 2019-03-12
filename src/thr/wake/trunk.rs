use crate::cpu;
use core::{
  ptr,
  task::{RawWaker, RawWakerVTable, Waker},
};

static VTABLE: RawWakerVTable = RawWakerVTable { clone, wake, drop };

pub struct WakeTrunk(());

impl WakeTrunk {
  #[inline]
  pub fn new() -> Self {
    Self(())
  }

  #[inline]
  pub fn wait() {
    cpu::wait_for_event();
  }

  #[inline]
  pub fn to_waker(&self) -> Waker {
    unsafe { Waker::new_unchecked(raw_waker()) }
  }
}

fn raw_waker() -> RawWaker {
  RawWaker::new(ptr::null(), &VTABLE)
}

unsafe fn clone(_data: *const ()) -> RawWaker {
  raw_waker()
}

unsafe fn wake(_data: *const ()) {
  cpu::send_event();
}
