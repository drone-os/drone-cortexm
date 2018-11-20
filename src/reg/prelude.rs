//! Memory-mapped registers prelude.

pub use drone_core::reg::prelude::*;

pub use super::{
  RRRegFieldBitBand, RegBitBand, RegExcl, RegHoldExcl, RwRegAtomic,
  RwRegAtomicRef, WRwRegFieldAtomic, WRwRegFieldBitAtomic,
  WRwRegFieldBitsAtomic, WWRegFieldBitBand,
};
