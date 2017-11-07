//! SysTick timer

use drone::reg;
use reg::prelude::*;

reg! {
  //! SysTick control and status register
  0xE000_E010 0x20 0x0000_0000
  Ctrl
  RReg WReg
}

impl CtrlVal {
  /// Returns `true` if timer counted to `0` since last time this was read
  #[inline]
  pub fn countflag(self) -> bool {
    unsafe { self.bit(16) }
  }

  /// Clock source selection
  #[inline]
  pub fn clksource(self) -> bool {
    unsafe { self.bit(2) }
  }

  /// Clock source selection
  #[inline]
  pub fn set_clksource(self, value: bool) -> Self {
    unsafe { self.set_bit(2, value) }
  }

  /// SysTick exception request enable
  #[inline]
  pub fn tickint(self) -> bool {
    unsafe { self.bit(1) }
  }

  /// SysTick exception request enable
  #[inline]
  pub fn set_tickint(self, value: bool) -> Self {
    unsafe { self.set_bit(1, value) }
  }

  /// Counter enable
  #[inline]
  pub fn enable(self) -> bool {
    unsafe { self.bit(0) }
  }

  /// Counter enable
  #[inline]
  pub fn set_enable(self, value: bool) -> Self {
    unsafe { self.set_bit(0, value) }
  }
}

reg! {
  //! SysTick reload value register
  0xE000_E014 0x20 0x0000_0000
  Load
  RReg WReg
}

impl LoadVal {
  /// RELOAD value
  #[inline]
  pub fn reload(self) -> u32 {
    unsafe { self.bits(0, 24) }
  }

  /// RELOAD value
  #[inline]
  pub fn set_reload(self, value: u32) -> Self {
    unsafe { self.set_bits(0, 24, value) }
  }
}

reg! {
  //! SysTick current value register
  0xE000_E018 0x20 0x0000_0000
  Val
  RReg WReg
}

impl ValVal {
  /// Current counter value
  #[inline]
  pub fn current(self) -> u32 {
    unsafe { self.bits(0, 24) }
  }

  /// Current counter value
  #[inline]
  pub fn set_current(self, value: u32) -> Self {
    unsafe { self.set_bits(0, 24, value) }
  }
}

reg! {
  //! SysTick calibration value register
  0xE000_E01C 0x20 0x0000_0000
  Calib
  RReg
}

impl CalibVal {
  /// NOREF flag
  #[inline]
  pub fn noref(self) -> bool {
    unsafe { self.bit(31) }
  }

  /// SKEW flag
  #[inline]
  pub fn skew(self) -> bool {
    unsafe { self.bit(30) }
  }

  /// Calibration value
  #[inline]
  pub fn tenms(self) -> u32 {
    unsafe { self.bits(0, 24) }
  }
}
