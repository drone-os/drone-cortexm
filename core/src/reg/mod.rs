//! Memory-mapped registers.

pub mod marker;
pub mod prelude;

mod atomic;
mod bit_band;
mod guard;

pub use self::atomic::*;
pub use self::bit_band::*;
pub use self::guard::{RegGuard, RegGuardCnt, RegGuardRes};

#[allow(clippy::doc_markdown)]
mod map {
  use drone_core::reg::map;
  use reg::prelude::*;

  include!(concat!(env!("OUT_DIR"), "/svd_reg_map.rs"));
}

pub use self::map::*;
