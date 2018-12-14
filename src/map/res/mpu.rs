//! MPU resource.

use drone_core::res;
use map;

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
