//! Memory Protection Unit.

use drone_core::periph;

periph::singular! {
    /// Extracts MPU register tokens.
    pub macro periph_mpu;

    /// MPU peripheral.
    pub struct Mpu;

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
