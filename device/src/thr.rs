//! Interrupt mappings.

pub use drone_stm32_core::thr::*;

/// Interrupt mappings.
#[allow(clippy::doc_markdown)]
pub mod int {
  pub use drone_stm32_core::thr::int::*;

  #[allow(unused_imports)]
  use drone_stm32_core::thr::int;
  #[allow(unused_imports)]
  use drone_stm32_core::thr::prelude::*;

  include!(concat!(env!("OUT_DIR"), "/svd_interrupts.rs"));
}
