use core::{
  mem::align_of,
  ptr::{write_volatile, NonNull},
  task::{LocalWaker, UnsafeWake, Waker},
};

const NVIC_STIR: usize = 0xE000_EF00;

pub struct WakeInt(usize);

struct WakeIntWrapped;

impl WakeInt {
  #[inline]
  pub fn new(int_num: usize) -> Self {
    Self(int_num)
  }

  #[inline]
  fn from_wrapped(wrapped: *const WakeIntWrapped) -> WakeInt {
    Self(wrapped as usize - align_of::<WakeIntWrapped>())
  }

  #[inline]
  fn into_wrapped(self) -> *mut WakeIntWrapped {
    (self.0 + align_of::<WakeIntWrapped>()) as _
  }

  #[inline]
  pub fn wake(&self) {
    unsafe { write_volatile(NVIC_STIR as *mut usize, self.0) };
  }

  #[inline]
  pub fn into_local_waker(self) -> LocalWaker {
    unsafe { LocalWaker::new(NonNull::new_unchecked(self.into_wrapped())) }
  }
}

unsafe impl UnsafeWake for WakeIntWrapped {
  #[inline]
  unsafe fn clone_raw(&self) -> Waker {
    Waker::new(NonNull::new_unchecked(self as *const Self as *mut Self))
  }

  #[inline]
  unsafe fn drop_raw(&self) {}

  #[inline]
  unsafe fn wake(&self) {
    WakeInt::from_wrapped(self).wake()
  }
}
