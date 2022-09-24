use drone_core::reg;

use crate::reg::prelude::*;

reg! {
    /// Asynchronous Clock Prescaler Register.
    pub TPIU ACPR => {
        address => 0xE004_0010;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// SWO baud rate prescaler value.
            SWOSCALER => { offset => 0; width => 16; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Selected Pin Protocol Register.
    pub TPIU SPPR => {
        address => 0xE004_00F0;
        size => 0x20;
        reset => 0x0000_0001;
        traits => { RReg WReg };
        fields => {
            /// Specified the protocol for trace output from the TPIU.
            TXMODE => { offset => 0; width => 2; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Formatter and Flush Control Register.
    pub TPIU FFCR => {
        address => 0xE004_0304;
        size => 0x20;
        reset => 0x0000_0102;
        traits => { RReg WReg };
        fields => {
            /// This bit Reads-As-One (RAO), specifying that triggers are inserted when
            /// a trigger pin is asserted.
            TrigIn => { offset => 8; width => 1; traits => { RRRegField RoRRegField } };
            /// Enable continuous formatting.
            EnFCont => { offset => 1; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}
