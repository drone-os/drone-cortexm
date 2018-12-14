//! FPU resource.

use drone_core::res;
use map;

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
