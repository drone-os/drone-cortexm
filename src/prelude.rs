//! The Drone Cortex-M Prelude.
//!
//! This module re-exports:
//! * Contents of [`drone_core::prelude`].
//! * [`print`] and [`println`] macros.
//! * Future and Stream extensions.
//!
//! By default Rust automatically injects libcore prelude imports into every
//! module. To inject the Drone prelude instead, place the following code to the
//! `src/lib.rs`:
//!
//! ```
//! #![feature(prelude_import)]
//!
//! #[prelude_import]
//! #[allow(unused_imports)]
//! use drone_cortex_m::prelude::*;
//! ```

pub use drone_core::prelude::*;

pub use crate::thr::{FutureExt as _, StreamExt as _};

#[cfg(not(feature = "std"))]
pub use crate::{print, println};

#[cfg(feature = "std")]
pub use std::{print, println};
