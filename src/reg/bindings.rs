include!(concat!(env!("OUT_DIR"), "/svd.rs"));

pub use self::scb::Scr as ScbScr;

/// System control block
pub mod scb {
  use drone::reg;
  use reg::prelude::*;

  reg! {
    //! System control register
    0xE000_ED10 0x20 0x0000_0000
    SCR
    RReg WReg
    /// Send Event on Pending bit
    SEVEONPEND { 4 1 }
    /// Controls whether the processor uses sleep or deep sleep as its low power
    /// mode
    SLEEPDEEP { 2 1 }
    /// Configures sleep-on-exit when returning from Handler mode to Thread mode
    SLEEPONEXIT { 1 1 }
  }
}

pub use self::stk::Ctrl as StkCtrl;
pub use self::stk::Load as StkLoad;

/// SysTick timer
pub mod stk {
  use drone::reg;
  use reg::prelude::*;

  reg! {
    //! SysTick control and status register
    0xE000_E010 0x20 0x0000_0000
    CTRL
    RReg WReg
    /// Returns `true` if timer counted to `0` since last time this was read
    COUNTFLAG { 16 1 }
    /// Clock source selection
    CLKSOURCE { 2 1 }
    /// SysTick exception request enable
    TICKINT { 1 1 }
    /// Counter enable
    ENABLE { 0 1 }
  }

  reg! {
    //! SysTick reload value register
    0xE000_E014 0x20 0x0000_0000
    LOAD
    RReg WReg
    /// RELOAD value
    RELOAD { 0 24 }
  }

  reg! {
    //! SysTick current value register
    0xE000_E018 0x20 0x0000_0000
    VAL
    RReg WReg
    /// Current counter value
    CURRENT { 0 24 }
  }

  reg! {
    //! SysTick calibration value register
    0xE000_E01C 0x20 0x0000_0000
    CALIB
    RReg
    /// NOREF flag
    NOREF { 31 1 }
    /// SKEW flag
    SKEW { 30 1 }
    /// Calibration value
    TENMS { 0 24 }
  }
}
