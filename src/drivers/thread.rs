//! Drone threading system resources.

use drivers::nvic::Nvic;
use drivers::prelude::*;
use drone_core::thread::ThdTokens;
use reg::prelude::*;
use reg::scb;

/// `Thread` driver.
pub struct Thread(ThreadRes);

/// `Thread` resource.
#[allow(missing_docs)]
pub struct ThreadRes {
  pub nvic: Nvic,
  pub scb_ccr: scb::Ccr<Srt>,
}

/// Creates a new `Thread`.
#[macro_export]
macro_rules! drv_thd {
  ($reg:ident) => {
    $crate::drivers::thread::Thread::from_res(
      $crate::drivers::thread::ThreadRes {
        nvic: drv_nvic!($reg),
        scb_ccr: $reg.scb_ccr,
      }
    )
  }
}

impl Driver for Thread {
  type Resource = ThreadRes;

  #[inline(always)]
  fn from_res(res: ThreadRes) -> Self {
    Thread(res)
  }

  #[inline(always)]
  fn into_res(self) -> ThreadRes {
    self.0
  }
}

impl Resource for ThreadRes {
  // FIXME https://github.com/rust-lang/rust/issues/47385
  type Input = Self;
}

impl Thread {
  /// Initialized the Drone threading system, and returns an instance of `T`.
  #[inline(always)]
  pub fn init<T: ThdTokens>(
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
