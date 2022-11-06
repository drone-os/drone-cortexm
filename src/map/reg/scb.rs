use crate::reg::prelude::*;
use drone_core::reg;

reg! {
    /// Provides identification information for the processor.
    pub SCB CPUID => {
        address => 0xE000_ED00;
        size => 0x20;
        reset => 0x410F_C241;
        traits => { RReg RoReg };
        fields => {
            /// Implementer code assigned by ARM.
            IMPLEMENTER => { offset => 24; width => 8; traits => { RRRegField RoRRegField } };
            /// Variant number.
            VARIANT => { offset => 20; width => 4; traits => { RRRegField RoRRegField } };
            /// Reads as `0xF`.
            ARCHITECTURE => { offset => 16; width => 4; traits => { RRRegField RoRRegField } };
            /// Part number of the processor.
            PARTNO => { offset => 4; width => 12; traits => { RRRegField RoRRegField } };
            /// Revision number.
            REVISION => { offset => 0; width => 4; traits => { RRRegField RoRRegField } };
        };
    };
}

reg! {
    /// Provides software control of the NMI, PendSV, and SysTick exceptions,
    /// and provides interrupt status information.
    pub SCB ICSR => {
        address => 0xE000_ED04;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg WoReg };
        fields => {
            /// NMI set-pending bit.
            NMIPENDSET => { offset => 31; width => 1; traits => { RRRegField WWRegField WoWRegField } };
            /// PendSV set-pending bit.
            PENDSVSET => { offset => 28; width => 1; traits => { RRRegField WWRegField WoWRegField } };
            /// PendSV clear-pending bit.
            PENDSVCLR => { offset => 27; width => 1; traits => { WWRegField WoWRegField } };
            /// SysTick exception set-pending bit.
            PENDSTSET => { offset => 26; width => 1; traits => { RRRegField WWRegField WoWRegField } };
            /// SysTick exception clear-pending bit.
            PENDSTCLR => { offset => 25; width => 1; traits => { WWRegField WoWRegField } };
            /// Interrupt pending flag, excluding NMI and Faults.
            ISRPENDING => { offset => 22; width => 1; traits => { RRRegField RoRRegField } };
            /// Pending vector. Indicates the exception number of the highest priority
            /// pending enabled exception.
            VECTPENDING => { offset => 12; width => 7; traits => { RRRegField RoRRegField } };
            /// Return to base level. Indicates whether there are preempted active
            /// exceptions.
            RETTOBASE => { offset => 11; width => 1; traits => { RRRegField RoRRegField } };
            /// Active vector. Contains the active exception number.
            VECTACTIVE => { offset => 0; width => 9; traits => { RRRegField RoRRegField } };
        };
    };
}

reg! {
    /// Holds the vector table address.
    pub SCB VTOR => {
        address => 0xE000_ED08;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Vector table base offset address.
            TBLOFF => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Application interrupt and reset control register.
    pub SCB AIRCR => {
        address => 0xE000_ED0C;
        size => 0x20;
        reset => 0xFA05_0000;
        traits => { RReg WReg };
        fields => {
            /// Vector Key.
            ///
            /// Register writes must write `0x05FA` to this field, otherwise the write
            /// is ignored.
            ///
            /// On reads, returns `0xFA05` .
            VECTKEY => { offset => 16; width => 16; traits => { RRRegField WWRegField } };
            /// Data endianness bit.
            ENDIANESS => { offset => 15; width => 1; traits => { RRRegField RoRRegField } };
            /// Interrupt priority grouping field.
            PRIGROUP => { offset => 8; width => 3; traits => { RRRegField WWRegField } };
            /// System reset request.
            SYSRESETREQ => { offset => 2; width => 1; traits => { WWRegField WoWRegField } };
            /// Clears all active state information for exceptions.
            VECTCLRACTIVE => { offset => 1; width => 1; traits => { WWRegField WoWRegField } };
            /// Resets the processor (except debug logic), but this will not reset
            /// circuits outside the processor.
            VECTRESET => { offset => 0; width => 1; traits => { WWRegField WoWRegField } };
        };
    };
}

reg! {
    /// System control register.
    pub SCB SCR => {
        address => 0xE000_ED10;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Send Event on Pending bit.
            SEVEONPEND => { offset => 4; width => 1; traits => { RRRegField WWRegField } };
            /// Controls whether the processor uses sleep or deep sleep as its low power
            /// mode.
            SLEEPDEEP => { offset => 2; width => 1; traits => { RRRegField WWRegField } };
            /// Configures sleep-on-exit when returning from Handler mode to Thread
            /// mode.
            SLEEPONEXIT => { offset => 1; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Configuration and control register.
    pub SCB CCR => {
        address => 0xE000_ED14;
        size => 0x20;
        reset => 0x0000_0200;
        traits => { RReg WReg };
        fields => {
            /// Force exception stacking start in double word aligned address.
            STKALIGN => { offset => 9; width => 1; traits => { RRRegField WWRegField } };
            /// Ignore data bus fault during HardFault and NMI handlers.
            BFHFNMIGN => { offset => 8; width => 1; traits => { RRRegField WWRegField } };
            /// Trap on divide by 0.
            DIV_0_TRP => { offset => 4; width => 1; traits => { RRRegField WWRegField } };
            /// Trap on unaligned accesses.
            UNALIGN_TRP => { offset => 3; width => 1; traits => { RRRegField WWRegField } };
            /// Enables unprivileged software access to Software Trigger Interrupt
            /// Register.
            USERSETMPEND => { offset => 1; width => 1; traits => { RRRegField WWRegField } };
            /// Non-base thread enable.
            NONBASETHRDENA => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// System handler priority register 1.
    pub SCB SHPR1 => {
        address => 0xE000_ED18;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Priority of system handler 6, usage fault.
            PRI_USAGE_FAULT => { offset => 16; width => 8; traits => { RRRegField WWRegField } };
            /// Priority of system handler 5, bus fault.
            PRI_BUS_FAULT => { offset => 8; width => 8; traits => { RRRegField WWRegField } };
            /// Priority of system handler 4, memory management fault.
            PRI_MEM_MANAGE => { offset => 0; width => 8; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// System handler priority register 2.
    pub SCB SHPR2 => {
        address => 0xE000_ED1C;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Priority of system handler 11, SVCall.
            PRI_SV_CALL => { offset => 24; width => 8; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// System handler priority register 3.
    pub SCB SHPR3 => {
        address => 0xE000_ED20;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Priority of system handler 15, SysTick exception.
            PRI_SYS_TICK => { offset => 24; width => 8; traits => { RRRegField WWRegField } };
            /// Priority of system handler 14, PendSV.
            PRI_PEND_SV => { offset => 16; width => 8; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// System handler control and state register.
    pub SCB SHCSR => {
        address => 0xE000_ED24;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Usage fault enable bit.
            USGFAULTENA => { offset => 18; width => 1; traits => { RRRegField WWRegField } };
            /// Bus fault enable bit.
            BUSFAULTENA => { offset => 17; width => 1; traits => { RRRegField WWRegField } };
            /// Memory management fault enable bit.
            MEMFAULTENA => { offset => 16; width => 1; traits => { RRRegField WWRegField } };
            /// SVC call pending bit.
            SVCALLPENDED => { offset => 15; width => 1; traits => { RRRegField WWRegField } };
            /// Bus fault exception pending bit.
            BUSFAULTPENDED => { offset => 14; width => 1; traits => { RRRegField WWRegField } };
            /// Memory management fault exception pending bit.
            MEMFAULTPENDED => { offset => 13; width => 1; traits => { RRRegField WWRegField } };
            /// Usage fault exception pending bit.
            USGFAULTPENDED => { offset => 12; width => 1; traits => { RRRegField WWRegField } };
            /// SysTick exception active bit.
            SYSTICKACT => { offset => 11; width => 1; traits => { RRRegField WWRegField } };
            /// PendSV exception active bit.
            PENDSVACT => { offset => 10; width => 1; traits => { RRRegField WWRegField } };
            /// Debug monitor active bit.
            MONITORACT => { offset => 8; width => 1; traits => { RRRegField WWRegField } };
            /// SVC call active bit.
            SVCALLACT => { offset => 7; width => 1; traits => { RRRegField WWRegField } };
            /// Usage fault exception active bit.
            USGFAULTACT => { offset => 3; width => 1; traits => { RRRegField WWRegField } };
            /// Bus fault exception active bit.
            BUSFAULTACT => { offset => 1; width => 1; traits => { RRRegField WWRegField } };
            /// Memory management fault exception active bit.
            MEMFAULTACT => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// MemManage Status Register.
    pub SCB MMFSR => {
        address => 0xE000_ED28;
        size => 0x8;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// MMFAR has valid contents.
            MMARVALID => { offset => 7; width => 1; traits => { RRRegField WWRegField } };
            /// A MemManage fault occurred during FP lazy state preservation.
            MLSPERR => { offset => 5; width => 1; traits => { RRRegField WWRegField } };
            /// A derived MemManage fault occurred on exception entry.
            MSTKERR => { offset => 4; width => 1; traits => { RRRegField WWRegField } };
            /// A derived MemManage fault occurred on exception return.
            MUNSTKERR => { offset => 3; width => 1; traits => { RRRegField WWRegField } };
            /// Data access violation. The MMFAR shows the data address that the load or
            /// store tried to access.
            DACCVIOL => { offset => 1; width => 1; traits => { RRRegField WWRegField } };
            /// MPU or Execute Never (XN) default memory map access violation on an
            /// instruction fetch has occurred. The fault is signalled only if the
            /// instruction is issued.
            IACCVIOL => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// BusFault Status Register.
    pub SCB BFSR => {
        address => 0xE000_ED29;
        size => 0x8;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// BFAR has valid contents.
            BFARVALID => { offset => 7; width => 1; traits => { RRRegField WWRegField } };
            /// A bus fault occurred during FP lazy state preservation.
            LSPERR => { offset => 5; width => 1; traits => { RRRegField WWRegField } };
            /// A derived bus fault has occurred on exception entry.
            STKERR => { offset => 4; width => 1; traits => { RRRegField WWRegField } };
            /// A derived bus fault has occurred on exception return.
            UNSTKERR => { offset => 3; width => 1; traits => { RRRegField WWRegField } };
            /// Imprecise data access error has occurred.
            IMPRECISERR => { offset => 2; width => 1; traits => { RRRegField WWRegField } };
            /// A precise data access error has occurred, and the processor has written
            /// the faulting address to the BFAR.
            PRECISERR => { offset => 1; width => 1; traits => { RRRegField WWRegField } };
            /// A bus fault on an instruction prefetch has occurred. The fault is
            /// signaled only if the instruction is issued.
            IBUSERR => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// UsageFault Status Register.
    pub SCB UFSR => {
        address => 0xE000_ED2A;
        size => 0x10;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Divide by zero error has occurred.
            DIVBYZERO => { offset => 9; width => 1; traits => { RRRegField WWRegField } };
            /// Unaligned access error has occurred.
            UNALIGNED => { offset => 8; width => 1; traits => { RRRegField WWRegField } };
            /// A coprocessor access error has occurred. This shows that the coprocessor
            /// is disabled or not present.
            NOCP => { offset => 3; width => 1; traits => { RRRegField WWRegField } };
            /// An integrity check error has occurred on EXC_RETURN.
            INVPC => { offset => 2; width => 1; traits => { RRRegField WWRegField } };
            /// Instruction executed with invalid EPSR.T or EPSR.IT field.
            INVSTATE => { offset => 1; width => 1; traits => { RRRegField WWRegField } };
            /// The processor has attempted to execute an undefined instruction.
            UNDEFINSTR => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// HardFault Status Register.
    pub SCB HFSR => {
        address => 0xE000_ED2C;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Debug event has occurred. The Debug Fault Status Register has been
            /// updated.
            DEBUGEVT => { offset => 31; width => 1; traits => { RRRegField WWRegField } };
            /// Indicates that a fault with configurable priority has been escalated to
            /// a HardFault exception, because it could not be made active, because of
            /// priority or because it was disabled.
            FORCED => { offset => 30; width => 1; traits => { RRRegField WWRegField } };
            /// Indicates when a fault has occurred because of a vector table read error
            /// on exception processing.
            VECTTBL => { offset => 1; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Debug Fault Status Register.
    pub SCB DFSR => {
        address => 0xE000_ED30;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Indicates a debug event generated because of the assertion of an
            /// external debug request.
            EXTERNAL => { offset => 4; width => 1; traits => { RRRegField WWRegField } };
            /// Indicates triggering of a Vector catch.
            VCATCH => { offset => 3; width => 1; traits => { RRRegField WWRegField } };
            /// Indicates a debug event generated by the DWT.
            DWTTRAP => { offset => 2; width => 1; traits => { RRRegField WWRegField } };
            /// Indicates a debug event generated by BKPT instruction execution or a
            /// breakpoint match in FPB.
            BKPT => { offset => 1; width => 1; traits => { RRRegField WWRegField } };
            /// Indicates a debug event generated by either.
            HALTED => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// MemManage Fault Address Register.
    pub SCB MMFAR => {
        address => 0xE000_ED34;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg };
        fields => {
            /// Data address for an MPU fault. This is the location addressed by an
            /// attempted load or store access that was faulted. The MemManage Status
            /// Register shows the cause of the fault, and whether MMFAR.ADDRESS is
            /// valid. When an unaligned access faults, the address is the actual
            /// address that faulted. Because an access might be split into multiple
            /// parts, each aligned, this address can be any offset in the range of the
            /// requested size.
            ADDRESS => { offset => 0; width => 32; traits => { RRRegField } };
        };
    };
}

reg! {
    /// BusFault Address Register.
    pub SCB BFAR => {
        address => 0xE000_ED38;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg };
        fields => {
            /// Data address for a precise bus fault. This is the location addressed by
            /// an attempted data access that was faulted. The BFSR shows the reason for
            /// the fault and whether BFAR.ADDRESS is valid.
            ///
            /// For unaligned access faults, the value returned is the address requested
            /// by the instruction. This might not be the address that faulted.
            ADDRESS => { offset => 0; width => 32; traits => { RRRegField } };
        };
    };
}

reg! {
    /// Auxiliary Fault Status Register.
    pub SCB AFSR => {
        address => 0xE000_ED3C;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Implementation defined.
            IMPDEF => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Debug Exception and Monitor Control Register.
    pub SCB DEMCR => {
        address => 0xE000_EDFC;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Global enable for all DWT and ITM features.
            TRCENA => { offset => 24; width => 1; traits => { RRRegField WWRegField } };
            /// DebugMonitor semaphore bit.
            MON_REQ => { offset => 19; width => 1; traits => { RRRegField WWRegField } };
            /// Setting this bit to 1 makes the step request pending.
            MON_STEP => { offset => 18; width => 1; traits => { RRRegField WWRegField } };
            /// Sets or clears the pending state of the DebugMonitor exception.
            MON_PEND => { offset => 17; width => 1; traits => { RRRegField WWRegField } };
            /// Enable the DebugMonitor exception.
            MON_EN => { offset => 16; width => 1; traits => { RRRegField WWRegField } };
            /// Enable halting debug trap on a HardFault exception.
            VC_HARDERR => { offset => 10; width => 1; traits => { RRRegField WWRegField } };
            /// Enable halting debug trap on a fault occurring during exception entry or
            /// exception return.
            VC_INTERR => { offset => 9; width => 1; traits => { RRRegField WWRegField } };
            /// Enable halting debug trap on a BusFault exception.
            VC_BUSERR => { offset => 8; width => 1; traits => { RRRegField WWRegField } };
            /// Enable halting debug trap on a UsageFault exception caused by a state
            /// information error, for example an Undefined Instruction exception.
            VC_STATERR => { offset => 7; width => 1; traits => { RRRegField WWRegField } };
            /// Enable halting debug trap on a UsageFault exception caused by a checking
            /// error, for example an alignment check error.
            VC_CHKERR => { offset => 6; width => 1; traits => { RRRegField WWRegField } };
            /// Enable halting debug trap on a UsageFault caused by an access to a
            /// Coprocessor.
            VC_NOCPERR => { offset => 5; width => 1; traits => { RRRegField WWRegField } };
            /// Enable halting debug trap on a MemManage exception.
            VC_MMERR => { offset => 4; width => 1; traits => { RRRegField WWRegField } };
            /// Enable Reset Vector Catch. This causes a Local reset to halt a running
            /// system.
            VC_CORERESET => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}
