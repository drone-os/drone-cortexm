//! The Drone ARM Cortex-M Prelude.
//!
//! It is an analogue of [`std::prelude`], which is not available in
//! `#![no_std]` contexts.
//!
//! To automatically inject the imports into every module, place this code to
//! the crate root:
//!
//! ```
//! #![feature(prelude_import)]
//!
//! #[prelude_import]
//! #[allow(unused_imports)]
//! use drone_cortex_m::prelude::*;
//! ```
//!
//! [`std::prelude`]: https://doc.rust-lang.org/std/prelude/

pub use drone_core::prelude::*;

pub use crate::thr::{FuturePlat, StreamPlat};
