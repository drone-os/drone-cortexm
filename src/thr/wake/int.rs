use core::{
  mem::align_of,
  ptr::{write_volatile, NonNull},
  task::{LocalWaker, UnsafeWake, Waker},
};

const NVIC_STIR: usize = 0xE000_EF00;

pub struct WakeInt(usize);

struct WakeIntWrapped;

impl WakeInt {
  #[inline(always)]
  pub fn new(int_num: usize) -> Self {
    Self(int_num + align_of::<WakeIntWrapped>())
  }

  #[inline(always)]
  pub fn wake(&self) {
    unsafe {
      write_volatile(
        NVIC_STIR as *mut usize,
        self.0 - align_of::<WakeIntWrapped>(),
      );
    }
  }

  #[inline]
  pub fn into_local_waker(self) -> LocalWaker {
    unsafe {
      LocalWaker::new(NonNull::new_unchecked(self.0 as *mut WakeIntWrapped))
    }
  }
}

unsafe impl UnsafeWake for WakeIntWrapped {
  #[inline]
  unsafe fn clone_raw(&self) -> Waker {
    WakeInt::new(self as *const _ as usize)
      .into_local_waker()
      .into_waker()
  }

  #[inline]
  unsafe fn drop_raw(&self) {}

  #[inline]
  unsafe fn wake(&self) {
    WakeInt::new(self as *const _ as usize).wake()
  }
}
