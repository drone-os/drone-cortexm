//! FPU resource.

use crate::map;
use drone_core::res;

res::one! {
  /// FPU resource.
  pub struct Fpu;
  map::reg; map::res::fpu;

  FPU {
    CPACR;
    FPCCR;
    FPCAR;
    FPDSCR;
  }
}
