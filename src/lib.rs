//! [Drone] implementation for STM32 microcontrollers.
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
//! # Development
//!
//! Check:
//!
//! ```sh
//! $ RUSTC_WRAPPER=./clippy-wrapper.sh cargo check --all --exclude drone-stm32
//! $ RUSTC_WRAPPER=./clippy-wrapper.sh xargo check \
//!   --target "thumbv7m-none-eabi" -p drone-stm32
//! ```
//!
//! Test:
//!
//! ```sh
//! $ RUSTC_WRAPPER=./rustc-wrapper.sh cargo test --all --exclude drone-stm32
//! $ RUSTC_WRAPPER=./rustc-wrapper.sh cargo drone test -p drone-stm32
//! ```
//!
//! Readme update:
//!
//! ```sh
//! $ cargo readme -o README.md
//! ```
//!
//! [Drone]: https://github.com/drone-os/drone
//! [OpenOCD]: http://openocd.org/
//! [rules.d]: https://github.com/texane/stlink/tree/master/etc/udev/rules.d

#![feature(alloc)]
#![feature(allocator_api)]
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
#![feature(range_contains)]
#![feature(tool_lints)]
#![feature(untagged_unions)]
#![no_std]
#![warn(missing_docs)]
#![allow(clippy::precedence, clippy::inline_always)]
#![doc(html_root_url = "https://docs.rs/drone-stm32/0.9.0")]
#![cfg_attr(test, feature(allocator_internals))]
#![cfg_attr(test, default_lib_allocator)]

extern crate alloc;
#[allow(clippy::useless_attribute)]
#[allow(unused_imports)]
#[macro_use]
extern crate drone_core;
extern crate drone_stm32_macros;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate futures;
#[cfg(test)]
#[macro_use]
extern crate test;

#[macro_use]
pub mod itm;

pub mod cpu;
pub mod drv;
pub mod fib;
pub mod prelude;
pub mod reg;
pub mod stack_adapter;
pub mod sv;
pub mod thr;

mod lang_items;

pub use drone_stm32_macros::{sv, vtable};

#[cfg(test)]
use drone_core::heap;

#[prelude_import]
#[allow(unused_imports)]
use prelude::*;

#[cfg(test)]
heap! {
  struct Heap;
  extern fn alloc_hook;
  extern fn dealloc_hook;
  size = 0x40000;
  pools = [
    [0x4; 0x4000],
    [0x20; 0x800],
    [0x100; 0x100],
    [0x800; 0x20],
  ];
}

#[cfg(test)]
#[global_allocator]
static mut GLOBAL: Heap = Heap::new();

#[cfg(test)]
fn alloc_hook(
  _layout: ::core::alloc::Layout,
  _pool: &::drone_core::heap::Pool,
) {
}

#[cfg(test)]
fn dealloc_hook(
  _layout: ::core::alloc::Layout,
  _pool: &::drone_core::heap::Pool,
) {
}
