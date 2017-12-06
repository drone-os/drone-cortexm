//! Memory-mapped registers.

pub mod prelude;

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
mod bit_band;
#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
mod map;
#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
mod shared;

pub use self::bit_band::*;
pub use self::map::*;
pub use self::shared::*;
