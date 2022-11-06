//! Memory Protection Unit.

use drone_core::periph;

periph::singular! {
    #[doc(hidden)]
    pub macro periph_mpu_inner;

    /// MPU peripheral.
    pub struct MpuPeriph;

    crate::map::reg;
    crate::map::periph::mpu;

    MPU {
        TYPE;
        CTRL;
        RNR;
        RBAR;
        RASR;
    }
}

// Workaround the `macro_expanded_macro_exports_accessed_by_absolute_paths`
// error.
#[doc(hidden)]
#[macro_export]
macro_rules! periph_mpu {
    ($($tt:tt)*) => {
        $crate::periph_mpu_inner!($($tt)*);
    };
}

/// Extracts MPU register tokens.
#[doc(inline)]
pub use crate::periph_mpu;
