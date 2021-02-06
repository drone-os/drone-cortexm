//! Procedural macros for [drone-cortexm].
//!
//! [drone-cortexm]: https://github.com/drone-os/drone-cortexm

#![recursion_limit = "256"]
#![feature(unsafe_block_in_unsafe_fn)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic)]

extern crate proc_macro;

mod sv_pool;
mod thr_nvic;

use proc_macro::TokenStream;

#[proc_macro]
pub fn sv_pool(input: TokenStream) -> TokenStream {
    sv_pool::proc_macro(input)
}

#[proc_macro]
pub fn thr_nvic(input: TokenStream) -> TokenStream {
    thr_nvic::proc_macro(input)
}
