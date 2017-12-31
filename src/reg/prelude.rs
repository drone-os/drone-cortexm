//! Memory-mapped registers prelude.

pub use drone_core::reg::prelude::*;

pub use super::{RRegFieldBitBand, RegBitBand, RegExcl, RegHoldExcl,
                RwRegShared, RwRegSharedRef, WRegFieldBitBand,
                WRwRegFieldBitShared, WRwRegFieldBitsShared, WRwRegFieldShared};
