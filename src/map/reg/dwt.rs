use drone_core::reg;

use crate::reg::prelude::*;

reg! {
    /// Control Register.
    pub DWT CTRL => {
        address => 0xE000_1000;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Number of comparators implemented.
            NUMCOMP => { offset => 28; width => 4; traits => { RRRegField } };
            /// Shows whether the implementation supports trace sampling and
            /// execution tracing.
            NOTRCPKT => { offset => 27; width => 1; traits => { RRRegField } };
            /// Shows whether the implementation includes external match
            /// signals.
            NOEXTTRIG => { offset => 26; width => 1; traits => { RRRegField } };
            /// Shows whether the implementation supports a cycle counter.
            NOCYCCNT => { offset => 25; width => 1; traits => { RRRegField } };
            /// Shows whether the implementation supports the profiling
            /// counters.
            NOPRFCNT => { offset => 24; width => 1; traits => { RRRegField } };
            /// Enables POSTCNT underflow Event counter packets generation.
            CYCEVTENA => { offset => 22; width => 1; traits => { RRRegField WWRegField } };
            /// Enables generation of the Folded-instruction counter overflow
            /// event.
            FOLDEVTENA => { offset => 21; width => 1; traits => { RRRegField WWRegField } };
            /// Enables generation of the LSU counter overflow event.
            LSUEVTENA => { offset => 20; width => 1; traits => { RRRegField WWRegField } };
            /// Enables generation of the Sleep counter overflow event.
            SLEEPEVTENA => { offset => 19; width => 1; traits => { RRRegField WWRegField } };
            /// Enables generation of the Exception overhead counter overflow
            /// event.
            EXCEVTENA => { offset => 18; width => 1; traits => { RRRegField WWRegField } };
            /// Enables generation of the CPI counter overflow event.
            CPIEVTENA => { offset => 17; width => 1; traits => { RRRegField WWRegField } };
            /// Enables generation of exception trace.
            EXCTRCENA => { offset => 16; width => 1; traits => { RRRegField WWRegField } };
            /// Enables use of POSTCNT counter as a timer for Periodic PC sample
            /// packet generation.
            PCSAMPLEENA => { offset => 12; width => 1; traits => { RRRegField WWRegField } };
            /// Selects the position of the synchronization packet counter tap
            /// on the CYCCNT counter.
            SYNCTAP => { offset => 10; width => 2; traits => { RRRegField WWRegField } };
            /// Selects the position of the POSTCNT tap on the CYCCNT counter.
            CYCTAP => { offset => 9; width => 1; traits => { RRRegField WWRegField } };
            /// Initial value for the POSTCNT counter.
            POSTINIT => { offset => 5; width => 4; traits => { RRRegField WWRegField } };
            /// Reload value for the POSTCNT counter.
            POSTPRESET => { offset => 1; width => 4; traits => { RRRegField WWRegField } };
            /// Enables CYCCNT.
            CYCCNTENA => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Cycle Count Register.
    pub DWT CYCCNT => {
        address => 0xE000_1004;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Incrementing cycle counter value.
            CYCCNT => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };
}
