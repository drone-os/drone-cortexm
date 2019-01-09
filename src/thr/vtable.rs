//! The vector table support.

use crate::thr::prelude::*;
use core::marker::PhantomData;

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
  type AThrToken = Reset<Att, &'static U>;
  type TThrToken = Reset<Ttt, &'static U>;
  type CThrToken = Reset<Ctt, &'static U>;
  type RThrToken = Reset<Rtt, &'static U>;

  const THR_NUM: usize = 0;

  #[inline(always)]
  unsafe fn take() -> Self {
    Self(PhantomData, PhantomData)
  }
}
