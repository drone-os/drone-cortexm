//! Memory-mapped registers.

pub use self::tokens::*;
pub use drone_stm32_core::reg::*;
pub use drone_stm32_device_pool_1::reg::*;
pub use drone_stm32_device_pool_10::reg::*;
pub use drone_stm32_device_pool_11::reg::*;
pub use drone_stm32_device_pool_12::reg::*;
pub use drone_stm32_device_pool_2::reg::*;
pub use drone_stm32_device_pool_3::reg::*;
pub use drone_stm32_device_pool_4::reg::*;
pub use drone_stm32_device_pool_5::reg::*;
pub use drone_stm32_device_pool_6::reg::*;
pub use drone_stm32_device_pool_7::reg::*;
pub use drone_stm32_device_pool_8::reg::*;
pub use drone_stm32_device_pool_9::reg::*;

#[allow(clippy::doc_markdown)]
mod tokens {
  use super::*;
  use drone_core::reg::tokens;

  include!(concat!(env!("OUT_DIR"), "/svd_reg_tokens.rs"));
}
