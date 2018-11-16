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
//! [Drone]: https://github.com/drone-os/drone
//! [OpenOCD]: http://openocd.org/
//! [rules.d]: https://github.com/texane/stlink/tree/master/etc/udev/rules.d

#![feature(allocator_api)]
#![no_std]
#![allow(clippy::precedence)]
#![cfg_attr(test, feature(allocator_internals))]
#![cfg_attr(test, default_lib_allocator)]

extern crate drone_core;
extern crate drone_stm32_core;
extern crate drone_stm32_device;
extern crate drone_stm32_drv_adc;
extern crate drone_stm32_drv_dma;
extern crate drone_stm32_drv_dmamux;
extern crate drone_stm32_drv_etc;
extern crate drone_stm32_drv_i2c;
extern crate drone_stm32_drv_i2c_master;
extern crate drone_stm32_drv_spi;
#[cfg(test)]
extern crate test;

pub mod drv;

pub use drone_stm32_core::*;
pub use drone_stm32_device::{reg, thr};

#[cfg(test)]
use drone_core::heap;

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
