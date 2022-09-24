use drone_core::reg;

use crate::reg::prelude::*;

reg! {
    /// Coprocessor access control register.
    pub FPU CPACR => {
        address => 0xE000_ED88;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Access privileges for coprocessor 11.
            CP11 => { offset => 22; width => 2; traits => { RRRegField WWRegField } };
            /// Access privileges for coprocessor 10.
            CP10 => { offset => 20; width => 2; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Floating-point context control register.
    pub FPU FPCCR => {
        address => 0xE000_EF34;
        size => 0x20;
        reset => 0xC000_0000;
        traits => { RReg WReg };
        fields => {
            /// When this bit is set to 1, execution of a floating-point instruction
            /// sets the CONTROL.FPCA bit to 1.
            ASPEN => { offset => 31; width => 1; traits => { RRRegField WWRegField } };
            /// Enables lazy context save of FP state.
            LSPEN => { offset => 30; width => 1; traits => { RRRegField WWRegField } };
            /// Indicates whether the software executing when the processor allocated
            /// the FP stack frame was able to set the DebugMonitor exception to
            /// pending.
            MONRDY => { offset => 8; width => 1; traits => { RRRegField } };
            /// Indicates whether the software executing when the processor allocated
            /// the FP stack frame was able to set the BusFault exception to pending.
            BFRDY => { offset => 6; width => 1; traits => { RRRegField } };
            /// Indicates whether the software executing when the processor allocated
            /// the FP stack frame was able to set the MemManage exception to pending.
            MMRDY => { offset => 5; width => 1; traits => { RRRegField } };
            /// Indicates whether the software executing when the processor allocated
            /// the FP stack frame was able to set the HardFault exception to pending.
            HFRDY => { offset => 4; width => 1; traits => { RRRegField } };
            /// Indicates the processor mode when it allocated the FP stack frame.
            THREAD => { offset => 3; width => 1; traits => { RRRegField } };
            /// Indicates the privilege level of the software executing when the
            /// processor allocated the FP stack frame.
            USER => { offset => 1; width => 1; traits => { RRRegField } };
            /// Indicates whether Lazy preservation of the FP state is active.
            LSPACT => { offset => 0; width => 1; traits => { RRRegField } };
        };
    };
}

reg! {
    /// Floating-point context address register.
    pub FPU FPCAR => {
        address => 0xE000_EF38;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg };
        fields => {
            /// The location of the unpopulated floating-point register space allocated
            /// on an exception stack frame.
            ADDRESS => { offset => 3; width => 29; traits => { RRRegField } };
        };
    };
}

reg! {
    /// Floating-point default status control register.
    pub FPU FPDSCR => {
        address => 0xE000_EF3C;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Default value for FPSCR.AHP.
            AHP => { offset => 26; width => 1; traits => { RRRegField WWRegField } };
            /// Default value for FPSCR.DN.
            DN => { offset => 25; width => 1; traits => { RRRegField WWRegField } };
            /// Default value for FPSCR.FZ.
            FZ => { offset => 24; width => 1; traits => { RRRegField WWRegField } };
            /// Default value for FPSCR.RMode.
            RMode => { offset => 22; width => 2; traits => { RRRegField WWRegField } };
        };
    };
}
