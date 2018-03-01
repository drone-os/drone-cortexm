//! Drone threading system resources.

use drone_core::thr::ThrTokens;
use drv::nvic::Nvic;
use drv::prelude::*;
use reg::prelude::*;
use reg::scb;

/// `Thr` driver.
pub struct Thr(ThrRes);

/// `Thr` resource.
#[allow(missing_docs)]
pub struct ThrRes {
  pub nvic: Nvic,
  pub scb_ccr: scb::Ccr<Srt>,
}

/// Creates a new `Thr`.
#[macro_export]
macro_rules! drv_thr {
  ($reg:ident) => {
    $crate::drv::thr::Thr::from_res(
      $crate::drv::thr::ThrRes {
        nvic: drv_nvic!($reg),
        scb_ccr: $reg.scb_ccr,
      }
    )
  }
}

impl Driver for Thr {
  type Resource = ThrRes;

  #[inline(always)]
  fn from_res(res: ThrRes) -> Self {
    Thr(res)
  }

  #[inline(always)]
  fn into_res(self) -> ThrRes {
    self.0
  }
}

impl Resource for ThrRes {
  // FIXME https://github.com/rust-lang/rust/issues/47385
  type Source = Self;
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
