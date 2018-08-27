//! Drone STM32 SVD bindings generator.
//!
//! See `drone-stm32` documentation for details.

#![feature(range_contains)]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/drone-stm32-svd/0.8.3")]
#![cfg_attr(feature = "cargo-clippy", allow(precedence))]

extern crate drone_mirror_failure as failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

mod device;

use device::Device;
use failure::Error;
use std::fs::File;
use std::io::prelude::*;

/// Generate bindings from SVD.
pub fn svd_generate(
  input: &mut File,
  reg_map: &mut File,
  reg_tokens: &mut File,
  interrupts: &mut File,
) -> Result<(), Error> {
  let mut xml = String::new();
  input.read_to_string(&mut xml)?;
  let device: Device = serde_xml_rs::deserialize(xml.as_bytes())?;
  device.generate(reg_map, reg_tokens, interrupts)
}
