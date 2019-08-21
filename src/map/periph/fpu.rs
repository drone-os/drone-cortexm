//! Floating Point Unit.

use crate::map;
use drone_core::periph;

periph::singular! {
    /// Acquires FPU.
    pub macro periph_fpu;

    /// FPU.
    pub struct FpuPeriph;

    map::reg;
    crate::map::periph::fpu;

    FPU {
        CPACR;
        FPCCR;
        FPCAR;
        FPDSCR;
    }
}
