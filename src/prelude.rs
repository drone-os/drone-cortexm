//! The Drone Cortex-M Prelude.
//!
//! **NOTE** This module documentation should be viewed as a continuation of
//! [the `drone_core` documentation](drone_core::prelude).
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

#[doc(no_inline)]
pub use drone_core::prelude::*;

#[cfg(not(feature = "std"))]
#[doc(no_inline)]
pub use crate::{dbg, eprint, eprintln, print, println};

#[cfg(feature = "std")]
#[doc(no_inline)]
pub use std::{dbg, eprint, eprintln, print, println};
