//! Drone for STM32. Core.
//!
//! See `drone-stm32` documentation for details.

#![feature(alloc)]
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
#![feature(self_struct_ctor)]
#![feature(untagged_unions)]
#![no_std]
#![warn(missing_docs)]
#![allow(clippy::precedence, clippy::inline_always)]

extern crate alloc;
#[allow(unused_imports)]
#[macro_use]
extern crate drone_core;
extern crate drone_stm32_macros;
extern crate futures;

#[macro_use]
pub mod itm;

pub mod cpu;
pub mod fib;
pub mod prelude;
pub mod reg;
pub mod stack_adapter;
pub mod sv;
pub mod thr;

mod lang_items;

pub use drone_stm32_macros::{sv, vtable};

#[prelude_import]
#[allow(unused_imports)]
use prelude::*;
