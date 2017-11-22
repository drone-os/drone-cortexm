//! Drone for ARM Cortex-M procedural macros.
//!
//! See `drone-cortex-m` documentation for details.
#![feature(decl_macro)]
#![feature(proc_macro)]
#![recursion_limit = "256"]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(precedence, doc_markdown))]

#[macro_use]
extern crate failure;
extern crate inflector;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

mod vtable;

use proc_macro::TokenStream;

#[doc(hidden)]
#[proc_macro]
pub fn vtable(input: TokenStream) -> TokenStream {
  tokens!(vtable::vtable(input))
}

macro tokens($tokens:expr) {
  match $tokens {
    Ok(tokens) => tokens.parse().unwrap(),
    Err(error) => panic!("{}", error),
  }
}
