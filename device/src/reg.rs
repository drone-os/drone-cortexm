//! Memory-mapped registers.

pub use self::tokens::*;
pub use drone_stm32_core::reg::*;
pub use drone_stm32_device_0::reg::*;
pub use drone_stm32_device_1::reg::*;
pub use drone_stm32_device_2::reg::*;
pub use drone_stm32_device_3::reg::*;
pub use drone_stm32_device_4::reg::*;
pub use drone_stm32_device_5::reg::*;

#[allow(clippy::doc_markdown)]
mod tokens {
  use super::*;
  use drone_core::reg::tokens;

  include!(concat!(env!("OUT_DIR"), "/svd_reg_tokens.rs"));
}
