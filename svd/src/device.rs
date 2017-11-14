use errors::*;
use quote::Tokens;
use serde::de::{self, Deserialize, Deserializer};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::ops::Range;
use std::result;
use syn::{Ident, IntTy, Lit};

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
  #[serde(deserialize_with = "deserialize_hex")] base_address: u32,
  registers: Option<Registers>,
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
  #[serde(deserialize_with = "deserialize_hex")] address_offset: u32,
  #[serde(deserialize_with = "deserialize_hex")] size: u32,
  access: Option<Access>,
  #[serde(deserialize_with = "deserialize_hex")] reset_value: u32,
  fields: Fields,
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
#[derive(Deserialize)]
enum Access {
  WriteOnly,
  ReadOnly,
  ReadWrite,
}

impl Device {
  pub fn generate(self, output: &mut File) -> Result<()> {
    for peripheral in self.peripherals.peripheral.values() {
      let tokens = peripheral.to_tokens(&self.peripherals)?;
      output.write_all(tokens.as_str().as_bytes())?;
    }
    Ok(())
  }
}

impl Peripheral {
  fn to_tokens(&self, peripherals: &Peripherals) -> Result<Tokens> {
    let parent = if let Some(ref derived_from) = self.derived_from {
      Some(peripherals
        .peripheral
        .get(derived_from)
        .ok_or("Peripheral `derivedFrom` not found")?)
    } else {
      None
    };
    let peripheral_description = self
      .description
      .as_ref()
      .or_else(|| parent.and_then(|x| x.description.as_ref()))
      .ok_or("Peripheral description not found")?;
    let peripheral_name = Ident::new(self.name.to_owned());
    let registers = self
      .registers
      .as_ref()
      .or_else(|| parent.and_then(|x| x.registers.as_ref()))
      .ok_or("Peripheral registers not found")?
      .to_tokens(self);
    Ok(quote! {
      reg_block! {
        #![doc = #peripheral_description]
        #peripheral_name
        #(#registers)*
      }
    })
  }
}

impl Registers {
  fn to_tokens(&self, peripheral: &Peripheral) -> Vec<Tokens> {
    self
      .register
      .iter()
      .map(|register| {
        let description = &register.description;
        let name = Ident::new(register.name.to_owned());
        let address = peripheral.base_address + register.address_offset;
        let size = Lit::Int(register.size as u64, IntTy::Unsuffixed);
        let reset = Lit::Int(register.reset_value as u64, IntTy::Unsuffixed);
        let mut traits = Vec::new();
        match register.access {
          Some(Access::WriteOnly) => {
            traits.push(Ident::new("WReg"));
            traits.push(Ident::new("WoReg"));
          }
          Some(Access::ReadOnly) => {
            traits.push(Ident::new("RReg"));
            traits.push(Ident::new("RoReg"));
          }
          Some(Access::ReadWrite) | None => {
            traits.push(Ident::new("RReg"));
            traits.push(Ident::new("WReg"));
          }
        }
        if BIT_BAND.contains(address) {
          traits.push(Ident::new("RegBitBand"));
        }
        let address = Lit::Int(address as u64, IntTy::Unsuffixed);
        let fields = register.fields.to_tokens(register);
        quote! {
          reg! {
            #![doc = #description]
            #name #address #size #reset
            #(#traits)*
            #(#fields)*
          }
        }
      })
      .collect()
  }
}

impl Fields {
  fn to_tokens(&self, register: &Register) -> Vec<Tokens> {
    self
      .field
      .iter()
      .map(|field| {
        let description = &field.description;
        let name = Ident::new(field.name.to_owned());
        let offset = Lit::Int(field.bit_offset as u64, IntTy::Unsuffixed);
        let width = Lit::Int(field.bit_width as u64, IntTy::Unsuffixed);
        let mut traits = Vec::new();
        match field.access.as_ref().or(register.access.as_ref()) {
          Some(&Access::WriteOnly) => {
            traits.push(Ident::new("WRegField"));
            traits.push(Ident::new("WoRegField"));
          }
          Some(&Access::ReadOnly) => {
            traits.push(Ident::new("RRegField"));
            traits.push(Ident::new("RoRegField"));
          }
          Some(&Access::ReadWrite) | None => {
            traits.push(Ident::new("RRegField"));
            traits.push(Ident::new("WRegField"));
          }
        }
        quote! {
          #[doc = #description]
          #name {
            #offset #width
            #(#traits)*
          }
        }
      })
      .collect()
  }
}

fn deserialize_peripheral<'de, D>(
  deserializer: D,
) -> result::Result<BTreeMap<String, Peripheral>, D::Error>
where
  D: Deserializer<'de>,
{
  let mut map = BTreeMap::new();
  let vec: Vec<Peripheral> = Deserialize::deserialize(deserializer)?;
  for peripheral in vec {
    map.insert(peripheral.name.clone(), peripheral);
  }
  Ok((map))
}

fn deserialize_hex<'de, D>(deserializer: D) -> result::Result<u32, D::Error>
where
  D: Deserializer<'de>,
{
  let s: String = Deserialize::deserialize(deserializer)?;
  let s = s.trim_left_matches("0x").trim_left_matches("0X");
  u32::from_str_radix(s, 16).map_err(de::Error::custom)
}
