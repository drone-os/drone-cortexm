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

impl<T: ThrTag, U: Thread> ThrToken<T> for Reset<T, &'static U> {
  type Thr = U;
  type UThrToken = Reset<Utt, &'static U>;
  type TThrToken = Reset<Ttt, &'static U>;
  type AThrToken = Reset<Att, &'static U>;

  const THR_NUM: usize = 0;

  #[inline(always)]
  unsafe fn new() -> Self {
    Reset(PhantomData, PhantomData)
  }
}

impl<T: ThrTag, U: Thread> AsRef<U> for Reset<T, &'static U> {
  #[inline(always)]
  fn as_ref(&self) -> &U {
    unsafe { Self::get_thr() }
  }
}
