//! Threading support resources.

use crate::map;
use drone_core::periph;

periph::one! {
  /// Acquires threading resources.
  pub macro periph_thr;

  /// Threading resources.
  pub struct ThrPeriph;

  map::reg; map::periph::thr;

  SCB {
    CCR;
  }
}