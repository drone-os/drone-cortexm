use drone_core::reg;

use crate::reg::prelude::*;

reg! {
    /// SysTick control and status register.
    pub STK CTRL => {
        address => 0xE000_E010;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Returns `true` if timer counted to `0` since last time this was read.
            COUNTFLAG => { offset => 16; width => 1; traits => { RRRegField WWRegField } };
            /// Clock source selection.
            CLKSOURCE => { offset => 2; width => 1; traits => { RRRegField WWRegField } };
            /// SysTick exception request enable.
            TICKINT => { offset => 1; width => 1; traits => { RRRegField WWRegField } };
            /// Counter enable.
            ENABLE => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// SysTick reload value register.
    pub STK LOAD => {
        address => 0xE000_E014;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// RELOAD value.
            RELOAD => { offset => 0; width => 24; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// SysTick current value register.
    pub STK VAL => {
        address => 0xE000_E018;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Current counter value.
            CURRENT => { offset => 0; width => 24; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// SysTick calibration value register.
    pub STK CALIB => {
        address => 0xE000_E01C;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg RoReg };
        fields => {
            /// NOREF flag.
            NOREF => { offset => 31; width => 1; traits => { RRRegField RoRRegField } };
            /// SKEW flag.
            SKEW => { offset => 30; width => 1; traits => { RRRegField RoRRegField } };
            /// Calibration value.
            TENMS => { offset => 0; width => 24; traits => { RRRegField RoRRegField } };
        };
    };
}
