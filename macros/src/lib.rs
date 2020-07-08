//! Procedural macros for [drone-cortexm].
//!
//! [drone-cortexm]: https://github.com/drone-os/drone-cortexm

#![recursion_limit = "256"]
#![feature(unsafe_block_in_unsafe_fn)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic)]

extern crate proc_macro;

mod int;
mod sv;
mod vtable;

use proc_macro::TokenStream;

#[proc_macro]
pub fn int(input: TokenStream) -> TokenStream {
    int::proc_macro(input)
}

#[proc_macro]
pub fn sv(input: TokenStream) -> TokenStream {
    sv::proc_macro(input)
}

#[proc_macro]
pub fn vtable(input: TokenStream) -> TokenStream {
    vtable::proc_macro(input)
}
