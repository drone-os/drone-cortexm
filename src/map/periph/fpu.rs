//! Floating Point Unit.

use drone_core::periph;

periph::singular! {
    /// Extracts FPU register tokens.
    pub macro periph_fpu;

    /// FPU peripheral.
    pub struct Fpu;

    crate::map::reg;
    crate::map::periph::fpu;

    FPU {
        CPACR;
        FPCCR;
        FPCAR;
        FPDSCR;
    }
}
