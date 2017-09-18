#![feature(range_contains)]

#[macro_use]
extern crate error_chain;
extern crate inflector;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

mod errors {
  use serde_xml_rs;
  use std;

  error_chain! {
    foreign_links {
      Io(std::io::Error);
      Env(std::env::VarError);
      ParseInt(std::num::ParseIntError);
      Xml(serde_xml_rs::Error);
    }
  }
}

use errors::*;
use inflector::Inflector;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::ops::Range;
use std::path::Path;

const BIT_BAND: Range<u32> = 0x4000_0000..0x4010_0000;

lazy_static! {
  static ref RESERVED: Regex = Regex::new(r"(?x)
    ^ ( as | break | const | continue | crate | else | enum | extern | false |
    fn | for | if | impl | in | let | loop | match | mod | move | mut | pub |
    ref | return | Self | self | static | struct | super | trait | true | type |
    unsafe | use | where | while | abstract | alignof | become | box | do |
    final | macro | offsetof | override | priv | proc | pure | sizeof | typeof |
    unsized | virtual | yield ) $
  ").unwrap();
}

#[derive(Debug, Deserialize)]
struct Device {
  peripherals: Peripherals,
}

#[derive(Debug, Deserialize)]
struct Peripherals {
  peripheral: Vec<Peripheral>,
}

#[derive(Debug, Deserialize)]
struct Peripheral {
  #[serde(rename = "derivedFrom")] derived_from: Option<String>,
  name: String,
  description: Option<String>,
  #[serde(rename = "groupName")] group_name: Option<String>,
  #[serde(rename = "baseAddress")] base_address: String,
  registers: Option<Registers>,
}

#[derive(Debug, Deserialize)]
struct Registers {
  register: Vec<Register>,
}

#[derive(Debug, Deserialize)]
struct Register {
  name: String,
  description: String,
  #[serde(rename = "addressOffset")] address_offset: String,
  #[serde(default)] access: String,
  fields: Fields,
}

#[derive(Debug, Deserialize)]
struct Fields {
  field: Vec<Field>,
}

#[derive(Debug, Deserialize)]
struct Field {
  name: String,
  description: String,
  #[serde(rename = "bitOffset")] bit_offset: String,
  #[serde(rename = "bitWidth")] bit_width: String,
}

quick_main!(run);

fn run() -> Result<()> {
  let out_dir = env::var("OUT_DIR")?;
  let out_dir = Path::new(&out_dir);
  let mut svd_out = File::create(out_dir.join("svd.rs"))?;
  let mut vtable_out = File::create(out_dir.join("vtable.rs"))?;
  if let Some(svd_file) = svd_from_feature() {
    svd_generate(&mut svd_out, &mut File::open(svd_file)?)?;
  }
  vtable_generate(&mut vtable_out, irq_from_feature())?;
  Ok(())
}

fn svd_generate(output: &mut File, input: &mut File) -> Result<()> {
  let mut xml = String::new();
  input.read_to_string(&mut xml)?;
  let device: Device = serde_xml_rs::deserialize(xml.as_bytes())?;
  macro_rules! w {
    ($($x:tt)*) => {
      output.write_all(format!($($x)*).as_bytes())?;
    };
  }
  for peripheral in &device.peripherals.peripheral {
    let derived = if let Some(ref derived_from) = peripheral.derived_from {
      Some(device
        .peripherals
        .peripheral
        .iter()
        .find(|peripheral| &peripheral.name == derived_from)
        .ok_or("derivedFrom not found")?)
    } else {
      None
    };
    let mod_name = peripheral.name.to_snake_case();
    let mod_prefix = peripheral.name.to_class_case();
    let doc = peripheral
      .description
      .as_ref()
      .or_else(|| derived.and_then(|x| x.description.as_ref()))
      .ok_or("Peripheral has no description")?;
    let registers = peripheral
      .registers
      .as_ref()
      .or_else(|| derived.and_then(|x| x.registers.as_ref()))
      .ok_or("registers not found")?;
    for register in &registers.register {
      w!(
        "pub use self::{0}::{2} as {1}{2};\n",
        mod_name,
        mod_prefix,
        register.name.to_class_case()
      );
    }
    w!("\n");
    for doc in doc.lines() {
      w!("/// {}\n", doc.trim());
    }
    w!("pub mod {} {{\n", mod_name);
    w!("  #[allow(unused_imports)]\n");
    w!("  use reg::prelude::*;\n\n");
    let base_address = u32::from_str_radix(
      &peripheral.base_address.trim_left_matches("0x"),
      16,
    )?;
    for register in &registers.register {
      let name = register.name.to_class_case();
      let address = base_address +
        u32::from_str_radix(
          &register.address_offset.trim_left_matches("0x"),
          16,
        )?;
      let mut hex_address = format!("{:08X}", address);
      hex_address.insert(4, '_');
      let doc = register
        .description
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .join("\n");
      w!("  reg! {{\n");
      w!("    [0x{}] u32\n", hex_address);
      w!("    #[doc = \"{}\"]\n", doc);
      w!("    {}\n", name);
      w!("    #[doc = \"{}\"]\n", doc);
      w!("    {}Value\n", name);
      w!("   ");
      if register.access != "write-only" {
        w!(" RReg {{}}");
      }
      if register.access != "read-only" {
        w!(" WReg {{}}");
      }
      if BIT_BAND.contains(address) {
        w!(" RegBitBand {{}}");
      }
      w!("\n");
      w!("  }}\n\n");
      if BIT_BAND.contains(address) {
        w!("  impl<T: RegFlavor> {}<T> {{\n", name);
        for field in &register.fields.field {
          let mut name = field.name.to_snake_case();
          let offset = field.bit_offset.parse::<u32>()?;
          let width = field.bit_width.parse::<u32>()?;
          if register.access != "read-only" && width == 1 {
            for doc in field.description.lines() {
              w!("    /// {}\n", doc.trim());
            }
            w!("    #[inline]\n");
            w!("    pub fn set_{}(&mut self, value: bool) {{\n", name);
            w!("      self.set_bit_band({}, value);\n", offset);
            w!("    }}\n\n");
          }
          if register.access != "write-only" && width == 1 {
            if RESERVED.is_match(&name) {
              name.insert(0, '_');
            }
            for doc in field.description.lines() {
              w!("    /// {}\n", doc.trim());
            }
            w!("    #[inline]\n");
            w!("    pub fn {}(&self) -> bool {{\n", name);
            w!("      self.bit_band({})\n", offset);
            w!("    }}\n\n");
          }
        }
        w!("  }}\n\n");
      }
      w!("  impl {}Value {{\n", name);
      for field in &register.fields.field {
        let mut name = field.name.to_snake_case();
        let offset = field.bit_offset.parse::<u32>()?;
        let width = field.bit_width.parse::<u32>()?;
        if register.access != "read-only" {
          for doc in field.description.lines() {
            w!("    /// {}\n", doc.trim());
          }
          w!("    #[inline]\n");
          if width == 1 {
            w!(
              "    pub fn set_{}(&mut self, value: bool) -> &mut Self {{\n",
              name
            );
            w!("      self.set_bit({}, value)\n", offset);
            w!("    }}\n\n");
          } else {
            w!(
              "    pub fn set_{}(&mut self, value: u32) -> &mut Self {{\n",
              name
            );
            w!("      self.set_bits({}, {}, value)\n", offset, width);
            w!("    }}\n\n");
          }
        }
        if register.access != "write-only" {
          if RESERVED.is_match(&name) {
            name.insert(0, '_');
          }
          for doc in field.description.lines() {
            w!("    /// {}\n", doc.trim());
          }
          w!("    #[inline]\n");
          if width == 1 {
            w!("    pub fn {}(&self) -> bool {{\n", name);
            w!("      self.bit({})\n", offset);
            w!("    }}\n\n");
          } else {
            w!("    pub fn {}(&self) -> u32 {{\n", name);
            w!("      self.bits({}, {})\n", offset, width);
            w!("    }}\n\n");
          }
        }
      }
      w!("  }}\n");
    }
    w!("}}\n\n");
  }
  Ok(())
}

fn vtable_generate(output: &mut File, irq_count: u32) -> Result<()> {
  macro_rules! w {
    ($($x:tt)*) => {
      output.write_all(format!($($x)*).as_bytes())?;
    };
  }
  w!("#[doc(hidden)]\n");
  w!("#[macro_export]\n");
  w!("macro_rules! vtable_struct_with_irq {{\n");
  w!("  () => {{\n");
  w!("    vtable_struct! {{\n");
  for i in 0..irq_count {
    w!("      irq{},\n", i);
  }
  w!("    }}\n");
  w!("  }}\n");
  w!("}}\n");
  w!("\n");
  w!("#[doc(hidden)]\n");
  w!("#[macro_export]\n");
  w!("macro_rules! vtable_default_with_irq {{\n");
  w!("  ($reset:ident) => {{\n");
  w!("    vtable_default! {{\n");
  w!("      $reset,\n");
  for i in 0..irq_count {
    w!("      irq{},\n", i);
  }
  w!("    }}\n");
  w!("  }}\n");
  w!("}}\n");
  Ok(())
}

fn svd_from_feature() -> Option<&'static str> {
  #[allow(unreachable_patterns)]
  match () {
    #[cfg(feature = "stm32f100")]
    () => Some("svd/STM32F100.svd"),
    #[cfg(feature = "stm32f101")]
    () => Some("svd/STM32F101.svd"),
    #[cfg(feature = "stm32f102")]
    () => Some("svd/STM32F102.svd"),
    #[cfg(feature = "stm32f103")]
    () => Some("svd/STM32F103.svd"),
    #[cfg(feature = "stm32f107")]
    () => Some("svd/STM32F107.svd"),
    #[cfg(feature = "stm32l4x1")]
    () => Some("svd/STM32L4x1.svd"),
    #[cfg(feature = "stm32l4x2")]
    () => Some("svd/STM32L4x2.svd"),
    #[cfg(feature = "stm32l4x3")]
    () => Some("svd/STM32L4x3.svd"),
    #[cfg(feature = "stm32l4x5")]
    () => Some("svd/STM32L4x5.svd"),
    #[cfg(feature = "stm32l4x6")]
    () => Some("svd/STM32L4x6.svd"),
    _ => None,
  }
}

fn irq_from_feature() -> u32 {
  #[allow(unreachable_patterns)]
  match () {
    #[cfg(any(feature = "stm32f100", feature = "stm32f101",
                feature = "stm32f102", feature = "stm32f103",
                feature = "stm32f107"))]
    () => 68,
    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
    () => 240,
    _ => 0,
  }
}
