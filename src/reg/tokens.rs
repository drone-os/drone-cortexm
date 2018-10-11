#![allow(clippy::doc_markdown)]

use drone_core::reg::tokens;
use reg::map::*;

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
  pub struct RegIdx;

  include!(concat!(env!("OUT_DIR"), "/svd_reg_tokens.rs"));

  ITM {
    /// Trace Privilege Register.
    TPR;
    /// Trace Control Register.
    TCR;
    /// ITM lock access register.
    LAR;
  }

  SCB {
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
    /// MemManage Status Register.
    MMFSR;
    /// BusFault Status Register.
    BFSR;
    /// UsageFault Status Register.
    UFSR;
    /// HardFault Status Register.
    HFSR;
    /// Debug Fault Status Register.
    DFSR;
    /// MemManage Fault Address Register.
    MMFAR;
    /// BusFault Address Register.
    BFAR;
    /// Debug Exception and Monitor Control Register.
    DEMCR;
  }

  STK {
    /// SysTick control and status register.
    CTRL;
    /// SysTick reload value register.
    LOAD;
    /// SysTick current value register.
    VAL;
    /// SysTick calibration value register.
    CALIB;
  }

  FPU {
    /// Coprocessor access control register.
    CPACR;
    /// Floating-point context control register.
    FPCCR;
    /// Floating-point context address register.
    FPCAR;
    /// Floating-point default status control register.
    FPDSCR;
  }

  MPU {
    /// Indicates how many regions the MPU support.
    TYPE;
    /// Enables the MPU, and when the MPU is enabled, controls whether the
    /// default memory map is enabled as a background region for privileged
    /// accesses, and whether the MPU is enabled for HardFaults, NMIs, and
    /// exception handlers when FAULTMASK is set to 1.
    CTRL;
    /// Selects the region currently accessed by MPU_RBAR and MPU_RASR.
    RNR;
    /// Holds the base address of the region identified by MPU_RNR. On a write,
    /// can also be used to update the base address of a specified region, in
    /// the range 0 to 15, updating MPU_RNR with the new region number.
    RBAR;
    /// Defines the size and access behavior of the region identified by
    /// MPU_RNR, and enables that region.
    RASR;
  }

  TPIU {
    /// Selected Pin Protocol Register.
    SPPR;
    /// Formatter and Flush Control Register.
    FFCR;
  }
}
