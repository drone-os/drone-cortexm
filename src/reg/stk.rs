//! SysTick timer

use super::prelude::*;

reg! {
  [0xE000_E010]
  #[doc = "SysTick control and status register"]
  Ctrl
  #[doc = "SysTick control and status register"]
  CtrlValue
  RReg {} WReg {}
}

impl CtrlValue {
  /// Returns `true` if timer counted to `0` since last time this was read
  pub fn countflag(&self) -> bool {
    self.bit(16)
  }

  /// Clock source selection
  pub fn clksource(&self) -> bool {
    self.bit(2)
  }

  /// Clock source selection
  pub fn set_clksource(&mut self, value: bool) -> &mut Self {
    self.set_bit(2, value)
  }

  /// SysTick exception request enable
  pub fn tickint(&self) -> bool {
    self.bit(1)
  }

  /// SysTick exception request enable
  pub fn set_tickint(&mut self, value: bool) -> &mut Self {
    self.set_bit(1, value)
  }

  /// Counter enable
  pub fn enable(&self) -> bool {
    self.bit(0)
  }

  /// Counter enable
  pub fn set_enable(&mut self, value: bool) -> &mut Self {
    self.set_bit(0, value)
  }
}

reg! {
  [0xE000_E014]
  #[doc = "SysTick reload value register"]
  Load
  #[doc = "SysTick reload value register"]
  LoadValue
  RReg {} WReg {}
}

impl LoadValue {
  /// RELOAD value
  pub fn reload(&self) -> usize {
    self.bits(0, 24)
  }

  /// RELOAD value
  pub fn set_reload(&mut self, value: usize) -> &mut Self {
    self.set_bits(0, 24, value)
  }
}
