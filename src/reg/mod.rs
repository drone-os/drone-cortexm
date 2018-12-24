//! Memory-mapped registers.

pub mod marker;
pub mod prelude;

mod atomic;
mod bit_band;
mod guard;

pub use self::{
  atomic::*,
  bit_band::*,
  guard::{RegGuard, RegGuardCnt, RegGuardRes},
};
