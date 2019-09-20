//! ARM® Cortex®-M platform crate for Drone, an Embedded Operating System.
//!
//! # Supported Cores
//!
//! | Architecture | Core name             | Cargo features                      | Rust target             |
//! |--------------|-----------------------|-------------------------------------|-------------------------|
//! | ARMv7-M      | ARM® Cortex®-M3 r0p0  | `cortex_m3_r0p0`                    | `thumbv7m-none-eabi`    |
//! | ARMv7-M      | ARM® Cortex®-M3 r1p0  | `cortex_m3_r1p0`                    | `thumbv7m-none-eabi`    |
//! | ARMv7-M      | ARM® Cortex®-M3 r1p1  | `cortex_m3_r1p1`                    | `thumbv7m-none-eabi`    |
//! | ARMv7-M      | ARM® Cortex®-M3 r2p0  | `cortex_m3_r2p0`                    | `thumbv7m-none-eabi`    |
//! | ARMv7-M      | ARM® Cortex®-M3 r2p1  | `cortex_m3_r2p1`                    | `thumbv7m-none-eabi`    |
//! | ARMv7E-M     | ARM® Cortex®-M4 r0p0  | `cortex_m4_r0p0`                    | `thumbv7em-none-eabi`   |
//! | ARMv7E-M     | ARM® Cortex®-M4 r0p1  | `cortex_m4_r0p1`                    | `thumbv7em-none-eabi`   |
//! | ARMv7E-M     | ARM® Cortex®-M4F r0p0 | `cortex_m4f_r0p0`, `fpu` (optional) | `thumbv7em-none-eabihf` |
//! | ARMv7E-M     | ARM® Cortex®-M4F r0p1 | `cortex_m4f_r0p1`, `fpu` (optional) | `thumbv7em-none-eabihf` |
//!
//! **NOTE** Cargo features for `drone-cortex-m` dependency and target triple
//! for the resulting binary should be selected for a particular core according
//! this table.
//!
//! # Documentation
//!
//! - [Drone Book](https://book.drone-os.com/)
//! - [API documentation](https://docs.rs/drone-cortex-m/0.10.0)
//!
//! # Usage
//!
//! Place the following to the Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! drone-cortex-m = { version = "0.10.0", features = [...] }
//! ```

#![feature(asm)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(exhaustive_patterns)]
#![feature(lang_items)]
#![feature(marker_trait_attr)]
#![feature(naked_functions)]
#![feature(never_type)]
#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(todo_macro)]
#![feature(untagged_unions)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::inline_always,
    clippy::module_name_repetitions,
    clippy::precedence,
    clippy::shadow_unrelated,
    clippy::type_repetition_in_bounds
)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod drv;
pub mod fib;
pub mod itm;
pub mod map;
pub mod prelude;
pub mod proc_loop;
pub mod processor;
pub mod reg;
pub mod sv;
pub mod thr;

#[cfg(not(feature = "std"))]
mod lang_items;

mod drone_core_macro_reexport {
    pub use drone_core::{reg, thr};
}

pub use drone_core_macro_reexport::*;

/// Defines the supervisor type.
///
/// See [the module level documentation](sv) for details.
#[doc(inline)]
pub use drone_cortex_m_macros::sv;

#[prelude_import]
#[allow(unused_imports)]
use crate::prelude::*;
