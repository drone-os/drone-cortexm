use reg::prelude::*;

reg! {
  /// Provides identification information for the processor.
  pub mod SCB CPUID;
  0xE000_ED00 0x20 0x410F_C241
  RReg RoReg;
  /// Implementer code assigned by ARM.
  IMPLEMENTER { 24 8 RRRegField RoRRegField }
  /// Variant number.
  VARIANT { 20 4 RRRegField RoRRegField }
  /// Reads as `0xF`.
  ARCHITECTURE { 16 4 RRRegField RoRRegField }
  /// Part number of the processor.
  PARTNO { 4 12 RRRegField RoRRegField }
  /// Revision number.
  REVISION { 0 4 RRRegField RoRRegField }
}

reg! {
  /// Provides software control of the NMI, PendSV, and SysTick exceptions,
  /// and provides interrupt status information.
  pub mod SCB ICSR;
  0xE000_ED04 0x20 0x0000_0000
  RReg WReg WoReg;
  /// NMI set-pending bit.
  NMIPENDSET { 31 1 RRRegField WWRegField WoWRegField }
  /// PendSV set-pending bit.
  PENDSVSET { 28 1 RRRegField WWRegField WoWRegField }
  /// PendSV clear-pending bit.
  PENDSVCLR { 27 1 WWRegField WoWRegField }
  /// SysTick exception set-pending bit.
  PENDSTSET { 26 1 RRRegField WWRegField WoWRegField }
  /// SysTick exception clear-pending bit.
  PENDSTCLR { 25 1 WWRegField WoWRegField }
  /// Interrupt pending flag, excluding NMI and Faults.
  ISRPENDING { 22 1 RRRegField RoRRegField }
  /// Pending vector. Indicates the exception number of the highest priority
  /// pending enabled exception.
  VECTPENDING { 12 7 RRRegField RoRRegField }
  /// Return to base level. Indicates whether there are preempted active
  /// exceptions.
  RETTOBASE { 11 1 RRRegField RoRRegField }
  /// Active vector. Contains the active exception number.
  VECTACTIVE { 0 9 RRRegField RoRRegField }
}

reg! {
  /// Holds the vector table address.
  pub mod SCB VTOR;
  0xE000_ED08 0x20 0x0000_0000
  RReg WReg;
  /// Vector table base offset field.
  TBLOFF { 9 21 RRRegField WWRegField }
}

reg! {
  /// Application interrupt and reset control register.
  pub mod SCB AIRCR;
  0xE000_ED0C 0x20 0xFA05_0000
  RReg WReg;
  /// Vector Key.
  ///
  /// Register writes must write `0x05FA` to this field, otherwise the write
  /// is ignored.
  ///
  /// On reads, returns `0xFA05` .
  VECTKEY { 16 16 RRRegField WWRegField }
  /// Data endianness bit.
  ENDIANESS { 15 1 RRRegField RoRRegField }
  /// Interrupt priority grouping field.
  PRIGROUP { 8 3 RRRegField WWRegField }
  /// System reset request.
  SYSRESETREQ { 2 1 WWRegField WoWRegField }
  /// Clears all active state information for exceptions.
  VECTCLRACTIVE { 1 1 WWRegField WoWRegField }
  /// Resets the processor (except debug logic), but this will not reset
  /// circuits outside the processor.
  VECTRESET { 0 1 WWRegField WoWRegField }
}

reg! {
  /// System control register.
  pub mod SCB SCR;
  0xE000_ED10 0x20 0x0000_0000
  RReg WReg;
  /// Send Event on Pending bit.
  SEVEONPEND { 4 1 RRRegField WWRegField }
  /// Controls whether the processor uses sleep or deep sleep as its low power
  /// mode.
  SLEEPDEEP { 2 1 RRRegField WWRegField }
  /// Configures sleep-on-exit when returning from Handler mode to Thread
  /// mode.
  SLEEPONEXIT { 1 1 RRRegField WWRegField }
}

reg! {
  /// Configuration and control register.
  pub mod SCB CCR;
  0xE000_ED14 0x20 0x0000_0200
  RReg WReg;
  /// Force exception stacking start in double word aligned address.
  STKALIGN { 9 1 RRRegField WWRegField }
  /// Ignore data bus fault during HardFault and NMI handlers.
  BFHFNMIGN { 8 1 RRRegField WWRegField }
  /// Trap on divide by 0.
  DIV_0_TRP { 4 1 RRRegField WWRegField }
  /// Trap on unaligned accesses.
  UNALIGN_TRP { 3 1 RRRegField WWRegField }
  /// Enables unprivileged software access to Software Trigger Interrupt
  /// Register.
  USERSETMPEND { 1 1 RRRegField WWRegField }
  /// Non-base thread enable.
  NONBASETHRDENA { 0 1 RRRegField WWRegField }
}

reg! {
  /// System handler priority register 1.
  pub mod SCB SHPR1;
  0xE000_ED18 0x20 0x0000_0000
  RReg WReg;
  /// Priority of system handler 6, usage fault.
  PRI_USAGE_FAULT { 16 8 RRRegField WWRegField }
  /// Priority of system handler 5, bus fault.
  PRI_BUS_FAULT { 8 8 RRRegField WWRegField }
  /// Priority of system handler 4, memory management fault.
  PRI_MEM_MANAGE { 0 8 RRRegField WWRegField }
}

reg! {
  /// System handler priority register 2.
  pub mod SCB SHPR2;
  0xE000_ED1C 0x20 0x0000_0000
  RReg WReg;
  /// Priority of system handler 11, SVCall.
  PRI_SV_CALL { 24 8 RRRegField WWRegField }
}

reg! {
  /// System handler priority register 3.
  pub mod SCB SHPR3;
  0xE000_ED20 0x20 0x0000_0000
  RReg WReg;
  /// Priority of system handler 15, SysTick exception.
  PRI_SYS_TICK { 24 8 RRRegField WWRegField }
  /// Priority of system handler 14, PendSV.
  PRI_PEND_SV { 16 8 RRRegField WWRegField }
}

reg! {
  /// System handler control and state register.
  pub mod SCB SHCSR;
  0xE000_ED24 0x20 0x0000_0000
  RReg WReg;
  /// Usage fault enable bit.
  USGFAULTENA { 18 1 RRRegField WWRegField }
  /// Bus fault enable bit.
  BUSFAULTENA { 17 1 RRRegField WWRegField }
  /// Memory management fault enable bit.
  MEMFAULTENA { 16 1 RRRegField WWRegField }
  /// SVC call pending bit.
  SVCALLPENDED { 15 1 RRRegField WWRegField }
  /// Bus fault exception pending bit.
  BUSFAULTPENDED { 14 1 RRRegField WWRegField }
  /// Memory management fault exception pending bit.
  MEMFAULTPENDED { 13 1 RRRegField WWRegField }
  /// Usage fault exception pending bit.
  USGFAULTPENDED { 12 1 RRRegField WWRegField }
  /// SysTick exception active bit.
  SYSTICKACT { 11 1 RRRegField WWRegField }
  /// PendSV exception active bit.
  PENDSVACT { 10 1 RRRegField WWRegField }
  /// Debug monitor active bit.
  MONITORACT { 8 1 RRRegField WWRegField }
  /// SVC call active bit.
  SVCALLACT { 7 1 RRRegField WWRegField }
  /// Usage fault exception active bit.
  USGFAULTACT { 3 1 RRRegField WWRegField }
  /// Bus fault exception active bit.
  BUSFAULTACT { 1 1 RRRegField WWRegField }
  /// Memory management fault exception active bit.
  MEMFAULTACT { 0 1 RRRegField WWRegField }
}

reg! {
  /// MemManage Status Register.
  pub mod SCB MMFSR;
  0xE000_ED28 0x8 0x0000_0000
  RReg WReg;
  /// MMFAR has valid contents.
  MMARVALID { 7 1 RRRegField WWRegField }
  /// A MemManage fault occurred during FP lazy state preservation.
  MLSPERR { 5 1 RRRegField WWRegField }
  /// A derived MemManage fault occurred on exception entry.
  MSTKERR { 4 1 RRRegField WWRegField }
  /// A derived MemManage fault occurred on exception return.
  MUNSTKERR { 3 1 RRRegField WWRegField }
  /// Data access violation. The MMFAR shows the data address that the load or
  /// store tried to access.
  DACCVIOL { 1 1 RRRegField WWRegField }
  /// MPU or Execute Never (XN) default memory map access violation on an
  /// instruction fetch has occurred. The fault is signalled only if the
  /// instruction is issued.
  IACCVIOL { 0 1 RRRegField WWRegField }
}

reg! {
  /// BusFault Status Register.
  pub mod SCB BFSR;
  0xE000_ED29 0x8 0x0000_0000
  RReg WReg;
  /// BFAR has valid contents.
  BFARVALID { 7 1 RRRegField WWRegField }
  /// A bus fault occurred during FP lazy state preservation.
  LSPERR { 5 1 RRRegField WWRegField }
  /// A derived bus fault has occurred on exception entry.
  STKERR { 4 1 RRRegField WWRegField }
  /// A derived bus fault has occurred on exception return.
  UNSTKERR { 3 1 RRRegField WWRegField }
  /// Imprecise data access error has occurred.
  IMPRECISERR { 2 1 RRRegField WWRegField }
  /// A precise data access error has occurred, and the processor has written
  /// the faulting address to the BFAR.
  PRECISERR { 1 1 RRRegField WWRegField }
  /// A bus fault on an instruction prefetch has occurred. The fault is
  /// signaled only if the instruction is issued.
  IBUSERR { 0 1 RRRegField WWRegField }
}

reg! {
  /// UsageFault Status Register.
  pub mod SCB UFSR;
  0xE000_ED2A 0x10 0x0000_0000
  RReg WReg;
  /// Divide by zero error has occurred.
  DIVBYZERO { 9 1 RRRegField WWRegField }
  /// Unaligned access error has occurred.
  UNALIGNED { 8 1 RRRegField WWRegField }
  /// A coprocessor access error has occurred. This shows that the coprocessor
  /// is disabled or not present.
  NOCP { 3 1 RRRegField WWRegField }
  /// An integrity check error has occurred on EXC_RETURN.
  INVPC { 2 1 RRRegField WWRegField }
  /// Instruction executed with invalid EPSR.T or EPSR.IT field.
  INVSTATE { 1 1 RRRegField WWRegField }
  /// The processor has attempted to execute an undefined instruction.
  UNDEFINSTR { 0 1 RRRegField WWRegField }
}

reg! {
  /// HardFault Status Register.
  pub mod SCB HFSR;
  0xE000_ED2C 0x20 0x0000_0000
  RReg WReg;
  /// Debug event has occurred. The Debug Fault Status Register has been
  /// updated.
  DEBUGEVT { 31 1 RRRegField WWRegField }
  /// Indicates that a fault with configurable priority has been escalated to
  /// a HardFault exception, because it could not be made active, because of
  /// priority or because it was disabled.
  FORCED { 30 1 RRRegField WWRegField }
  /// Indicates when a fault has occurred because of a vector table read error
  /// on exception processing.
  VECTTBL { 1 1 RRRegField WWRegField }
}

reg! {
  /// Debug Fault Status Register.
  pub mod SCB DFSR;
  0xE000_ED30 0x20 0x0000_0000
  RReg WReg;
  /// Indicates a debug event generated because of the assertion of an
  /// external debug request.
  EXTERNAL { 4 1 RRRegField WWRegField }
  /// Indicates triggering of a Vector catch.
  VCATCH { 3 1 RRRegField WWRegField }
  /// Indicates a debug event generated by the DWT.
  DWTTRAP { 2 1 RRRegField WWRegField }
  /// Indicates a debug event generated by BKPT instruction execution or a
  /// breakpoint match in FPB.
  BKPT { 1 1 RRRegField WWRegField }
  /// Indicates a debug event generated by either.
  HALTED { 0 1 RRRegField WWRegField }
}

reg! {
  /// MemManage Fault Address Register.
  pub mod SCB MMFAR;
  0xE000_ED34 0x20 0x0000_0000
  RReg;
  /// Data address for an MPU fault. This is the location addressed by an
  /// attempted load or store access that was faulted. The MemManage Status
  /// Register shows the cause of the fault, and whether MMFAR.ADDRESS is
  /// valid. When an unaligned access faults, the address is the actual
  /// address that faulted. Because an access might be split into multiple
  /// parts, each aligned, this address can be any offset in the range of the
  /// requested size.
  ADDRESS { 0 32 RRRegField }
}

reg! {
  /// BusFault Address Register.
  pub mod SCB BFAR;
  0xE000_ED38 0x20 0x0000_0000
  RReg;
  /// Data address for a precise bus fault. This is the location addressed by
  /// an attempted data access that was faulted. The BFSR shows the reason for
  /// the fault and whether BFAR.ADDRESS is valid.
  ///
  /// For unaligned access faults, the value returned is the address requested
  /// by the instruction. This might not be the address that faulted.
  ADDRESS { 0 32 RRRegField }
}

reg! {
  /// Auxiliary Fault Status Register.
  pub mod SCB AFSR;
  0xE000_ED3C 0x20 0x0000_0000
  RReg WReg;
  /// Implementation defined.
  IMPDEF { 0 32 RRRegField WWRegField }
}

reg! {
  /// Debug Exception and Monitor Control Register.
  pub mod SCB DEMCR;
  0xE000_EDFC 0x20 0x0000_0000
  RReg WReg;
  /// Global enable for all DWT and ITM features.
  TRCENA { 24 1 RRRegField WWRegField }
  /// DebugMonitor semaphore bit.
  MON_REQ { 19 1 RRRegField WWRegField }
  /// Setting this bit to 1 makes the step request pending.
  MON_STEP { 18 1 RRRegField WWRegField }
  /// Sets or clears the pending state of the DebugMonitor exception.
  MON_PEND { 17 1 RRRegField WWRegField }
  /// Enable the DebugMonitor exception.
  MON_EN { 16 1 RRRegField WWRegField }
  /// Enable halting debug trap on a HardFault exception.
  VC_HARDERR { 10 1 RRRegField WWRegField }
  /// Enable halting debug trap on a fault occurring during exception entry or
  /// exception return.
  VC_INTERR { 9 1 RRRegField WWRegField }
  /// Enable halting debug trap on a BusFault exception.
  VC_BUSERR { 8 1 RRRegField WWRegField }
  /// Enable halting debug trap on a UsageFault exception caused by a state
  /// information error, for example an Undefined Instruction exception.
  VC_STATERR { 7 1 RRRegField WWRegField }
  /// Enable halting debug trap on a UsageFault exception caused by a checking
  /// error, for example an alignment check error.
  VC_CHKERR { 6 1 RRRegField WWRegField }
  /// Enable halting debug trap on a UsageFault caused by an access to a
  /// Coprocessor.
  VC_NOCPERR { 5 1 RRRegField WWRegField }
  /// Enable halting debug trap on a MemManage exception.
  VC_MMERR { 4 1 RRRegField WWRegField }
  /// Enable Reset Vector Catch. This causes a Local reset to halt a running
  /// system.
  VC_CORERESET { 0 1 RRRegField WWRegField }
}
