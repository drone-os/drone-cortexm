//! SysTick timer.

use drone_core::periph;

periph::singular! {
    #[doc(hidden)]
    pub macro periph_sys_tick_inner;

    /// SysTick peripheral.
    pub struct SysTickPeriph;

    crate::map::reg;
    crate::map::periph::sys_tick;

    SCB {
        ICSR {
            PENDSTCLR;
            PENDSTSET;
        }
    }

    STK {
        CTRL;
        LOAD;
        VAL;
    }
}

// Workaround the `macro_expanded_macro_exports_accessed_by_absolute_paths`
// error.
#[doc(hidden)]
#[macro_export]
macro_rules! periph_sys_tick {
    ($($tt:tt)*) => {
        $crate::periph_sys_tick_inner!($($tt)*);
    };
}

/// Extracts SysTick register tokens.
#[doc(inline)]
pub use crate::periph_sys_tick;
