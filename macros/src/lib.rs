//! Procedural macros for [drone-cortex-m].
//!
//! [drone-cortex-m]: https://github.com/drone-os/drone-cortex-m

#![recursion_limit = "256"]
#![deny(elided_lifetimes_in_paths)]
#![warn(clippy::pedantic)]

extern crate proc_macro;

mod int;
mod itm_update_prescaler;
mod sv;
mod vtable;

use proc_macro::TokenStream;

#[proc_macro]
pub fn int(input: TokenStream) -> TokenStream {
    int::proc_macro(input)
}

#[proc_macro]
pub fn itm_update_prescaler(input: TokenStream) -> TokenStream {
    itm_update_prescaler::proc_macro(input)
}

#[proc_macro]
pub fn sv(input: TokenStream) -> TokenStream {
    sv::proc_macro(input)
}

#[proc_macro]
pub fn vtable(input: TokenStream) -> TokenStream {
    vtable::proc_macro(input)
}
