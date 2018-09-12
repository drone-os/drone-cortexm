//! Drone threading system resources.

use drone_core::thr::ThrTokens;
use drv::nvic::Nvic;
use reg::prelude::*;
use reg::{mpu, scb};

static MPU_RESET_TABLE: [u32; 16] = [
  rbar_reset(0),
  0,
  rbar_reset(1),
  0,
  rbar_reset(2),
  0,
  rbar_reset(3),
  0,
  rbar_reset(4),
  0,
  rbar_reset(5),
  0,
  rbar_reset(6),
  0,
  rbar_reset(7),
  0,
];

/// `Thr` driver.
#[derive(Driver)]
pub struct Thr(ThrRes);

/// `Thr` resource.
#[allow(missing_docs)]
#[derive(Resource)]
pub struct ThrRes {
  pub nvic: Nvic,
  pub mpu_type: mpu::Type<Srt>,
  pub mpu_ctrl: mpu::Ctrl<Srt>,
  pub mpu_rnr: mpu::Rnr<Srt>,
  pub mpu_rbar: mpu::Rbar<Srt>,
  pub mpu_rasr: mpu::Rasr<Srt>,
  pub scb_ccr: scb::Ccr<Srt>,
}

/// Creates a new `Thr`.
#[macro_export]
macro_rules! drv_thr {
  ($reg:ident) => {
    $crate::drv::thr::Thr::new($crate::drv::thr::ThrRes {
      nvic: drv_nvic!($reg),
      mpu_type: $reg.mpu_type,
      mpu_ctrl: $reg.mpu_ctrl,
      mpu_rnr: $reg.mpu_rnr,
      mpu_rbar: $reg.mpu_rbar,
      mpu_rasr: $reg.mpu_rasr,
      scb_ccr: $reg.scb_ccr,
    })
  };
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
    unsafe {
      self.mpu_reset();
      T::new()
    }
  }

  unsafe fn mpu_reset(&self) {
    if self.0.mpu_type.load().dregion() == 0 {
      return;
    }
    self.0.mpu_ctrl.reset();
    let mut _table_ptr = &MPU_RESET_TABLE;
    asm!("
      ldmia $0!, {r5-r12}
      stmia $1, {r5-r12}
      ldmia $0!, {r5-r12}
      stmia $1, {r5-r12}
    " : "+&rm"(_table_ptr)
      : "r"(mpu::Rbar::<Srt>::ADDRESS)
      : "r5", "r6", "r7", "r8", "r9", "r10", "r11", "r12"
      : "volatile");
  }
}

#[allow(clippy::cast_lossless)]
const fn rbar_reset(region: u8) -> u32 {
  1 << 4 | region as u32 & 0b1111
}
