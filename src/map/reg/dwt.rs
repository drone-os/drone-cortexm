use crate::reg::prelude::*;
use drone_core::reg;

reg! {
    /// Control Register.
    pub mod DWT CTRL;
    0xE000_1000 0x20 0x0000_0000
    RReg WReg;
    /// Number of comparators implemented.
    NUMCOMP { 28 4 RRRegField }
    /// Shows whether the implementation supports trace sampling and execution
    /// tracing.
    NOTRCPKT { 27 1 RRRegField }
    /// Shows whether the implementation includes external match signals.
    NOEXTTRIG { 26 1 RRRegField }
    /// Shows whether the implementation supports a cycle counter.
    NOCYCCNT { 25 1 RRRegField }
    /// Shows whether the implementation supports the profiling counters.
    NOPRFCNT { 24 1 RRRegField }
    /// Enables POSTCNT underflow Event counter packets generation.
    CYCEVTENA { 22 1 RRRegField WWRegField }
    /// Enables generation of the Folded-instruction counter overflow event.
    FOLDEVTENA { 21 1 RRRegField WWRegField }
    /// Enables generation of the LSU counter overflow event.
    LSUEVTENA { 20 1 RRRegField WWRegField }
    /// Enables generation of the Sleep counter overflow event.
    SLEEPEVTENA { 19 1 RRRegField WWRegField }
    /// Enables generation of the Exception overhead counter overflow event.
    EXCEVTENA { 18 1 RRRegField WWRegField }
    /// Enables generation of the CPI counter overflow event.
    CPIEVTENA { 17 1 RRRegField WWRegField }
    /// Enables generation of exception trace.
    EXCTRCENA { 16 1 RRRegField WWRegField }
    /// Enables use of POSTCNT counter as a timer for Periodic PC sample packet
    /// generation.
    PCSAMPLEENA { 12 1 RRRegField WWRegField }
    /// Selects the position of the synchronization packet counter tap on the
    /// CYCCNT counter.
    SYNCTAP { 10 2 RRRegField WWRegField }
    /// Selects the position of the POSTCNT tap on the CYCCNT counter.
    CYCTAP { 9 1 RRRegField WWRegField }
    /// Initial value for the POSTCNT counter.
    POSTINIT { 5 4 RRRegField WWRegField }
    /// Reload value for the POSTCNT counter.
    POSTPRESET { 1 4 RRRegField WWRegField }
    /// Enables CYCCNT.
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
