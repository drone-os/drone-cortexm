//! The Memory-Mapped Registers module.
//!
//! **NOTE** This module documentation should be viewed as a continuation of
//! [the `drone_core` documentation](mod@drone_core::reg).
//!
//! # API
//!
//! These API tables extend [the `drone_core` API tables](mod@drone_core::reg).
//!
//! ## Field Token
#![doc = "

|                                                                   | Field Width | Field Mode | Register Mode | Tag |
|------------------------------------------------------------------------|-----------|-------|------------|----------|
| [`to_bit_band_ptr`](field::RRRegFieldBitBand::to_bit_band_ptr)         | one-bit   | read  | read       |          |
| [`read_bit_band`](field::RRRegFieldBitBand::read_bit_band)             | one-bit   | read  | read       |          |
| [`to_bit_band_mut_ptr`](field::WWRegFieldBitBand::to_bit_band_mut_ptr) | one-bit   | write | write-only |          |
| *or*                                                                   | one-bit   | write | write      | Urt      |
| [`set_bit_band`](field::WWRegFieldBitBand::set_bit_band)               | one-bit   | write | write-only |          |
| *or*                                                                   | one-bit   | write | write      | Urt      |
| [`clear_bit_band`](field::WWRegFieldBitBand::clear_bit_band)           | one-bit   | write | write-only |          |
| *or*                                                                   | one-bit   | write | write      | Urt      |

"]

pub mod field;
pub mod marker;
pub mod prelude;

#[cfg(feature = "bit-band")]
mod bit_band;

#[cfg(feature = "bit-band")]
pub use self::bit_band::{RegBitBand, BIT_BAND_BASE, BIT_BAND_WIDTH};
#[doc(no_inline)]
pub use drone_core::reg::*;
