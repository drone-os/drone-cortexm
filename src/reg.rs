//! Memory-mapped registers.

pub mod field;
pub mod marker;
pub mod prelude;

mod atomic;
mod bit_band;

pub use self::{
    atomic::{AtomicBits, RegExcl, RegHoldExcl, RwRegAtomic, RwRegAtomicRef},
    bit_band::{RegBitBand, BIT_BAND_BASE, BIT_BAND_WIDTH},
};
pub use drone_core::reg::*;
