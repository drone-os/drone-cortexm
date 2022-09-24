use drone_core::reg;

use crate::reg::prelude::*;

reg! {
    /// Trace Privilege Register.
    pub ITM TPR => {
        address => 0xE000_0E40;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Bit mask to enable unprivileged access to ITM stimulus ports.
            PRIVMASK => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Trace Control Register.
    pub ITM TCR => {
        address => 0xE000_0E80;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Indicates whether the ITM is currently processing events.
            BUSY => { offset => 23; width => 1; traits => { RRRegField RoRRegField } };
            /// Identifier for multi-source trace stream formatting.
            TraceBusID => { offset => 16; width => 7; traits => { RRRegField WWRegField } };
            /// Global timestamp frequency.
            GTSFREQ => { offset => 10; width => 2; traits => { RRRegField WWRegField } };
            /// Local timestamp prescaler, used with the trace packet reference clock.
            TSPrescale => { offset => 8; width => 2; traits => { RRRegField WWRegField } };
            /// Enables asynchronous clocking of the timestamp counter.
            SWOENA => { offset => 4; width => 1; traits => { RRRegField WWRegField } };
            /// Enables forwarding of hardware event packet from the DWT unit to the ITM
            /// for output to the TPIU.
            TXENA => { offset => 3; width => 1; traits => { RRRegField WWRegField } };
            /// Enables Synchronization packet transmission for a synchronous TPIU.
            SYNCENA => { offset => 2; width => 1; traits => { RRRegField WWRegField } };
            /// Enables Local timestamp generation.
            TSENA => { offset => 1; width => 1; traits => { RRRegField WWRegField } };
            /// Enables the ITM.
            ITMENA => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// ITM lock access register.
    pub ITM LAR => {
        address => 0xE000_0FB0;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { WReg WoReg };
        fields => {
            /// Write `0xC5ACCE55` to unlock Write Access to the other ITM registers.
            UNLOCK => { offset => 0; width => 32; traits => { WWRegField WoWRegField } };
        };
    };
}
