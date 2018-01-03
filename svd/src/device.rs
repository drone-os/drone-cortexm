use failure::{err_msg, Error};
use quote::Tokens;
use serde::de::{self, Deserialize, Deserializer};
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::ops::Range;
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
  #[serde(default)] interrupt: Vec<Interrupt>,
  registers: Option<Registers>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize)]
struct Interrupt {
  name: String,
  description: String,
  #[serde(deserialize_with = "deserialize_dec")] value: u32,
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
#[derive(Deserialize)]
enum Access {
  WriteOnly,
  ReadOnly,
  ReadWrite,
}

impl Device {
  pub fn generate(
    self,
    mappings: &mut File,
    tokens: &mut File,
    interrupts: &mut File,
  ) -> Result<(), Error> {
    let mut interrupt_names = HashSet::new();
    for peripheral in self.peripherals.peripheral.values() {
      let (mapping_tokens, token_tokens, interrupt_tokens) =
        peripheral.to_tokens(&self.peripherals, &mut interrupt_names)?;
      mappings.write_all(mapping_tokens.as_str().as_bytes())?;
      tokens.write_all(token_tokens.as_str().as_bytes())?;
      interrupts.write_all(interrupt_tokens.as_str().as_bytes())?;
    }
    Ok(())
  }
}

impl Peripheral {
  fn to_tokens(
    &self,
    peripherals: &Peripherals,
    interrupt_names: &mut HashSet<String>,
  ) -> Result<(Tokens, Tokens, Tokens), Error> {
    let parent = if let Some(ref derived_from) = self.derived_from {
      Some(peripherals
        .peripheral
        .get(derived_from)
        .ok_or_else(|| err_msg("Peripheral `derivedFrom` not found"))?)
    } else {
      None
    };
    let peripheral_description = self
      .description
      .as_ref()
      .or_else(|| parent.and_then(|x| x.description.as_ref()))
      .ok_or_else(|| err_msg("Peripheral description not found"))?;
    let peripheral_name = Ident::new(self.name.to_owned());
    let (mappings, tokens) = self
      .registers
      .as_ref()
      .or_else(|| parent.and_then(|x| x.registers.as_ref()))
      .ok_or_else(|| err_msg("Peripheral registers not found"))?
      .to_tokens(self);
    let interrupts = self.interrupt.to_tokens(interrupt_names);
    Ok((
      quote! {
        mappings! {
          #[doc = #peripheral_description]
          #peripheral_name;
          #(#mappings)*
        }
      },
      quote! {
        reg::#peripheral_name {
          #(#tokens)*
        }
      },
      quote! {
        #(#interrupts)*
      },
    ))
  }
}

trait Interrupts {
  fn to_tokens(&self, interrupt_names: &mut HashSet<String>) -> Vec<Tokens>;
}

impl Interrupts for Vec<Interrupt> {
  fn to_tokens(&self, interrupt_names: &mut HashSet<String>) -> Vec<Tokens> {
    self
      .iter()
      .filter(|interrupt| interrupt_names.insert(interrupt.name.to_owned()))
      .map(|interrupt| {
        let description = &interrupt.description;
        let name = Ident::new(interrupt.name.to_owned());
        let value = Lit::Int(interrupt.value as u64, IntTy::Unsuffixed);
        quote! {
          interrupt! {
            #[doc = #description]
            #name;
            #value;
          }
        }
      })
      .collect()
  }
}

impl Registers {
  fn to_tokens(&self, peripheral: &Peripheral) -> (Vec<Tokens>, Vec<Tokens>) {
    self
      .register
      .iter()
      .filter_map(|register| {
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
        let fields = register.fields.as_ref()?.to_tokens(register);
        Some((
          quote! {
            #[doc = #description]
            #name {
              #address #size #reset
              #(#traits)*;
              #(#fields)*
            }
          },
          quote! {
            #[doc = #description]
            #name;
          },
        ))
      })
      .unzip()
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
) -> Result<BTreeMap<String, Peripheral>, D::Error>
where
  D: Deserializer<'de>,
{
  let mut map = BTreeMap::new();
  for peripheral in Vec::<Peripheral>::deserialize(deserializer)? {
    map.insert(peripheral.name.clone(), peripheral);
  }
  Ok((map))
}

fn deserialize_hex<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  let s = s.trim_left_matches("0x").trim_left_matches("0X");
  u32::from_str_radix(s, 16).map_err(de::Error::custom)
}

fn deserialize_dec<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  u32::from_str_radix(&s, 10).map_err(de::Error::custom)
}
