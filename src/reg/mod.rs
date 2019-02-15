//! Memory-mapped registers.

pub mod marker;
pub mod prelude;

mod atomic;
mod bit_band;

pub use self::{atomic::*, bit_band::*};
pub use drone_core::reg::*;
