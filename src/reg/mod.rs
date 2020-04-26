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
//! |                                                                        | Field Width | Field Mode | Register Mode | Tag |
//! |-----------------------------------------------------------------------------|-----------|-------|------------|----------|
//! | [`modify`](reg::field::WRwRegFieldAtomic::modify)                           |           | write | read-write | Srt, Crt |
//! | [`set_bit`](reg::field::WRwRegFieldBitAtomic::set_bit)                      | one-bit   | write | read-write | Srt, Crt |
//! | [`clear_bit`](reg::field::WRwRegFieldBitAtomic::clear_bit)                  | one-bit   | write | read-write | Srt, Crt |
//! | [`toggle_bit`](reg::field::WRwRegFieldBitAtomic::toggle_bit)                | one-bit   | write | read-write | Srt, Crt |
//! | [`write_bits`](reg::field::WRwRegFieldBitsAtomic::write_bits)               | multi-bit | write | read-write | Srt, Crt |
//! | [`to_bit_band_ptr`](reg::field::RRRegFieldBitBand::to_bit_band_ptr)         | one-bit   | read  | read       |          |
//! | [`read_bit_band`](reg::field::RRRegFieldBitBand::read_bit_band)             | one-bit   | read  | read       |          |
//! | [`to_bit_band_mut_ptr`](reg::field::WWRegFieldBitBand::to_bit_band_mut_ptr) | one-bit   | write | write-only |          |
//! | *or*                                                                        | one-bit   | write | write      | Urt      |
//! | [`set_bit_band`](reg::field::WWRegFieldBitBand::set_bit_band)               | one-bit   | write | write-only |          |
//! | *or*                                                                        | one-bit   | write | write      | Urt      |
//! | [`clear_bit_band`](reg::field::WWRegFieldBitBand::clear_bit_band)           | one-bit   | write | write-only |          |
//! | *or*                                                                        | one-bit   | write | write      | Urt      |
//!
//! ## Register Token
//!
//! |                                           | Mode       | Tag      |
//! |-------------------------------------------|------------|----------|
//! | [`modify`](reg::RwRegAtomic::modify)      | read-write | Srt, Crt |

pub mod field;
pub mod marker;
pub mod prelude;

mod atomic;
#[cfg(all(
    feature = "bit-band",
    any(
        cortexm_core = "cortexm3_r0p0",
        cortexm_core = "cortexm3_r1p0",
        cortexm_core = "cortexm3_r1p1",
        cortexm_core = "cortexm3_r2p0",
        cortexm_core = "cortexm3_r2p1",
        cortexm_core = "cortexm4_r0p0",
        cortexm_core = "cortexm4_r0p1",
        cortexm_core = "cortexm4f_r0p0",
        cortexm_core = "cortexm4f_r0p1",
    )
))]
mod bit_band;

#[doc(no_inline)]
pub use drone_core::reg::*;

pub use self::atomic::RwRegAtomic;
#[cfg(all(
    feature = "bit-band",
    any(
        cortexm_core = "cortexm3_r0p0",
        cortexm_core = "cortexm3_r1p0",
        cortexm_core = "cortexm3_r1p1",
        cortexm_core = "cortexm3_r2p0",
        cortexm_core = "cortexm3_r2p1",
        cortexm_core = "cortexm4_r0p0",
        cortexm_core = "cortexm4_r0p1",
        cortexm_core = "cortexm4f_r0p0",
        cortexm_core = "cortexm4f_r0p1",
    )
))]
pub use self::bit_band::{RegBitBand, BIT_BAND_BASE, BIT_BAND_WIDTH};
