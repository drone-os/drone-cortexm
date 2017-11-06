//! System control block

use drone::reg;
use reg::prelude::*;

reg! {
  //! System control register
  0xE000_ED10 0x20
  Scr
  RReg WReg
}

impl ScrVal {
  /// Send Event on Pending bit
  #[inline]
  pub fn seveonpend(self) -> bool {
    unsafe { self.bit(4) }
  }

  /// Send Event on Pending bit
  #[inline]
  pub fn set_seveonpend(self, value: bool) -> Self {
    unsafe { self.set_bit(4, value) }
  }

  /// Controls whether the processor uses sleep or deep sleep as its low power
  /// mode
  #[inline]
  pub fn sleepdeep(self) -> bool {
    unsafe { self.bit(2) }
  }

  /// Controls whether the processor uses sleep or deep sleep as its low power
  /// mode
  #[inline]
  pub fn set_sleepdeep(self, value: bool) -> Self {
    unsafe { self.set_bit(2, value) }
  }

  /// Configures sleep-on-exit when returning from Handler mode to Thread mode
  #[inline]
  pub fn sleeponexit(self) -> bool {
    unsafe { self.bit(1) }
  }

  /// Configures sleep-on-exit when returning from Handler mode to Thread mode
  #[inline]
  pub fn set_sleeponexit(self, value: bool) -> Self {
    unsafe { self.set_bit(1, value) }
  }
}
