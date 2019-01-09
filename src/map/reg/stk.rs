use crate::reg::prelude::*;
use drone_core::reg;

reg! {
  /// SysTick control and status register.
  pub mod STK CTRL;
  0xE000_E010 0x20 0x0000_0000
  RReg WReg;
  /// Returns `true` if timer counted to `0` since last time this was read.
  COUNTFLAG { 16 1 RRRegField WWRegField }
  /// Clock source selection.
  CLKSOURCE { 2 1 RRRegField WWRegField }
  /// SysTick exception request enable.
  TICKINT { 1 1 RRRegField WWRegField }
  /// Counter enable.
  ENABLE { 0 1 RRRegField WWRegField }
}

reg! {
  /// SysTick reload value register.
  pub mod STK LOAD;
  0xE000_E014 0x20 0x0000_0000
  RReg WReg;
  /// RELOAD value.
  RELOAD { 0 24 RRRegField WWRegField }
}

reg! {
  /// SysTick current value register.
  pub mod STK VAL;
  0xE000_E018 0x20 0x0000_0000
  RReg WReg;
  /// Current counter value.
  CURRENT { 0 24 RRRegField WWRegField }
}

reg! {
  /// SysTick calibration value register.
  pub mod STK CALIB;
  0xE000_E01C 0x20 0x0000_0000
  RReg RoReg;
  /// NOREF flag.
  NOREF { 31 1 RRRegField RoRRegField }
  /// SKEW flag.
  SKEW { 30 1 RRRegField RoRRegField }
  /// Calibration value.
  TENMS { 0 24 RRRegField RoRRegField }
}
