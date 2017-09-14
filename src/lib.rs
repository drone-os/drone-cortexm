//! *Drone* bindings for *STM32* microcontrollers.
#![feature(alloc)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(drop_types_in_const)]
#![feature(fnbox)]
#![feature(generators)]
#![feature(generator_trait)]
#![feature(lang_items)]
#![feature(linkage)]
#![no_std]
#![warn(missing_docs)]
#![cfg_attr(test, feature(compiler_builtins_lib))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(precedence, doc_markdown))]

extern crate alloc;
#[cfg(test)]
extern crate compiler_builtins;
#[macro_use]
extern crate drone;
#[cfg(test)]
#[macro_use]
extern crate test;

#[macro_use]
pub mod itm;
pub mod panicking;
pub mod reg;
pub mod mcu;
#[macro_use]
pub mod vtable;
