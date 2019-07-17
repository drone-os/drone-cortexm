use crate::reg::prelude::*;
use drone_core::reg;

reg! {
    /// The MPU Type Register indicates how many regions the MPU support.
    /// Software can use it to determine if the processor implements an MPU.
    pub mod MPU TYPE;
    0xE000_ED90 0x20 0x0000_0000
    RReg RoReg;
    /// Instruction region.
    IREGION { 16 8 RRRegField RoRRegField }
    /// Number of regions supported by the MPU. If this field reads-as-zero the
    /// processor does not implement an MPU.
    DREGION { 8 8 RRRegField RoRRegField }
    /// Indicates support for separate instruction and data address maps.
    SEPARATE { 0 1 RRRegField RoRRegField }
}

reg! {
    /// Enables the MPU, and when the MPU is enabled, controls whether the
    /// default memory map is enabled as a background region for privileged
    /// accesses, and whether the MPU is enabled for HardFaults, NMIs, and
    /// exception handlers when FAULTMASK is set to 1.
    pub mod MPU CTRL;
    0xE000_ED94 0x20 0x0000_0000
    RReg WReg;
    /// Enables the default memory map as a background region for privileged
    /// access.
    PRIVDEFENA { 2 1 RRRegField WWRegField }
    /// Enables the operation of MPU during hard fault, NMI, and FAULTMASK
    /// handlers.
    HFNMIENA { 1 1 RRRegField WWRegField }
    /// Enables the MPU.
    ENABLE { 0 1 RRRegField WWRegField }
}

reg! {
    /// Selects the region currently accessed by RBAR and RASR.
    pub mod MPU RNR;
    0xE000_ED98 0x20 0x0000_0000
    RReg WReg;
    /// Indicates the memory region accessed by RBAR and RASR.
    REGION { 0 8 RRRegField WWRegField }
}

reg! {
    /// Holds the base address of the region identified by RNR. On a write, can
    /// also be used to update the base address of a specified region, in the
    /// range 0 to 15, updating RNR with the new region number.
    pub mod MPU RBAR;
    0xE000_ED9C 0x20 0x0000_0000
    RReg WReg;
    /// Region base address field.
    ADDR { 5 27 RRRegField WWRegField }
    /// MPU region number valid.
    VALID { 4 1 RRRegField WWRegField }
    /// MPU region field.
    REGION { 0 4 RRRegField WWRegField }
}

reg! {
    /// Defines the size and access behavior of the region identified by RNR,
    /// and enables that region.
    pub mod MPU RASR;
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
