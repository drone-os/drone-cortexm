//! Drone for STM32. SPI driver.
//!
//! See `drone-stm32` documentation for details.

#![feature(generators)]
#![feature(never_type)]
#![feature(prelude_import)]
#![no_std]
#![warn(missing_docs)]
#![allow(clippy::precedence)]

#[allow(unused_imports)]
#[macro_use]
extern crate drone_core;
extern crate drone_stm32_core;
extern crate drone_stm32_device;
extern crate drone_stm32_drv_dma;
extern crate drone_stm32_drv_dmamux;
extern crate failure;
#[allow(unused_imports)]
#[macro_use]
extern crate failure_derive;
extern crate futures;

pub mod spi;

#[prelude_import]
#[allow(unused_imports)]
use drone_stm32_core::prelude::*;
