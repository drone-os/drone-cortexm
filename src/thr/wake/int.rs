use core::ptr::write_volatile;
use futures::task::{UnsafeWake, Waker};

const NVIC_STIR: usize = 0xE000_EF00;

pub(in crate::thr) struct WakeInt(usize);

struct WakeIntWrapped;

impl WakeInt {
  #[inline(always)]
  pub(in crate::thr) fn new(int_num: usize) -> Self {
    Self(int_num)
  }

  #[inline(always)]
  pub(in crate::thr) fn wake(&self) {
    unsafe { write_volatile(NVIC_STIR as *mut u32, self.0 as u32) };
  }

  #[inline]
  pub(in crate::thr) fn into_waker(self) -> Waker {
    unsafe { Waker::new(self.0 as *const WakeIntWrapped as *const UnsafeWake) }
  }
}

unsafe impl UnsafeWake for WakeIntWrapped {
  unsafe fn clone_raw(&self) -> Waker {
    WakeInt::new(self as *const Self as usize).into_waker()
  }

  unsafe fn drop_raw(&self) {}

  unsafe fn wake(&self) {
    WakeInt::new(self as *const Self as usize).wake()
  }
}
