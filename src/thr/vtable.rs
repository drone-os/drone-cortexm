//! The vector table support.

use core::marker::PhantomData;
use thr::prelude::*;

/// Pointer to an exception handler.
pub type Handler = unsafe extern "C" fn();

/// Pointer to a reset handler.
pub type ResetHandler = unsafe extern "C" fn() -> !;

/// Reserved pointer in a vector table.
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Reserved {
  /// The only allowed zero-value.
  Vector = 0,
}

/// Reset thread token.
#[derive(Clone, Copy)]
pub struct Reset<T: ThrTag, U>(PhantomData<T>, PhantomData<U>);

impl<T: ThrTag, U: Thread> Reset<T, &'static U> {
  #[doc(hidden)]
  #[inline(always)]
  pub unsafe fn new() -> Self {
    Reset(PhantomData, PhantomData)
  }
}

impl<T: ThrTag, U: Thread> ThrToken<T> for Reset<T, &'static U> {
  type Thr = U;

  const THR_NUM: usize = 0;
}

impl<T: ThrTag, U: Thread> AsRef<U> for Reset<T, &'static U> {
  #[inline(always)]
  fn as_ref(&self) -> &U {
    Self::get_thr()
  }
}

impl<U: Thread> From<Reset<Ctt, &'static U>> for Reset<Ttt, &'static U> {
  #[inline(always)]
  fn from(_token: Reset<Ctt, &'static U>) -> Self {
    unsafe { Self::new() }
  }
}

impl<U: Thread> From<Reset<Ctt, &'static U>> for Reset<Ltt, &'static U> {
  #[inline(always)]
  fn from(_token: Reset<Ctt, &'static U>) -> Self {
    unsafe { Self::new() }
  }
}

impl<U: Thread> From<Reset<Ttt, &'static U>> for Reset<Ltt, &'static U> {
  #[inline(always)]
  fn from(_token: Reset<Ttt, &'static U>) -> Self {
    unsafe { Self::new() }
  }
}
