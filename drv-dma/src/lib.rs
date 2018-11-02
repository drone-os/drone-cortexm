//! Drone for STM32. DMA driver.
//!
//! See `drone-stm32` documentation for details.

#![feature(generators)]
#![feature(prelude_import)]
#![no_std]
#![allow(clippy::precedence)]

#[allow(unused_imports)]
#[macro_use]
extern crate drone_core;
extern crate drone_stm32_core as drone_stm32;
extern crate drone_stm32_drv_dmamux;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate futures;

pub mod dma;

#[prelude_import]
#[allow(unused_imports)]
use drone_stm32::prelude::*;