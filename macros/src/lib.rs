//! Drone for ARM Cortex-M. Procedural macros.
//!
//! See `drone-cortex-m` documentation for details.

#![recursion_limit = "256"]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![warn(clippy::pedantic)]

extern crate proc_macro;

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
