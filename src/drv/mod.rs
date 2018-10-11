//! Device drivers.

pub mod dma;
pub mod exti;
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
pub mod fpu;
pub mod gpio;
pub mod i2c;
pub mod i2c_dma_master_sess;
pub mod nvic;
pub mod spi;
pub mod thr;
pub mod timer;
