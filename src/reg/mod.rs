//! Memory-mapped registers.

pub mod map;
pub mod marker;
pub mod prelude;

mod atomic;
mod bit_band;
mod guard;

pub use self::atomic::*;
pub use self::bit_band::*;
pub use self::guard::{RegGuard, RegGuardCnt, RegGuardRes};
