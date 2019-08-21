//! Memory-mapped registers prelude.

pub use crate::reg::RegBitBand;
pub use drone_core::reg::prelude::*;

pub use crate::reg::{
    field::{
        RRRegFieldBitBand as _, WRwRegFieldAtomic as _, WRwRegFieldBitAtomic as _,
        WRwRegFieldBitsAtomic as _, WWRegFieldBitBand as _,
    },
    RegExcl as _, RegHoldExcl as _, RwRegAtomic as _, RwRegAtomicRef as _,
};
