//! Drone for STM32. Procedural macros.
//!
//! See `drone-stm32` documentation for details.

#![feature(proc_macro)]
#![doc(html_root_url = "https://docs.rs/drone-stm32-macros/0.8.0")]
#![recursion_limit = "256"]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(precedence))]

#[macro_use]
extern crate drone_macros_core;
#[macro_use]
extern crate failure_dup as failure;
extern crate inflector;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

mod interrupt;
mod vtable;

use proc_macro::TokenStream;

#[doc(hidden)]
#[proc_macro]
pub fn interrupt(input: TokenStream) -> TokenStream {
  tokens!(interrupt::interrupt(input))
}

#[doc(hidden)]
#[proc_macro]
pub fn vtable(input: TokenStream) -> TokenStream {
  tokens!(vtable::vtable(input))
}
