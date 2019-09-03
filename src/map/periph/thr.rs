//! Registers for Drone threads.

use drone_core::periph;

periph::singular! {
    #[doc(hidden)]
    pub macro periph_thr_inner;

    /// Registers for Drone threads.
    pub struct ThrPeriph;

    crate::map::reg;
    crate::map::periph::thr;

    SCB {
        CCR;
    }
}

// Workaround the `macro_expanded_macro_exports_accessed_by_absolute_paths`
// error.
#[doc(hidden)]
#[macro_export]
macro_rules! periph_thr {
    ($($tt:tt)*) => {
        $crate::periph_thr_inner!($($tt)*);
    };
}

/// Extracts Drone thread register tokens.
#[doc(inline)]
pub use crate::periph_thr;
