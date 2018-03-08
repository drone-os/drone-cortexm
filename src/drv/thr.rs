//! Drone threading system resources.

use drone_core::thr::ThrTokens;
use drv::nvic::Nvic;
use reg::prelude::*;
use reg::scb;

/// `Thr` driver.
#[derive(Driver)]
pub struct Thr(ThrRes);

/// `Thr` resource.
#[allow(missing_docs)]
#[derive(Resource)]
pub struct ThrRes {
  pub nvic: Nvic,
  pub scb_ccr: scb::Ccr<Srt>,
}

/// Creates a new `Thr`.
#[macro_export]
macro_rules! drv_thr {
  ($reg:ident) => {
    $crate::drv::thr::Thr::new(
      $crate::drv::thr::ThrRes {
        nvic: drv_nvic!($reg),
        scb_ccr: $reg.scb_ccr,
      }
    )
  }
}

impl Thr {
  /// Initialized the Drone threading system, and returns an instance of `T`.
  #[inline(always)]
  pub fn init<T: ThrTokens>(
    self,
    scb_ccr_init: impl for<'a, 'b> FnOnce(&'b mut scb::ccr::Hold<'a, Srt>)
      -> &'b mut scb::ccr::Hold<'a, Srt>,
  ) -> T {
    self
      .0
      .scb_ccr
      .store(|r| scb_ccr_init(r).set_stkalign().set_nonbasethrdena());
    unsafe { T::new() }
  }
}
