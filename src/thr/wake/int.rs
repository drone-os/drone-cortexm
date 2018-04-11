use core::ptr::write_volatile;
use futures::task::{UnsafeWake, Waker};

#[derive(Clone)]
pub(in thr) struct WakeInt(usize);

const NVIC_STIR: usize = 0xE000_EF00;

impl WakeInt {
  pub(in thr) fn new(int_num: usize) -> Self {
    WakeInt(int_num)
  }

  pub(in thr) fn waker(self) -> Waker {
    unsafe { Waker::new(&self) }
  }
}

unsafe impl UnsafeWake for WakeInt {
  #[inline(always)]
  unsafe fn clone_raw(&self) -> Waker {
    (*self).clone().waker()
  }

  #[inline(always)]
  unsafe fn drop_raw(&self) {}

  #[inline(always)]
  unsafe fn wake(&self) {
    write_volatile(NVIC_STIR as *mut u32, self.0 as u32);
  }
}
