//! Thread resource.

use drone_core::res;
use map;

res::one! {
  /// Thread resource.
  pub struct Thr;
  map::reg; map::res::thr;

  SCB {
    CCR;
  }
}
