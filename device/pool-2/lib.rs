#![no_std]
#![allow(clippy::precedence, clippy::doc_markdown)]

#[allow(unused_imports)]
#[macro_use]
extern crate drone_core;
extern crate drone_stm32_core;

pub mod reg {
  #[allow(unused_imports)]
  use drone_core::reg::map;
  #[allow(unused_imports)]
  use drone_stm32_core::reg::prelude::*;

  include!(concat!(env!("OUT_DIR"), "/svd_reg_map.rs"));
}
