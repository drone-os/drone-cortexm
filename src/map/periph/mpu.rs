//! Memory Protection Unit.

use crate::map;
use drone_core::periph;

periph::singular! {
    /// Acquires MPU.
    pub macro periph_mpu;

    /// MPU.
    pub struct MpuPeriph;

    map::reg;
    crate::map::periph::mpu;

    MPU {
        TYPE;
        CTRL;
        RNR;
        RBAR;
        RASR;
    }
}
