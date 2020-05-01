//! The Memory-Mapped Registers module.
//!
//! **NOTE** This module documentation should be viewed as a continuation of
//! [the `drone_core` documentation](drone_core::reg).
//!
//! # API
//!
//! These API tables extend [the `drone_core` API tables](drone_core::reg).
//!
//! ## Field Token
//!
//! |                                                                   | Field Width | Field Mode | Register Mode | Tag |
//! |------------------------------------------------------------------------|-----------|-------|------------|----------|
//! | [`modify`](field::WRwRegFieldAtomic::modify)                           |           | write | read-write | Srt, Crt |
//! | [`set_bit`](field::WRwRegFieldBitAtomic::set_bit)                      | one-bit   | write | read-write | Srt, Crt |
//! | [`clear_bit`](field::WRwRegFieldBitAtomic::clear_bit)                  | one-bit   | write | read-write | Srt, Crt |
//! | [`toggle_bit`](field::WRwRegFieldBitAtomic::toggle_bit)                | one-bit   | write | read-write | Srt, Crt |
//! | [`write_bits`](field::WRwRegFieldBitsAtomic::write_bits)               | multi-bit | write | read-write | Srt, Crt |
//! | [`to_bit_band_ptr`](field::RRRegFieldBitBand::to_bit_band_ptr)         | one-bit   | read  | read       |          |
//! | [`read_bit_band`](field::RRRegFieldBitBand::read_bit_band)             | one-bit   | read  | read       |          |
//! | [`to_bit_band_mut_ptr`](field::WWRegFieldBitBand::to_bit_band_mut_ptr) | one-bit   | write | write-only |          |
//! | *or*                                                                   | one-bit   | write | write      | Urt      |
//! | [`set_bit_band`](field::WWRegFieldBitBand::set_bit_band)               | one-bit   | write | write-only |          |
//! | *or*                                                                   | one-bit   | write | write      | Urt      |
//! | [`clear_bit_band`](field::WWRegFieldBitBand::clear_bit_band)           | one-bit   | write | write-only |          |
//! | *or*                                                                   | one-bit   | write | write      | Urt      |
//!
//! ## Register Token
//!
//! |                                      | Mode       | Tag      |
//! |--------------------------------------|------------|----------|
//! | [`modify`](RwRegAtomic::modify)      | read-write | Srt, Crt |

pub mod field;
pub mod marker;
pub mod prelude;

mod atomic;
#[cfg(feature = "bit-band")]
mod bit_band;

#[doc(no_inline)]
pub use drone_core::reg::*;

pub use self::atomic::RwRegAtomic;
#[cfg(feature = "bit-band")]
pub use self::bit_band::{RegBitBand, BIT_BAND_BASE, BIT_BAND_WIDTH};
