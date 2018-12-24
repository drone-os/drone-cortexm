//! Core ARM Cortex-M register mappings.

#[path = "."]
mod inner {
  mod fpu;
  mod itm;
  mod mpu;
  mod scb;
  mod stk;
  mod tpiu;

  pub use self::{fpu::*, itm::*, mpu::*, scb::*, stk::*, tpiu::*};
}

use drone_core::reg;

reg::index! {
  pub macro cortex_m_reg_index;
  super::inner; map::reg;

  /// Instrumentation trace macrocell.
  pub mod ITM {
    TPR; TCR; LAR;
  }

  /// System control block.
  pub mod SCB {
    CPUID; ICSR; VTOR; AIRCR; SCR; CCR; SHPR1; SHPR2; SHPR3; SHCSR; MMFSR;
    BFSR; UFSR; HFSR; DFSR; MMFAR; BFAR; AFSR; DEMCR;
  }

  /// SysTick timer.
  pub mod STK {
    CTRL; LOAD; VAL; CALIB;
  }

  /// Floating point unit.
  pub mod FPU {
    CPACR; FPCCR; FPCAR; FPDSCR;
  }

  /// Memory protection unit.
  pub mod MPU {
    TYPE; CTRL; RNR; RBAR; RASR;
  }

  /// Trace port interface unit.
  pub mod TPIU {
    SPPR; FFCR;
  }
}
