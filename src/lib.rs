//! ARM® Cortex®-M platform crate for Drone, an Embedded Operating System.
//!
//! # Supported Cores
#![doc /* flags */ = "

| Architecture | Core name              | Build target                | `drone_cortexm` Rust flag |
|--------------|------------------------|-----------------------------|---------------------------|
| ARMv6-M      | ARM® Cortex®-M0+ r0p0  | `thumbv6m-none-eabi`        | `cortexm0plus_r0p0`       |
| ARMv6-M      | ARM® Cortex®-M0+ r0p1  | `thumbv6m-none-eabi`        | `cortexm0plus_r0p1`       |
| ARMv7-M      | ARM® Cortex®-M3 r0p0   | `thumbv7m-none-eabi`        | `cortexm3_r0p0`           |
| ARMv7-M      | ARM® Cortex®-M3 r1p0   | `thumbv7m-none-eabi`        | `cortexm3_r1p0`           |
| ARMv7-M      | ARM® Cortex®-M3 r1p1   | `thumbv7m-none-eabi`        | `cortexm3_r1p1`           |
| ARMv7-M      | ARM® Cortex®-M3 r2p0   | `thumbv7m-none-eabi`        | `cortexm3_r2p0`           |
| ARMv7-M      | ARM® Cortex®-M3 r2p1   | `thumbv7m-none-eabi`        | `cortexm3_r2p1`           |
| ARMv7E-M     | ARM® Cortex®-M4 r0p0   | `thumbv7em-none-eabi`       | `cortexm4_r0p0`           |
| ARMv7E-M     | ARM® Cortex®-M4 r0p1   | `thumbv7em-none-eabi`       | `cortexm4_r0p1`           |
| ARMv7E-M     | ARM® Cortex®-M4F r0p0  | `thumbv7em-none-eabihf`     | `cortexm4f_r0p0`          |
| ARMv7E-M     | ARM® Cortex®-M4F r0p1  | `thumbv7em-none-eabihf`     | `cortexm4f_r0p1`          |
| ARMv8-M      | ARM® Cortex®-M33 r0p2  | `thumbv8m.main-none-eabi`   | `cortexm33_r0p2`          |
| ARMv8-M      | ARM® Cortex®-M33 r0p3  | `thumbv8m.main-none-eabi`   | `cortexm33_r0p3`          |
| ARMv8-M      | ARM® Cortex®-M33 r0p4  | `thumbv8m.main-none-eabi`   | `cortexm33_r0p4`          |
| ARMv8-M      | ARM® Cortex®-M33F r0p2 | `thumbv8m.main-none-eabihf` | `cortexm33f_r0p2`         |
| ARMv8-M      | ARM® Cortex®-M33F r0p3 | `thumbv8m.main-none-eabihf` | `cortexm33f_r0p3`         |
| ARMv8-M      | ARM® Cortex®-M33F r0p4 | `thumbv8m.main-none-eabihf` | `cortexm33f_r0p4`         |

"]
//! Rust target triple and `drone_cortexm` Rust flag should be set at the
//! application level according to this table.
//!
//! # Documentation
//!
//! - [Drone Book](https://book.drone-os.com/)
//! - [API documentation](https://api.drone-os.com/drone-cortexm/0.15/)
//!
//! # Usage
//!
//! Add the crate to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! drone-cortexm = { version = "0.15.0", features = [...] }
//! ```
//!
//! Add or extend `host` feature as follows:
//!
//! ```toml
//! [features]
//! host = ["drone-cortexm/host"]
//! ```

#![feature(asm_const)]
#![feature(exhaustive_patterns)]
#![feature(marker_trait_attr)]
#![feature(naked_functions)]
#![feature(never_type)]
#![feature(never_type_fallback)]
#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![warn(missing_docs, unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::inline_always,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::precedence,
    clippy::shadow_unrelated,
    clippy::type_repetition_in_bounds
)]
#![cfg_attr(not(feature = "host"), no_std)]

extern crate alloc;

pub mod drv;
pub mod fib;
pub mod map;
pub mod platform;
pub mod proc_loop;
pub mod reg;
pub mod sv;
pub mod thr;

mod drone_core_macro_reexport {
    pub use drone_core::{reg, thr};
}

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
pub use drone_core_macro_reexport::*;
