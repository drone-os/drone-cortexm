//! Memory-mapped register fields module.
//!
//! See [the top-level module documentation](self) for details.

#[doc(no_inline)]
pub use drone_core::reg::field::*;

pub use crate::reg::atomic::{WRwRegFieldAtomic, WRwRegFieldBitAtomic, WRwRegFieldBitsAtomic};
#[cfg(all(
    feature = "bit-band",
    any(
        cortex_m_core = "cortex_m3_r0p0",
        cortex_m_core = "cortex_m3_r1p0",
        cortex_m_core = "cortex_m3_r1p1",
        cortex_m_core = "cortex_m3_r2p0",
        cortex_m_core = "cortex_m3_r2p1",
        cortex_m_core = "cortex_m4_r0p0",
        cortex_m_core = "cortex_m4_r0p1",
        cortex_m_core = "cortex_m4f_r0p0",
        cortex_m_core = "cortex_m4f_r0p1",
    )
))]
pub use crate::reg::bit_band::{RRRegFieldBitBand, WWRegFieldBitBand};
