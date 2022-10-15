//! The Memory-Mapped Registers prelude.
//!
//! The purpose of this module is to alleviate imports of many common `reg`
//! traits by adding a glob import to the top of `reg` heavy modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use drone_cortexm::reg::prelude::*;
//! ```

#[cfg(feature = "bit-band")]
#[doc(no_inline)]
pub use crate::reg::field::{RRRegFieldBitBand as _, WWRegFieldBitBand as _};
#[cfg(feature = "bit-band")]
#[doc(no_inline)]
pub use crate::reg::RegBitBand;
#[doc(no_inline)]
pub use crate::reg::{
    field::{WRwRegFieldAtomic as _, WRwRegFieldBitAtomic as _, WRwRegFieldBitsAtomic as _},
    RwRegAtomic as _,
};
#[doc(no_inline)]
pub use drone_core::reg::prelude::*;
