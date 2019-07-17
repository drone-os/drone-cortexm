//! Core ARM Cortex-M register mappings.

#[path = "reg"]
mod inner {
    mod dwt;
    mod fpu;
    mod itm;
    mod mpu;
    mod scb;
    mod stk;
    mod tpiu;

    pub use self::{dwt::*, fpu::*, itm::*, mpu::*, scb::*, stk::*, tpiu::*};
}

use drone_core::reg;

reg::unsafe_tokens! {
    /// Defines an index of core ARM Cortex-M register tokens.
    ///
    /// # Safety
    ///
    /// See [`::drone_core::reg::unsafe_tokens!`].
    pub macro unsafe_cortex_m_reg_tokens;
    super::inner; map::reg;

    /// Data watchpoint and trace.
    pub mod DWT {
        CYCCNT;
    }

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
        ACPR; SPPR; FFCR;
    }
}
