//! Memory-mapped registers prelude.

pub use super::RegBitBand;
pub use drone_core::reg::prelude::*;

pub use super::{
  RRRegFieldBitBand as _, RegExcl as _, RegHoldExcl as _, RwRegAtomic as _,
  RwRegAtomicRef as _, WRwRegFieldAtomic as _, WRwRegFieldBitAtomic as _,
  WRwRegFieldBitsAtomic as _, WWRegFieldBitBand as _,
};
