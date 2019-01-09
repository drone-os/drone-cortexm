//! Thread resource.

use crate::map;
use drone_core::res;

res::one! {
  /// Thread resource.
  pub struct Thr;
  map::reg; map::res::thr;

  SCB {
    CCR;
  }
}
