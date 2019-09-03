//! Memory-mapped register fields module.
//!
//! See [the top-level module documentation](self) for details.

#[doc(no_inline)]
pub use drone_core::reg::field::*;

pub use crate::reg::{
    atomic::{WRwRegFieldAtomic, WRwRegFieldBitAtomic, WRwRegFieldBitsAtomic},
    bit_band::{RRRegFieldBitBand, WWRegFieldBitBand},
};
