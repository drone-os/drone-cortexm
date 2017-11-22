//! Drone ARM Cortex-M SVD bindings generator.
//!
//! See `drone-cortex-m` documentation for details.
#![feature(decl_macro)]
#![feature(range_contains)]
#![warn(missing_docs)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(precedence, doc_markdown))]

extern crate failure;
#[macro_use]
extern crate quote;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate syn;

mod device;

use device::Device;
use failure::Error;
use std::fs::File;
use std::io::Read;

/// Generate bindings from SVD.
pub fn svd_generate(input: &mut File, output: &mut File) -> Result<(), Error> {
  let mut xml = String::new();
  input.read_to_string(&mut xml)?;
  let device: Device = serde_xml_rs::deserialize(xml.as_bytes())?;
  device.generate(output)
}
