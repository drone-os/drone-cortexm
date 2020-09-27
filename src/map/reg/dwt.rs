use crate::reg::prelude::*;
use drone_core::reg;

reg! {
    /// DWT Control Register to enable the DWT unit.
    pub mod DWT DWTCTRL;
    0xE000_1000 0x20 0x0000_0000
    RReg WReg;
    /// Enable the CYCCNT counter. 
    CYCCNTENA { 0 1 RRRegField WWRegField }
}

reg! {
    /// Cycle Count Register.
    pub mod DWT CYCCNT;
    0xE000_1004 0x20 0x0000_0000
    RReg WReg;
    /// Incrementing cycle counter value.
    CYCCNT { 0 32 RRRegField WWRegField }
}

reg! {
    /// Control Register
    pub mod DWT DEMCR;
    0xE000_EDFC 0x20 0x0000_0000
    RReg WReg;
    /// This enables control of power usage unless tracing is required. 
    TRCENA { 24 1 RRRegField WWRegField }
}
