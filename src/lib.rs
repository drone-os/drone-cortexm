//! *Drone* bindings for *STM32* microcontrollers.
#![feature(decl_macro)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(linkage)]
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
#![cfg_attr(feature = "clippy", allow(precedence, doc_markdown))]

#[cfg(test)]
extern crate alloc;
#[cfg(test)]
extern crate compiler_builtins;
extern crate drone;
extern crate drone_cortex_m_macros;
#[cfg(test)]
#[macro_use]
extern crate test;

#[macro_use]
pub mod itm;
pub mod panicking;
pub mod reg;
pub mod mcu;
pub mod vtable;

pub use vtable::vtable;

#[cfg(test)]
drone::heap! {
  #![global_allocator]
}
