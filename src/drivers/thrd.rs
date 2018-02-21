//! Drone threading system resources.

use drivers::nvic::Nvic;
use drivers::prelude::*;
use drone_core::thread::ThreadTokens;
use reg::prelude::*;
use reg::scb;

/// `Thrd` driver.
pub struct Thrd(ThrdRes);

/// `Thrd` resource.
#[allow(missing_docs)]
pub struct ThrdRes {
  pub nvic: Nvic,
  pub scb_ccr: scb::Ccr<Srt>,
}

/// Creates a new `Thrd`.
#[macro_export]
macro_rules! drv_thrd {
  ($regs:ident) => {
    $crate::drivers::thrd::Thrd::from_res(
      $crate::drivers::thrd::ThrdRes {
        nvic: drv_nvic!($regs),
        scb_ccr: $regs.scb_ccr,
      }
    )
  }
}

impl Driver for Thrd {
  type Resource = ThrdRes;

  #[inline(always)]
  fn from_res(res: ThrdRes) -> Self {
    Thrd(res)
  }

  #[inline(always)]
  fn into_res(self) -> ThrdRes {
    self.0
  }
}

impl Resource for ThrdRes {
  // FIXME https://github.com/rust-lang/rust/issues/47385
  type Input = Self;
}

impl Thrd {
  /// Initialized the Drone threading system, and returns an instance of `T`.
  #[inline(always)]
  pub fn init<T: ThreadTokens>(
    self,
    scb_ccr_init: impl for<'a, 'b> FnOnce(&'b mut scb::ccr::Hold<'a, Srt>)
      -> &'b mut scb::ccr::Hold<'a, Srt>,
  ) -> T {
    self.0.scb_ccr.store(|r| scb_ccr_init(r).set_stkalign());
    unsafe { T::new() }
  }
}
