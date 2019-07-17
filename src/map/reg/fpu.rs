use crate::reg::prelude::*;
use drone_core::reg;

reg! {
    /// Coprocessor access control register.
    pub mod FPU CPACR;
    0xE000_ED88 0x20 0x0000_0000
    RReg WReg;
    /// Access privileges for coprocessor 11.
    CP11 { 22 2 RRRegField WWRegField }
    /// Access privileges for coprocessor 10.
    CP10 { 20 2 RRRegField WWRegField }
}

reg! {
    /// Floating-point context control register.
    pub mod FPU FPCCR;
    0xE000_EF34 0x20 0xC000_0000
    RReg WReg;
    /// When this bit is set to 1, execution of a floating-point instruction
    /// sets the CONTROL.FPCA bit to 1.
    ASPEN { 31 1 RRRegField WWRegField }
    /// Enables lazy context save of FP state.
    LSPEN { 30 1 RRRegField WWRegField }
    /// Indicates whether the software executing when the processor allocated
    /// the FP stack frame was able to set the DebugMonitor exception to
    /// pending.
    MONRDY { 8 1 RRRegField }
    /// Indicates whether the software executing when the processor allocated
    /// the FP stack frame was able to set the BusFault exception to pending.
    BFRDY { 6 1 RRRegField }
    /// Indicates whether the software executing when the processor allocated
    /// the FP stack frame was able to set the MemManage exception to pending.
    MMRDY { 5 1 RRRegField }
    /// Indicates whether the software executing when the processor allocated
    /// the FP stack frame was able to set the HardFault exception to pending.
    HFRDY { 4 1 RRRegField }
    /// Indicates the processor mode when it allocated the FP stack frame.
    THREAD { 3 1 RRRegField }
    /// Indicates the privilege level of the software executing when the
    /// processor allocated the FP stack frame.
    USER { 1 1 RRRegField }
    /// Indicates whether Lazy preservation of the FP state is active.
    LSPACT { 0 1 RRRegField }
}

reg! {
    /// Floating-point context address register.
    pub mod FPU FPCAR;
    0xE000_EF38 0x20 0x0000_0000
    RReg;
    /// The location of the unpopulated floating-point register space allocated
    /// on an exception stack frame.
    ADDRESS { 3 29 RRRegField }
}

reg! {
    /// Floating-point default status control register.
    pub mod FPU FPDSCR;
    0xE000_EF3C 0x20 0x0000_0000
    RReg WReg;
    /// Default value for FPSCR.AHP.
    AHP { 26 1 RRRegField WWRegField }
    /// Default value for FPSCR.DN.
    DN { 25 1 RRRegField WWRegField }
    /// Default value for FPSCR.FZ.
    FZ { 24 1 RRRegField WWRegField }
    /// Default value for FPSCR.RMode.
    RMode { 22 2 RRRegField WWRegField }
}
