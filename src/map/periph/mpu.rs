//! Memory Protection Unit.

use crate::map;
use drone_core::periph;

periph::one! {
  /// Acquires MPU.
  pub macro periph_mpu;

  /// MPU.
  pub struct MpuPeriph;

  map::reg; map::periph::mpu;

  MPU {
    TYPE;
    CTRL;
    RNR;
    RBAR;
    RASR;
  }
}
