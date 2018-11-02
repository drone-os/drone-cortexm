//! Drone for STM32. DMAMUX driver.
//!
//! See `drone-stm32` documentation for details.

#![feature(prelude_import)]
#![no_std]
#![allow(clippy::precedence)]

#[allow(unused_imports)]
#[macro_use]
extern crate drone_core;
extern crate drone_stm32_core as drone_stm32;

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
pub mod dmamux;

#[prelude_import]
#[allow(unused_imports)]
use drone_stm32::prelude::*;
