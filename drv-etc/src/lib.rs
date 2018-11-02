//! Drone for STM32. Drivers.
//!
//! See `drone-stm32` documentation for details.

#![feature(asm)]
#![feature(generators)]
#![feature(never_type)]
#![feature(prelude_import)]
#![no_std]
#![allow(clippy::precedence)]

#[allow(unused_imports)]
#[macro_use]
extern crate drone_core;
extern crate drone_stm32_core as drone_stm32;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate futures;

pub mod exti;
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
pub mod fpu;
pub mod gpio;
pub mod nvic;
pub mod thr;
pub mod timer;

#[prelude_import]
#[allow(unused_imports)]
use drone_stm32::prelude::*;
