//! Drone for STM32. I2C Master driver.
//!
//! See `drone-stm32` documentation for details.

#![feature(exhaustive_patterns)]
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
extern crate drone_stm32_drv_i2c;
extern crate failure;
#[allow(unused_imports)]
#[macro_use]
extern crate failure_derive;
extern crate futures;

#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
pub mod i2c_master;

#[prelude_import]
#[allow(unused_imports)]
use drone_stm32_core::prelude::*;
