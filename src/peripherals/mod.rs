//! Peripheral drivers.

pub mod dma;
pub mod gpio;
pub mod spi;
pub mod spi_dma;
pub mod timer;

#[doc(hidden)]
pub mod rt {
  pub use core::marker::PhantomData;
}
