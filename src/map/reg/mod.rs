//! Core ARM Cortex-M register mappings.

#[path = "."]
mod inner {
    mod dwt;
    #[cfg(feature = "floating-point-unit")]
    mod fpu;
    mod itm;
    #[cfg(feature = "memory-protection-unit")]
    mod mpu;
    mod scb;
    mod stk;
    mod tpiu;

    #[cfg(feature = "floating-point-unit")]
    pub use self::fpu::*;
    #[cfg(feature = "memory-protection-unit")]
    pub use self::mpu::*;
    pub use self::{dwt::*, itm::*, scb::*, stk::*, tpiu::*};
}

use drone_core::reg;

reg::tokens! {
    #[doc(hidden)]
    pub macro cortexm_reg_tokens_inner;
    super::inner;
    crate::map::reg;

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
    #[cfg(feature = "floating-point-unit")]
    pub mod FPU {
        CPACR; FPCCR; FPCAR; FPDSCR;
    }

    /// Memory protection unit.
    #[cfg(feature = "memory-protection-unit")]
    pub mod MPU {
        TYPE; CTRL; RNR; RBAR; RASR;
    }

    /// Trace port interface unit.
    pub mod TPIU {
        ACPR; SPPR; FFCR;
    }
}

// Workaround the `macro_expanded_macro_exports_accessed_by_absolute_paths`
// error.
#[doc(hidden)]
#[macro_export]
macro_rules! cortexm_reg_tokens {
    ($($tt:tt)*) => {
        use $crate::cortexm_reg_tokens_inner;
        cortexm_reg_tokens_inner!($($tt)*);
    };
}
