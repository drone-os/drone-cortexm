use drone_core::reg::mappings;
use reg::prelude::*;

include!(concat!(env!("OUT_DIR"), "/svd_mappings.rs"));

mappings! {
  /// System control block.
  SCB;

  /// System control register.
  SCR {
    0xE000_ED10 0x20 0x0000_0000
    RReg WReg;
    /// Send Event on Pending bit.
    SEVEONPEND { 4 1 RRRegField WWRegField }
    /// Controls whether the processor uses sleep or deep sleep as its low power
    /// mode.
    SLEEPDEEP { 2 1 RRRegField WWRegField }
    /// Configures sleep-on-exit when returning from Handler mode to Thread
    /// mode.
    SLEEPONEXIT { 1 1 RRRegField WWRegField }
  }
}

mappings! {
  /// SysTick timer.
  STK;

  /// SysTick control and status register.
  CTRL {
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

  /// SysTick reload value register.
  LOAD {
    0xE000_E014 0x20 0x0000_0000
    RReg WReg;
    /// RELOAD value.
    RELOAD { 0 24 RRRegField WWRegField }
  }

  /// SysTick current value register.
  VAL {
    0xE000_E018 0x20 0x0000_0000
    RReg WReg;
    /// Current counter value.
    CURRENT { 0 24 RRRegField WWRegField }
  }

  /// SysTick calibration value register.
  CALIB {
    0xE000_E01C 0x20 0x0000_0000
    RReg RoReg;
    /// NOREF flag.
    NOREF { 31 1 RRRegField RoRRegField }
    /// SKEW flag.
    SKEW { 30 1 RRRegField RoRRegField }
    /// Calibration value.
    TENMS { 0 24 RRRegField RoRRegField }
  }
}
