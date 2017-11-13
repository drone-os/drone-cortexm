#![feature(decl_macro)]
#![feature(range_contains)]

#[macro_use]
extern crate error_chain;
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
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::ops::Range;
use std::path::Path;

const BIT_BAND: Range<u32> = 0x4000_0000..0x4010_0000;

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
  size: String,
  access: Option<String>,
  #[serde(rename = "resetValue")] reset_value: String,
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
  access: Option<String>,
}

quick_main!(run);

fn run() -> Result<()> {
  let out_dir = env::var("OUT_DIR")?;
  let out_dir = Path::new(&out_dir);
  let mut svd_out = File::create(out_dir.join("svd.rs"))?;
  if let Some(svd_file) = svd_from_feature() {
    svd_generate(&mut svd_out, &mut File::open(svd_file)?)?;
  }
  Ok(())
}

fn svd_generate(output: &mut File, input: &mut File) -> Result<()> {
  let mut xml = String::new();
  input.read_to_string(&mut xml)?;
  let device: Device = serde_xml_rs::deserialize(xml.as_bytes())?;
  macro w($($x:tt)*) {
    output.write_all(format!($($x)*).as_bytes())?;
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
    w!("reg_block! {{\n");
    for doc in doc.lines() {
      w!("  //! {}\n", doc.trim());
    }
    w!("  {}\n", peripheral.name);
    let base_address = u32::from_str_radix(
      &peripheral.base_address.trim_left_matches("0x"),
      16,
    )?;
    for register in &registers.register {
      let address = base_address
        + u32::from_str_radix(
          &register.address_offset.trim_left_matches("0x"),
          16,
        )?;
      let mut hex_address = format!("{:08X}", address);
      hex_address.insert(4, '_');
      let reset = u32::from_str_radix(
        &register
          .reset_value
          .trim_left_matches("0x")
          .trim_left_matches("0X"),
        16,
      )?;
      let mut hex_reset = format!("{:08X}", reset);
      hex_reset.insert(4, '_');
      w!("\n");
      w!("  reg! {{\n");
      for doc in register.description.lines() {
        w!("    //! {}\n", doc.trim());
      }
      w!("    {}\n", register.name);
      w!("    0x{}\n", hex_address);
      w!("    {}\n", register.size);
      w!("    0x{}\n", hex_reset);
      match register.access {
        Some(ref access) if access == "write-only" => {
          w!("    WReg WoReg");
        }
        Some(ref access) if access == "read-only" => {
          w!("    RReg RoReg");
        }
        Some(ref access) if access == "read-write" => {
          w!("    RReg WReg");
        }
        None => {
          w!("    RReg WReg");
        }
        Some(ref access) => {
          bail!(
            "Unknown register access `{}` for `{}->{}`",
            access,
            peripheral.name,
            register.name
          );
        }
      }
      if BIT_BAND.contains(address) {
        w!(" RegBitBand");
      }
      w!("\n");
      for field in &register.fields.field {
        for doc in field.description.lines() {
          w!("    /// {}\n", doc.trim());
        }
        let offset = field.bit_offset.parse::<u32>()?;
        let width = field.bit_width.parse::<u32>()?;
        w!("    {} {{ {} {}", field.name, offset, width);
        match field.access.as_ref().or(register.access.as_ref()) {
          Some(access) if access == "write-only" => {
            w!(" WRegField WoRegField");
          }
          Some(access) if access == "read-only" => {
            w!(" RRegField RoRegField");
          }
          Some(access) if access == "read-write" => {
            w!(" RRegField WRegField");
          }
          None => {
            w!(" RRegField WRegField");
          }
          Some(access) => {
            bail!(
              "Unknown field access `{}` for `{}->{}->{}`",
              access,
              peripheral.name,
              register.name,
              field.name
            );
          }
        }
        w!(" }}\n");
      }
      w!("  }}\n");
    }
    w!("}}\n\n");
  }
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
