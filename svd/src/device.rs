use failure::{err_msg, Error};
use serde::de::{self, Deserialize, Deserializer};
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::ops::Range;

const BIT_BAND: Range<u32> = 0x4000_0000..0x4010_0000;

#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
pub struct Device {
  peripherals: Peripherals,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
struct Peripherals {
  #[serde(deserialize_with = "deserialize_peripheral")]
  peripheral: BTreeMap<String, Peripheral>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
struct Peripheral {
  derived_from: Option<String>,
  name: String,
  description: Option<String>,
  #[serde(deserialize_with = "deserialize_hex")]
  base_address: u32,
  #[serde(default)]
  interrupt: Vec<Interrupt>,
  registers: Option<Registers>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
struct Interrupt {
  name: String,
  description: String,
  #[serde(deserialize_with = "deserialize_dec")]
  value: u32,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
struct Registers {
  register: Vec<Register>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
struct Register {
  name: String,
  description: String,
  #[serde(deserialize_with = "deserialize_hex")]
  address_offset: u32,
  #[serde(deserialize_with = "deserialize_hex")]
  size: u32,
  access: Option<Access>,
  #[serde(deserialize_with = "deserialize_hex")]
  reset_value: u32,
  fields: Option<Fields>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
struct Fields {
  field: Vec<Field>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
struct Field {
  name: String,
  description: String,
  bit_offset: usize,
  bit_width: usize,
  access: Option<Access>,
}

#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Clone, Copy)]
enum Access {
  WriteOnly,
  ReadOnly,
  ReadWrite,
}

impl Device {
  pub fn generate(
    self,
    reg_map: &mut File,
    reg_tokens: &mut File,
    interrupts: &mut File,
  ) -> Result<(), Error> {
    let mut int_names = HashSet::new();
    for peripheral in self.peripherals.peripheral.values() {
      peripheral.generate(
        &self.peripherals,
        &mut int_names,
        reg_map,
        reg_tokens,
        interrupts,
      )?;
    }
    Ok(())
  }
}

impl Peripheral {
  fn generate(
    &self,
    peripherals: &Peripherals,
    int_names: &mut HashSet<String>,
    reg_map: &mut File,
    reg_tokens: &mut File,
    interrupts: &mut File,
  ) -> Result<(), Error> {
    let &Peripheral {
      ref derived_from,
      ref name,
      ref description,
      base_address,
      ref interrupt,
      ref registers,
    } = self;
    let parent = if let &Some(ref derived_from) = derived_from {
      Some(peripherals
        .peripheral
        .get(derived_from)
        .ok_or_else(|| err_msg("Peripheral `derivedFrom` not found"))?)
    } else {
      None
    };
    let description = description
      .as_ref()
      .or_else(|| parent.and_then(|x| x.description.as_ref()))
      .ok_or_else(|| err_msg("Peripheral description not found"))?;
    writeln!(reg_map, "map! {{")?;
    for line in description.lines() {
      writeln!(reg_map, "  /// {}", line.trim())?;
    }
    writeln!(reg_map, "  pub mod {};", name)?;
    writeln!(reg_tokens, "  {} {{", name)?;
    registers
      .as_ref()
      .or_else(|| parent.and_then(|x| x.registers.as_ref()))
      .ok_or_else(|| err_msg("Peripheral registers not found"))?
      .generate(base_address, reg_map, reg_tokens)?;
    interrupt.generate(int_names, interrupts)?;
    writeln!(reg_tokens, "  }}")?;
    writeln!(reg_map, "}}")?;
    Ok(())
  }
}

trait Interrupts {
  fn generate(
    &self,
    int_names: &mut HashSet<String>,
    interrupts: &mut File,
  ) -> Result<(), Error>;
}

impl Interrupts for Vec<Interrupt> {
  fn generate(
    &self,
    int_names: &mut HashSet<String>,
    interrupts: &mut File,
  ) -> Result<(), Error> {
    for interrupt in self {
      if int_names.insert(interrupt.name.to_owned()) {
        let &Interrupt {
          ref name,
          ref description,
          value,
        } = interrupt;
        writeln!(interrupts, "int! {{")?;
        for line in description.lines() {
          writeln!(interrupts, "  /// {}", line.trim())?;
        }
        writeln!(interrupts, "  pub trait {}: {};", name, value)?;
        writeln!(interrupts, "}}")?;
      }
    }
    Ok(())
  }
}

impl Registers {
  fn generate(
    &self,
    base_address: u32,
    reg_map: &mut File,
    reg_tokens: &mut File,
  ) -> Result<(), Error> {
    for register in &self.register {
      let &Register {
        ref name,
        ref description,
        address_offset,
        size,
        access,
        reset_value,
        ref fields,
      } = register;
      let address = base_address + address_offset;
      for line in description.lines() {
        writeln!(reg_map, "  /// {}", line.trim())?;
      }
      writeln!(reg_map, "  {} {{", name)?;
      writeln!(
        reg_map,
        "    0x{:04X}_{:04X} {} 0x{:04X}_{:04X}",
        address >> 16,
        address & 0xFFFF,
        size,
        reset_value >> 16,
        reset_value & 0xFFFF,
      )?;
      write!(reg_map, "   ")?;
      match access {
        Some(Access::WriteOnly) => {
          write!(reg_map, " WReg")?;
          write!(reg_map, " WoReg")?;
        }
        Some(Access::ReadOnly) => {
          write!(reg_map, " RReg")?;
          write!(reg_map, " RoReg")?;
        }
        Some(Access::ReadWrite) | None => {
          write!(reg_map, " RReg")?;
          write!(reg_map, " WReg")?;
        }
      }
      if BIT_BAND.contains(&address) {
        write!(reg_map, " RegBitBand")?;
      }
      writeln!(reg_map, ";")?;
      if let &Some(ref fields) = fields {
        fields.generate(access, reg_map)?;
      }
      writeln!(reg_map, "  }}")?;
      for line in description.lines() {
        writeln!(reg_tokens, "    /// {}", line.trim())?;
      }
      writeln!(reg_tokens, "    {};", name)?;
    }
    Ok(())
  }
}

impl Fields {
  fn generate(
    &self,
    base_access: Option<Access>,
    reg_map: &mut File,
  ) -> Result<(), Error> {
    for field in &self.field {
      let &Field {
        ref name,
        ref description,
        bit_offset,
        bit_width,
        access,
      } = field;
      for line in description.lines() {
        writeln!(reg_map, "    /// {}", line.trim())?;
      }
      write!(
        reg_map,
        "    {} {{ {} {}",
        name, bit_offset, bit_width
      )?;
      match access.as_ref().or(base_access.as_ref()) {
        Some(&Access::WriteOnly) => {
          write!(reg_map, " WWRegField")?;
          write!(reg_map, " WoWRegField")?;
        }
        Some(&Access::ReadOnly) => {
          write!(reg_map, " RRRegField")?;
          write!(reg_map, " RoRRegField")?;
        }
        Some(&Access::ReadWrite) | None => {
          write!(reg_map, " RRRegField")?;
          write!(reg_map, " WWRegField")?;
        }
      }
      writeln!(reg_map, " }}")?;
    }
    Ok(())
  }
}

fn deserialize_peripheral<'de, D>(
  deserializer: D,
) -> Result<BTreeMap<String, Peripheral>, D::Error>
where
  D: Deserializer<'de>,
{
  let mut map = BTreeMap::new();
  for peripheral in Vec::<Peripheral>::deserialize(deserializer)? {
    map.insert(peripheral.name.clone(), peripheral);
  }
  Ok(map)
}

fn deserialize_hex<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  let s = s.trim_left_matches("0x")
    .trim_left_matches("0X");
  u32::from_str_radix(s, 16).map_err(de::Error::custom)
}

fn deserialize_dec<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  u32::from_str_radix(&s, 10).map_err(de::Error::custom)
}
