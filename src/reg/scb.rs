//! System control block

use super::prelude::*;

reg! {
  [0xE000_ED10] u32
  #[doc = "System control register"]
  Scr
  #[doc = "System control register"]
  ScrValue
  RReg {} WReg {}
}

impl ScrValue {
  /// Send Event on Pending bit
  #[inline]
  pub fn seveonpend(&self) -> bool {
    self.bit(4)
  }

  /// Send Event on Pending bit
  #[inline]
  pub fn set_seveonpend(&mut self, value: bool) -> &mut Self {
    self.set_bit(4, value)
  }

  /// Controls whether the processor uses sleep or deep sleep as its low power
  /// mode
  #[inline]
  pub fn sleepdeep(&self) -> bool {
    self.bit(2)
  }

  /// Controls whether the processor uses sleep or deep sleep as its low power
  /// mode
  #[inline]
  pub fn set_sleepdeep(&mut self, value: bool) -> &mut Self {
    self.set_bit(2, value)
  }

  /// Configures sleep-on-exit when returning from Handler mode to Thread mode
  #[inline]
  pub fn sleeponexit(&self) -> bool {
    self.bit(1)
  }

  /// Configures sleep-on-exit when returning from Handler mode to Thread mode
  #[inline]
  pub fn set_sleeponexit(&mut self, value: bool) -> &mut Self {
    self.set_bit(1, value)
  }
}
