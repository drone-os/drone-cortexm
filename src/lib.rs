//! [Drone] implementation for ARM Cortex-M microcontrollers.
//!
//! # Installation
//!
//! Instructions will be given for Debian-based Linux systems.
//!
//! Install the following packages:
//!
//! ```sh
//! $ sudo apt-get install build-essential cmake libusb-1.0-0 libusb-1.0-0-dev \
//!   pandoc gcc-arm-none-eabi gdb-arm-none-eabi qemu-system-arm qemu-user
//! ```
//!
//! Copy [udev rules][rules.d] for ST-Link programmer to the
//! `/etc/udev/rules.d/`, and run the following commands:
//!
//! ```sh
//! $ sudo udevadm control --reload-rules
//! $ sudo udevadm trigger
//! ```
//!
//! [OpenOCD] is required. It is recommended to install it from the source,
//! because repository package is outdated and doesn't contain configuration for
//! newer chips and boards.
//!
//! [Drone]: https://github.com/drone-os/drone
//! [OpenOCD]: http://openocd.org/
//! [rules.d]: https://github.com/texane/stlink/tree/master/etc/udev/rules.d

#![feature(asm)]
#![feature(associated_type_defaults)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(exhaustive_patterns)]
#![feature(generators)]
#![feature(generator_trait)]
#![feature(lang_items)]
#![feature(linkage)]
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
    clippy::enum_glob_use,
    clippy::inline_always,
    clippy::module_name_repetitions,
    clippy::precedence,
    clippy::shadow_unrelated,
    clippy::type_repetition_in_bounds,
    clippy::use_self
)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod cpu;
pub mod drv;
pub mod fib;
pub mod itm;
pub mod map;
pub mod prelude;
pub mod reg;
pub mod stack_loop;
pub mod sv;
pub mod thr;

#[cfg(not(feature = "std"))]
mod lang_items;

pub use drone_cortex_m_macros::{sv, vtable};

#[prelude_import]
#[allow(unused_imports)]
use crate::prelude::*;
