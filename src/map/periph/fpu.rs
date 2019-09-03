//! Floating Point Unit.

use drone_core::periph;

periph::singular! {
    #[doc(hidden)]
    pub macro periph_fpu_inner;

    /// FPU peripheral.
    pub struct FpuPeriph;

    crate::map::reg;
    crate::map::periph::fpu;

    FPU {
        CPACR;
        FPCCR;
        FPCAR;
        FPDSCR;
    }
}

// Workaround the `macro_expanded_macro_exports_accessed_by_absolute_paths`
// error.
#[doc(hidden)]
#[macro_export]
macro_rules! periph_fpu {
    ($($tt:tt)*) => {
        $crate::periph_fpu_inner!($($tt)*);
    };
}

/// Extracts FPU register tokens.
#[doc(inline)]
pub use crate::periph_fpu;
