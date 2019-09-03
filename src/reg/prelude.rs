//! The Memory-Mapped Registers prelude.
//!
//! The purpose of this module is to alleviate imports of many common `reg`
//! traits by adding a glob import to the top of `reg` heavy modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use drone_cortex_m::reg::prelude::*;
//! ```

#[doc(no_inline)]
pub use crate::reg::RegBitBand;

#[doc(no_inline)]
pub use drone_core::reg::prelude::*;

#[doc(no_inline)]
pub use crate::reg::{
    field::{
        RRRegFieldBitBand as _, WRwRegFieldAtomic as _, WRwRegFieldBitAtomic as _,
        WRwRegFieldBitsAtomic as _, WWRegFieldBitBand as _,
    },
    RwRegAtomic as _,
};
