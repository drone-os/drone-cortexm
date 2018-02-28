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
    mappings: &mut File,
    tokens: &mut File,
    irq: &mut File,
  ) -> Result<(), Error> {
    let mut irq_names = HashSet::new();
    for peripheral in self.peripherals.peripheral.values() {
      let (mapping_tokens, token_tokens, irq_tokens) =
        peripheral.to_tokens(&self.peripherals, &mut irq_names)?;
      mappings.write_all(mapping_tokens.to_string().as_bytes())?;
      tokens.write_all(token_tokens.to_string().as_bytes())?;
      irq.write_all(irq_tokens.to_string().as_bytes())?;
    }
    Ok(())
  }
}

impl Peripheral {
  fn to_tokens(
    &self,
    peripherals: &Peripherals,
    irq_names: &mut HashSet<String>,
  ) -> Result<(Tokens, Tokens, Tokens), Error> {
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
    let name = Ident::from(name.to_owned());
    let (mappings, tokens) = registers
      .as_ref()
      .or_else(|| parent.and_then(|x| x.registers.as_ref()))
      .ok_or_else(|| err_msg("Peripheral registers not found"))?
      .to_tokens(base_address);
    let interrupts = interrupt.to_tokens(irq_names);
    Ok((
      quote! {
        mappings! {
          #[doc = #description]
          #name;
          #(#mappings)*
        }
      },
      quote! {
        reg::#name {
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
  fn to_tokens(&self, irq_names: &mut HashSet<String>) -> Vec<Tokens>;
}

impl Interrupts for Vec<Interrupt> {
  fn to_tokens(&self, irq_names: &mut HashSet<String>) -> Vec<Tokens> {
    self
      .iter()
      .filter(|interrupt| irq_names.insert(interrupt.name.to_owned()))
      .map(|interrupt| {
        let &Interrupt {
          ref name,
          ref description,
          value,
        } = interrupt;
        let name = Ident::from(name.to_owned());
        let value = Lit::Int(value as u64, IntTy::Unsuffixed);
        quote! {
          interrupt! {
            #[doc = #description]
            pub trait #name: #value;
          }
        }
      })
      .collect()
  }
}

impl Registers {
  fn to_tokens(&self, base_address: u32) -> (Vec<Tokens>, Vec<Tokens>) {
    self
      .register
      .iter()
      .filter_map(|register| {
        let &Register {
          ref name,
          ref description,
          address_offset,
          size,
          access,
          reset_value,
          ref fields,
        } = register;
        let name = Ident::from(name.to_owned());
        let address = base_address + address_offset;
        let mut traits = Vec::new();
        match access {
          Some(Access::WriteOnly) => {
            traits.push(Ident::from("WReg"));
            traits.push(Ident::from("WoReg"));
          }
          Some(Access::ReadOnly) => {
            traits.push(Ident::from("RReg"));
            traits.push(Ident::from("RoReg"));
          }
          Some(Access::ReadWrite) | None => {
            traits.push(Ident::from("RReg"));
            traits.push(Ident::from("WReg"));
          }
        }
        if BIT_BAND.contains(address) {
          traits.push(Ident::from("RegBitBand"));
        }
        let address = Lit::Int(address as u64, IntTy::Unsuffixed);
        let size = Lit::Int(size as u64, IntTy::Unsuffixed);
        let reset_value = Lit::Int(reset_value as u64, IntTy::Unsuffixed);
        let fields = fields.as_ref()?.to_tokens(access);
        Some((
          quote! {
            #[doc = #description]
            #name {
              #address #size #reset_value
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
  fn to_tokens(&self, base_access: Option<Access>) -> Vec<Tokens> {
    self
      .field
      .iter()
      .map(|field| {
        let &Field {
          ref name,
          ref description,
          bit_offset,
          bit_width,
          access,
        } = field;
        let name = Ident::from(name.to_owned());
        let mut traits = Vec::new();
        match access.as_ref().or(base_access.as_ref()) {
          Some(&Access::WriteOnly) => {
            traits.push(Ident::from("WWRegField"));
            traits.push(Ident::from("WoWRegField"));
          }
          Some(&Access::ReadOnly) => {
            traits.push(Ident::from("RRRegField"));
            traits.push(Ident::from("RoRRegField"));
          }
          Some(&Access::ReadWrite) | None => {
            traits.push(Ident::from("RRRegField"));
            traits.push(Ident::from("WWRegField"));
          }
        }
        let bit_offset = Lit::Int(bit_offset as u64, IntTy::Unsuffixed);
        let bit_width = Lit::Int(bit_width as u64, IntTy::Unsuffixed);
        quote! {
          #[doc = #description]
          #name {
            #bit_offset #bit_width
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
  Ok(map)
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
