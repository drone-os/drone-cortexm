//! Memory-mapped register fields module.
//!
//! See [the top-level module documentation](self) for details.

#[doc(no_inline)]
pub use drone_core::reg::field::*;

pub use crate::reg::atomic::{WRwRegFieldAtomic, WRwRegFieldBitAtomic, WRwRegFieldBitsAtomic};
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
pub use crate::reg::bit_band::{RRRegFieldBitBand, WWRegFieldBitBand};
