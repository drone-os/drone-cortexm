//! Floating point unit.

use crate::{map::periph::fpu::FpuPeriph, reg::prelude::*};

/// FPU driver.
pub struct Fpu {
    periph: FpuPeriph,
}

/// Acquires [`Fpu`].
#[macro_export]
macro_rules! drv_fpu {
    ($reg:ident) => {
        $crate::drv::fpu::Fpu::new($crate::periph_fpu!($reg))
    };
}

impl Fpu {
    /// Creates a new [`Fpu`].
    #[inline]
    pub fn new(periph: FpuPeriph) -> Self {
        Self { periph }
    }

    /// Releases the peripheral.
    #[inline]
    pub fn free(self) -> FpuPeriph {
        self.periph
    }
}

impl Fpu {
    /// Enables the FPU.
    pub fn enable(&self) {
        self.periph
            .fpu_cpacr
            .store(|r| r.write_cp10(0b11).write_cp11(0b11));
        unsafe {
            asm!("
                dsb
                isb
            "   :
                :
                :
                : "volatile"
            );
        }
    }
}
