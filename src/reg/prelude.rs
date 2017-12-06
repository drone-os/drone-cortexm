//! Memory-mapped registers prelude.

pub use drone::reg::prelude::*;

pub use super::{RRegFieldBitBand, RegBitBand, RegExcl, RegHoldExcl,
                RwRegShared, RwRegSharedRef, WRegFieldBitBand,
                WRwRegFieldBitShared, WRwRegFieldBitsShared, WRwRegFieldShared};
