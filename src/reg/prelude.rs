//! The Memory-Mapped Registers prelude.
//!
//! The purpose of this module is to alleviate imports of many common `reg`
//! traits by adding a glob import to the top of `reg` heavy modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use drone_cortex_m::reg::prelude::*;
//! ```

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
#[doc(no_inline)]
pub use crate::reg::RegBitBand;

#[doc(no_inline)]
pub use drone_core::reg::prelude::*;

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
#[doc(no_inline)]
pub use crate::reg::field::{RRRegFieldBitBand as _, WWRegFieldBitBand as _};
#[doc(no_inline)]
pub use crate::reg::{
    field::{WRwRegFieldAtomic as _, WRwRegFieldBitAtomic as _, WRwRegFieldBitsAtomic as _},
    RwRegAtomic as _,
};
