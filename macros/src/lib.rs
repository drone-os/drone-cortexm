//! Procedural macros for [drone-cortexm].
//!
//! [drone-cortexm]: https://github.com/drone-os/drone-cortexm

#![recursion_limit = "256"]
#![feature(unsafe_block_in_unsafe_fn)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic)]

extern crate proc_macro;

mod sv;
mod thr;

use proc_macro::TokenStream;

#[proc_macro]
pub fn sv(input: TokenStream) -> TokenStream {
    sv::proc_macro(input)
}

#[proc_macro]
pub fn thr(input: TokenStream) -> TokenStream {
    thr::proc_macro(input)
}
