use drone_core::reg;

use crate::reg::prelude::*;

reg! {
    /// The MPU Type Register indicates how many regions the MPU support.
    /// Software can use it to determine if the processor implements an MPU.
    pub MPU TYPE => {
        address => 0xE000_ED90;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg RoReg };
        fields => {
            /// Instruction region.
            IREGION => { offset => 16; width => 8; traits => { RRRegField RoRRegField } };
            /// Number of regions supported by the MPU. If this field reads-as-zero the
            /// processor does not implement an MPU.
            DREGION => { offset => 8; width => 8; traits => { RRRegField RoRRegField } };
            /// Indicates support for separate instruction and data address maps.
            SEPARATE => { offset => 0; width => 1; traits => { RRRegField RoRRegField } };
        };
    };
}

reg! {
    /// Enables the MPU, and when the MPU is enabled, controls whether the
    /// default memory map is enabled as a background region for privileged
    /// accesses, and whether the MPU is enabled for HardFaults, NMIs, and
    /// exception handlers when FAULTMASK is set to 1.
    pub MPU CTRL => {
        address => 0xE000_ED94;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Enables the default memory map as a background region for privileged
            /// access.
            PRIVDEFENA => { offset => 2; width => 1; traits => { RRRegField WWRegField } };
            /// Enables the operation of MPU during hard fault, NMI, and FAULTMASK
            /// handlers.
            HFNMIENA => { offset => 1; width => 1; traits => { RRRegField WWRegField } };
            /// Enables the MPU.
            ENABLE => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Selects the region currently accessed by RBAR and RASR.
    pub MPU RNR => {
        address => 0xE000_ED98;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Indicates the memory region accessed by RBAR and RASR.
            REGION => { offset => 0; width => 8; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Holds the base address of the region identified by RNR. On a write, can
    /// also be used to update the base address of a specified region, in the
    /// range 0 to 15, updating RNR with the new region number.
    pub MPU RBAR => {
        address => 0xE000_ED9C;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Region base address field.
            ADDR => { offset => 5; width => 27; traits => { RRRegField WWRegField } };
            /// MPU region number valid.
            VALID => { offset => 4; width => 1; traits => { RRRegField WWRegField } };
            /// MPU region field.
            REGION => { offset => 0; width => 4; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Defines the size and access behavior of the region identified by RNR,
    /// and enables that region.
    pub MPU RASR => {
        address => 0xE000_EDA0;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Instruction access disable bit.
            XN => { offset => 28; width => 1; traits => { RRRegField WWRegField } };
            /// Access permission.
            AP => { offset => 24; width => 3; traits => { RRRegField WWRegField } };
            /// Memory attribute.
            TEX => { offset => 19; width => 3; traits => { RRRegField WWRegField } };
            /// Shareable memory attribute.
            S => { offset => 18; width => 1; traits => { RRRegField WWRegField } };
            /// Memory attribute.
            C => { offset => 17; width => 1; traits => { RRRegField WWRegField } };
            /// Memory attribute.
            B => { offset => 16; width => 1; traits => { RRRegField WWRegField } };
            /// Subregion disable bits.
            SRD => { offset => 8; width => 8; traits => { RRRegField WWRegField } };
            /// Size of the MPU protection region.
            SIZE => { offset => 1; width => 5; traits => { RRRegField WWRegField } };
            /// Region enable bit.
            ENABLE => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}
