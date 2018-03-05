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
extern crate inflector;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod sv;
mod thr_int;
mod vtable;

use proc_macro::TokenStream;

#[proc_macro]
pub fn sv(input: TokenStream) -> TokenStream {
  sv::proc_macro(input)
}

#[proc_macro]
pub fn thr_int(input: TokenStream) -> TokenStream {
  thr_int::proc_macro(input)
}

#[proc_macro]
pub fn vtable(input: TokenStream) -> TokenStream {
  vtable::proc_macro(input)
}
