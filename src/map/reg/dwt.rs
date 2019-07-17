use crate::reg::prelude::*;
use drone_core::reg;

reg! {
    /// Cycle Count Register.
    pub mod DWT CYCCNT;
    0xE000_1004 0x20 0x0000_0000
    RReg WReg;
    /// Incrementing cycle counter value.
    CYCCNT { 0 32 RRRegField WWRegField }
}
