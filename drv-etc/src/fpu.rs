//! Floating point unit.

use drone_stm32_device::reg::prelude::*;
use drone_stm32_device::reg::{fpu, fpu_cpacr};

/// FPU driver.
#[derive(Driver)]
pub struct Fpu(FpuRes);

/// FPU resource.
#[allow(missing_docs)]
#[derive(Resource)]
pub struct FpuRes {
  pub fpu_cpacr: fpu_cpacr::Cpacr<Srt>,
  pub fpu_fpccr: fpu::Fpccr<Srt>,
  pub fpu_fpcar: fpu::Fpcar<Srt>,
  pub fpu_fpscr: fpu::Fpscr<Srt>,
}

/// Creates a new `Fpu`.
#[macro_export]
macro_rules! drv_fpu {
  ($reg:ident) => {
    <$crate::fpu::Fpu as ::drone_core::drv::Driver>::new($crate::fpu::FpuRes {
      fpu_cpacr: $reg.fpu_cpacr_cpacr,
      fpu_fpccr: $reg.fpu_fpccr,
      fpu_fpcar: $reg.fpu_fpcar,
      fpu_fpscr: $reg.fpu_fpscr,
    })
  };
}

#[allow(missing_docs)]
impl Fpu {
  #[inline(always)]
  pub fn fpu_cpacr(&self) -> &fpu_cpacr::Cpacr<Srt> {
    &self.0.fpu_cpacr
  }

  #[inline(always)]
  pub fn fpu_fpccr(&self) -> &fpu::Fpccr<Srt> {
    &self.0.fpu_fpccr
  }

  #[inline(always)]
  pub fn fpu_fpcar(&self) -> &fpu::Fpcar<Srt> {
    &self.0.fpu_fpcar
  }

  #[inline(always)]
  pub fn fpu_fpscr(&self) -> &fpu::Fpscr<Srt> {
    &self.0.fpu_fpscr
  }
}

impl Fpu {
  /// Enables FPU.
  pub fn enable(&self) {
    self.0.fpu_cpacr.store(|r| r.write_cp(0b1111));
    unsafe {
      asm!("
        dsb
        isb
      " :
        :
        :
        : "volatile");
    }
  }
}
