//! Floating Point Unit.

#![cfg_attr(feature = "std", allow(unreachable_code))]

use crate::{map::periph::fpu::FpuPeriph, reg::prelude::*};

/// FPU driver.
pub struct Fpu {
    periph: FpuPeriph,
}

impl Fpu {
    /// Creates a new driver from the peripheral.
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
        #[cfg(feature = "std")]
        return unimplemented!();
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
