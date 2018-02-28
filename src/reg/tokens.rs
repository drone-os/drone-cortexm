use drone_core::reg::tokens;
use reg::prelude::*;

tokens! {
  #[allow(missing_docs)]
  #[cfg_attr(feature = "stm32f100", doc = "Register tokens for STM32F100.")]
  #[cfg_attr(feature = "stm32f101", doc = "Register tokens for STM32F101.")]
  #[cfg_attr(feature = "stm32f102", doc = "Register tokens for STM32F102.")]
  #[cfg_attr(feature = "stm32f103", doc = "Register tokens for STM32F103.")]
  #[cfg_attr(feature = "stm32f107", doc = "Register tokens for STM32F107.")]
  #[cfg_attr(feature = "stm32l4x1", doc = "Register tokens for STM32L4x1.")]
  #[cfg_attr(feature = "stm32l4x2", doc = "Register tokens for STM32L4x2.")]
  #[cfg_attr(feature = "stm32l4x3", doc = "Register tokens for STM32L4x3.")]
  #[cfg_attr(feature = "stm32l4x5", doc = "Register tokens for STM32L4x5.")]
  #[cfg_attr(feature = "stm32l4x6", doc = "Register tokens for STM32L4x6.")]
  RegIdx;

  include!(concat!(env!("OUT_DIR"), "/svd_tokens.rs"));

  reg::ITM {
    /// Trace Privilege Register.
    TPR;
    /// Trace Control Register.
    TCR;
    /// ITM lock access register.
    LAR;
  }

  reg::SCB {
    /// Interrupt control and state register.
    ICSR;
    /// Application interrupt and reset control register.
    AIRCR;
    /// System control register.
    SCR;
    /// Configuration and control register.
    CCR;
    /// System handler priority register 1.
    SHPR1;
    /// System handler priority register 2.
    SHPR2;
    /// System handler priority register 3.
    SHPR3;
    /// System handler control and state register.
    SHCSR;
    /// Debug Exception and Monitor Control Register.
    DEMCR;
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

  reg::TPIU {
    /// Selected Pin Protocol Register.
    SPPR;
    /// Formatter and Flush Control Register.
    FFCR;
  }
}
