use core::ptr::write_volatile;
use futures::task::{UnsafeWake, Waker};

const NVIC_STIR: usize = 0xE000_EF00;

pub(in thr) struct WakeInt(usize);

struct WakeIntWrapped;

impl WakeInt {
  #[inline(always)]
  pub(in thr) fn new(int_num: usize) -> Self {
    WakeInt(int_num)
  }

  #[inline(always)]
  pub(in thr) fn wake(&self) {
    unsafe { write_volatile(NVIC_STIR as *mut u32, self.0 as u32) };
  }

  #[inline(always)]
  pub(in thr) fn into_waker(self) -> Waker {
    unsafe { Waker::new(self.0 as *const WakeIntWrapped as *const UnsafeWake) }
  }
}

unsafe impl UnsafeWake for WakeIntWrapped {
  unsafe fn clone_raw(&self) -> Waker {
    WakeInt::new(self as *const WakeIntWrapped as usize).into_waker()
  }

  unsafe fn drop_raw(&self) {}

  unsafe fn wake(&self) {
    WakeInt::new(self as *const WakeIntWrapped as usize).wake()
  }
}
