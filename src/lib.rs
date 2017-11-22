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
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(prelude_import)]
#![feature(proc_macro)]
#![no_std]
#![warn(missing_docs)]
#![cfg_attr(test, feature(alloc))]
#![cfg_attr(test, feature(allocator_api))]
#![cfg_attr(test, feature(allocator_internals))]
#![cfg_attr(test, feature(compiler_builtins_lib))]
#![cfg_attr(test, feature(global_allocator))]
#![cfg_attr(test, feature(slice_get_slice))]
#![cfg_attr(test, default_lib_allocator)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(precedence, doc_markdown, inline_always))]

#[cfg(test)]
extern crate alloc;
#[cfg(test)]
extern crate compiler_builtins;
extern crate drone;
extern crate drone_cortex_m_macros;
extern crate futures;
#[cfg(test)]
#[macro_use]
extern crate test;

#[macro_use]
pub mod itm;
pub mod mcu;
pub mod panicking;
pub mod peripherals;
pub mod prelude;
pub mod reg;
pub mod task;
pub mod vtable;

pub use drone_cortex_m_macros::vtable;

#[prelude_import]
#[allow(unused_imports)]
use prelude::*;

#[cfg(test)]
drone::heap! {
  #![global_allocator]
}
