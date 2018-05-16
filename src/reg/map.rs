#![cfg_attr(feature = "cargo-clippy", allow(doc_markdown))]

use drone_core::reg::map;
use reg::prelude::*;

include!(concat!(env!("OUT_DIR"), "/svd_reg_map.rs"));

map! {
  /// Instrumentation trace macrocell.
  pub mod ITM;

  /// Trace Privilege Register.
  TPR {
    0xE000_0E40 0x20 0x0000_0000
    RReg WReg;
    /// Bit mask to enable unprivileged access to ITM stimulus ports.
    PRIVMASK { 0 32 RRRegField WWRegField }
  }

  /// Trace Control Register.
  TCR {
    0xE000_0E80 0x20 0x0000_0000
    RReg WReg;
    /// Indicates whether the ITM is currently processing events.
    BUSY { 23 1 RRRegField RoRRegField }
    /// Identifier for multi-source trace stream formatting.
    TraceBusID { 16 7 RRRegField WWRegField }
    /// Global timestamp frequency.
    GTSFREQ { 10 2 RRRegField WWRegField }
    /// Local timestamp prescaler, used with the trace packet reference clock.
    TSPrescale { 8 2 RRRegField WWRegField }
    /// Enables asynchronous clocking of the timestamp counter.
    SWOENA { 4 1 RRRegField WWRegField }
    /// Enables forwarding of hardware event packet from the DWT unit to the ITM
    /// for output to the TPIU.
    TXENA { 3 1 RRRegField WWRegField }
    /// Enables Synchronization packet transmission for a synchronous TPIU.
    SYNCENA { 2 1 RRRegField WWRegField }
    /// Enables Local timestamp generation.
    TSENA { 1 1 RRRegField WWRegField }
    /// Enables the ITM.
    ITMENA { 0 1 RRRegField WWRegField }
  }

  /// ITM lock access register.
  LAR {
    0xE000_0FB0 0x20 0x0000_0000
    WReg WoReg;
    /// Write `0xC5ACCE55` to unlock Write Access to the other ITM registers.
    UNLOCK { 0 32 WWRegField WoWRegField }
  }
}

map! {
  /// System control block.
  pub mod SCB;

  /// Interrupt control and state register.
  ICSR {
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

  /// Application interrupt and reset control register.
  AIRCR {
    0xE000_ED0C 0x20 0xFA05_0000
    RReg WReg;
    /// Register key. On writes, write `0x5FA` to `VECTKEY`, otherwise the write
    /// is ignored.
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

  /// System control register.
  SCR {
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

  /// Configuration and control register.
  CCR {
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

  /// System handler priority register 1.
  SHPR1 {
    0xE000_ED18 0x20 0x0000_0000
    RReg WReg;
    /// Priority of system handler 6, usage fault.
    PRI_USAGE_FAULT { 16 8 RRRegField WWRegField }
    /// Priority of system handler 5, bus fault.
    PRI_BUS_FAULT { 8 8 RRRegField WWRegField }
    /// Priority of system handler 4, memory management fault.
    PRI_MEM_MANAGE { 0 8 RRRegField WWRegField }
  }

  /// System handler priority register 2.
  SHPR2 {
    0xE000_ED1C 0x20 0x0000_0000
    RReg WReg;
    /// Priority of system handler 11, SVCall.
    PRI_SV_CALL { 24 8 RRRegField WWRegField }
  }

  /// System handler priority register 3.
  SHPR3 {
    0xE000_ED20 0x20 0x0000_0000
    RReg WReg;
    /// Priority of system handler 15, SysTick exception.
    PRI_SYS_TICK { 24 8 RRRegField WWRegField }
    /// Priority of system handler 14, PendSV.
    PRI_PEND_SV { 16 8 RRRegField WWRegField }
  }

  /// System handler control and state register.
  SHCSR {
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

  /// MemManage Status Register.
  MMFSR {
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

  /// BusFault Status Register.
  BFSR {
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

  /// UsageFault Status Register.
  UFSR {
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

  /// HardFault Status Register.
  HFSR {
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

  /// Debug Fault Status Register.
  DFSR {
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

  /// MemManage Fault Address Register.
  MMFAR {
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

  /// BusFault Address Register.
  BFAR {
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

  /// Floating Point Context Control Register.
  FPCCR {
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

  /// Floating Point Context Address Register.
  FPCAR {
    0xE000_EF38 0x20 0x0000_0000
    RReg;
    /// The location of the unpopulated floating-point register space allocated
    /// on an exception stack frame.
    ADDRESS { 3 29 RRRegField }
  }

  /// Floating Point Default Status Control Register.
  FPDSCR {
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

  /// Debug Exception and Monitor Control Register.
  DEMCR {
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
}

map! {
  /// SysTick timer.
  pub mod STK;

  /// SysTick control and status register.
  CTRL {
    0xE000_E010 0x20 0x0000_0000
    RReg WReg;
    /// Returns `true` if timer counted to `0` since last time this was read.
    COUNTFLAG { 16 1 RRRegField WWRegField }
    /// Clock source selection.
    CLKSOURCE { 2 1 RRRegField WWRegField }
    /// SysTick exception request enable.
    TICKINT { 1 1 RRRegField WWRegField }
    /// Counter enable.
    ENABLE { 0 1 RRRegField WWRegField }
  }

  /// SysTick reload value register.
  LOAD {
    0xE000_E014 0x20 0x0000_0000
    RReg WReg;
    /// RELOAD value.
    RELOAD { 0 24 RRRegField WWRegField }
  }

  /// SysTick current value register.
  VAL {
    0xE000_E018 0x20 0x0000_0000
    RReg WReg;
    /// Current counter value.
    CURRENT { 0 24 RRRegField WWRegField }
  }

  /// SysTick calibration value register.
  CALIB {
    0xE000_E01C 0x20 0x0000_0000
    RReg RoReg;
    /// NOREF flag.
    NOREF { 31 1 RRRegField RoRRegField }
    /// SKEW flag.
    SKEW { 30 1 RRRegField RoRRegField }
    /// Calibration value.
    TENMS { 0 24 RRRegField RoRRegField }
  }
}

map! {
  /// Memory protection unit.
  pub mod MPU;

  /// Indicates how many regions the MPU support.
  TYPE {
    0xE000_ED90 0x20 0x0000_0000
    RReg RoReg;
    /// Instruction region.
    IREGION { 16 8 RRRegField RoRRegField }
    /// Number of regions supported by the MPU.
    DREGION { 8 8 RRRegField RoRRegField }
    /// Indicates support for separate instruction and data address maps.
    SEPARATE { 0 1 RRRegField RoRRegField }
  }

  /// Enables the MPU, and when the MPU is enabled, controls whether the
  /// default memory map is enabled as a background region for privileged
  /// accesses, and whether the MPU is enabled for HardFaults, NMIs, and
  /// exception handlers when FAULTMASK is set to 1.
  CTRL {
    0xE000_ED94 0x20 0x0000_0000
    RReg WReg;
    /// Enable priviliged software access to default memory map.
    PRIVDEFENA { 2 1 RRRegField WWRegField }
    /// Enables the operation of MPU during hard fault, NMI, and FAULTMASK
    /// handlers.
    HFNMIENA { 1 1 RRRegField WWRegField }
    /// Enables the MPU.
    ENABLE { 0 1 RRRegField WWRegField }
  }

  /// Selects the region currently accessed by MPU_RBAR and MPU_RASR.
  RNR {
    0xE000_ED98 0x20 0x0000_0000
    RReg WReg;
    /// Indicates the memory region accessed by MPU_RBAR and MPU_RASR.
    REGION { 0 8 RRRegField WWRegField }
  }

  /// Holds the base address of the region identified by MPU_RNR. On a write,
  /// can also be used to update the base address of a specified region, in
  /// the range 0 to 15, updating MPU_RNR with the new region number.
  RBAR {
    0xE000_ED9C 0x20 0x0000_0000
    RReg WReg;
    /// Region base address field.
    ADDR { 5 27 RRRegField WWRegField }
    /// MPU region number valid.
    VALID { 4 1 RRRegField WWRegField }
    /// MPU region field.
    REGION { 0 4 RRRegField WWRegField }
  }

  /// Defines the size and access behavior of the region identified by
  /// MPU_RNR, and enables that region.
  RASR {
    0xE000_EDA0 0x20 0x0000_0000
    RReg WReg;
    /// Instruction access disable bit.
    XN { 28 1 RRRegField WWRegField }
    /// Access permission.
    AP { 24 3 RRRegField WWRegField }
    /// Memory attribute.
    TEX { 19 3 RRRegField WWRegField }
    /// Shareable memory attribute.
    S { 18 1 RRRegField WWRegField }
    /// Memory attribute.
    C { 17 1 RRRegField WWRegField }
    /// Memory attribute.
    B { 16 1 RRRegField WWRegField }
    /// Subregion disable bits.
    SRD { 8 8 RRRegField WWRegField }
    /// Size of the MPU protection region.
    SIZE { 1 5 RRRegField WWRegField }
    /// Region enable bit.
    ENABLE { 0 1 RRRegField WWRegField }
  }
}

map! {
  /// Trace port interface unit.
  pub mod TPIU;

  /// Selected Pin Protocol Register.
  SPPR {
    0xE004_00F0 0x20 0x0000_0001
    RReg WReg;
    /// Specified the protocol for trace output from the TPIU.
    TXMODE { 0 2 RRRegField WWRegField }
  }

  /// Formatter and Flush Control Register.
  FFCR {
    0xE004_0304 0x20 0x0000_0102
    RReg WReg;
    /// This bit Reads-As-One (RAO), specifying that triggers are inserted when
    /// a trigger pin is asserted.
    TrigIn { 8 1 RRRegField RoRRegField }
    /// Enable continuous formatting.
    EnFCont { 1 1 RRRegField WWRegField }
  }
}
