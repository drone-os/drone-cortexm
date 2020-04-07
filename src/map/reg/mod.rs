//! Core ARM Cortex-M register mappings.

#[path = "."]
mod inner {
    mod dwt;
    #[cfg(all(
        feature = "floating-point-unit",
        any(
            cortex_m_core = "cortex_m4f_r0p0",
            cortex_m_core = "cortex_m4f_r0p1",
            cortex_m_core = "cortex_m33f_r0p2",
            cortex_m_core = "cortex_m33f_r0p3",
            cortex_m_core = "cortex_m33f_r0p4"
        )
    ))]
    mod fpu;
    mod itm;
    mod mpu;
    mod scb;
    mod stk;
    mod tpiu;

    #[cfg(all(
        feature = "floating-point-unit",
        any(
            cortex_m_core = "cortex_m4f_r0p0",
            cortex_m_core = "cortex_m4f_r0p1",
            cortex_m_core = "cortex_m33f_r0p2",
            cortex_m_core = "cortex_m33f_r0p3",
            cortex_m_core = "cortex_m33f_r0p4"
        )
    ))]
    pub use self::fpu::*;
    pub use self::{dwt::*, itm::*, mpu::*, scb::*, stk::*, tpiu::*};
}

use drone_core::reg;

reg::tokens! {
    #[doc(hidden)]
    pub macro cortex_m_reg_tokens_inner;
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
    #[cfg(all(
        feature = "floating-point-unit",
        any(
            cortex_m_core = "cortex_m4f_r0p0",
            cortex_m_core = "cortex_m4f_r0p1",
            cortex_m_core = "cortex_m33f_r0p2",
            cortex_m_core = "cortex_m33f_r0p3",
            cortex_m_core = "cortex_m33f_r0p4"
        )
    ))]
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

// Workaround the `macro_expanded_macro_exports_accessed_by_absolute_paths`
// error.
#[doc(hidden)]
#[macro_export]
macro_rules! cortex_m_reg_tokens {
    ($($tt:tt)*) => {
        use $crate::cortex_m_reg_tokens_inner;
        cortex_m_reg_tokens_inner!($($tt)*);
    };
}
