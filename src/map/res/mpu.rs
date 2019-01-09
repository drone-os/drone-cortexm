//! MPU resource.

use crate::map;
use drone_core::res;

res::one! {
  /// MPU resource.
  pub struct Mpu;
  map::reg; map::res::mpu;

  MPU {
    TYPE;
    CTRL;
    RNR;
    RBAR;
    RASR;
  }
}
