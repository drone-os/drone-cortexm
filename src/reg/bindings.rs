use drone::reg::bindings;
use reg::prelude::*;

bindings! {
  #[allow(missing_docs)]
  #[cfg_attr(feature = "stm32f100", doc = "Register bindings for STM32F100.")]
  #[cfg_attr(feature = "stm32f101", doc = "Register bindings for STM32F101.")]
  #[cfg_attr(feature = "stm32f102", doc = "Register bindings for STM32F102.")]
  #[cfg_attr(feature = "stm32f103", doc = "Register bindings for STM32F103.")]
  #[cfg_attr(feature = "stm32f107", doc = "Register bindings for STM32F107.")]
  #[cfg_attr(feature = "stm32l4x1", doc = "Register bindings for STM32L4x1.")]
  #[cfg_attr(feature = "stm32l4x2", doc = "Register bindings for STM32L4x2.")]
  #[cfg_attr(feature = "stm32l4x3", doc = "Register bindings for STM32L4x3.")]
  #[cfg_attr(feature = "stm32l4x5", doc = "Register bindings for STM32L4x5.")]
  #[cfg_attr(feature = "stm32l4x6", doc = "Register bindings for STM32L4x6.")]
  RegIndex;

  include!(concat!(env!("OUT_DIR"), "/svd_bindings.rs"));

  reg::SCB {
    /// System control register.
    SCR;
  }

  reg::STK {
    /// SysTick control and status register.
    CTRL;
    /// SysTick reload value register.
    LOAD;
    /// SysTick current value register.
    VAL;
    /// SysTick calibration value register.
    CALIB;
  }
}
