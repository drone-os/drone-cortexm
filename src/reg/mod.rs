//! Memory-mapped registers.

pub mod prelude;

mod bindings;
mod bit_band;
mod shared;

pub use self::bindings::*;
pub use self::bit_band::*;
pub use self::shared::*;
pub use drone::reg::bind;
