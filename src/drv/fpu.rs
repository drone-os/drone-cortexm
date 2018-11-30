//! Floating point unit.

use map::reg::fpu;
use reg::prelude::*;

/// FPU driver.
pub struct Fpu(FpuRes);

/// FPU resource.
#[allow(missing_docs)]
pub struct FpuRes {
  pub fpu_cpacr: fpu::Cpacr<Srt>,
  pub fpu_fpccr: fpu::Fpccr<Srt>,
  pub fpu_fpcar: fpu::Fpcar<Srt>,
  pub fpu_fpdscr: fpu::Fpdscr<Srt>,
}

/// Creates a new `Fpu`.
#[macro_export]
macro_rules! drv_fpu {
  ($reg:ident) => {
    $crate::drv::fpu::Fpu::new($crate::drv::fpu::FpuRes {
      fpu_cpacr: $reg.fpu_cpacr,
      fpu_fpccr: $reg.fpu_fpccr,
      fpu_fpcar: $reg.fpu_fpcar,
      fpu_fpdscr: $reg.fpu_fpdscr,
    })
  };
}

#[allow(missing_docs)]
impl Fpu {
  #[inline(always)]
  pub fn fpu_cpacr(&self) -> &fpu::Cpacr<Srt> {
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
  pub fn fpu_fpdscr(&self) -> &fpu::Fpdscr<Srt> {
    &self.0.fpu_fpdscr
  }
}

impl Fpu {
  /// Creates a new `Fpu`.
  #[inline(always)]
  pub fn new(res: FpuRes) -> Self {
    Fpu(res)
  }

  /// Releases the underlying resources.
  #[inline(always)]
  pub fn free(self) -> FpuRes {
    self.0
  }
}

impl Fpu {
  /// Enables FPU.
  pub fn enable(&self) {
    self
      .0
      .fpu_cpacr
      .store(|r| r.write_cp10(0b11).write_cp11(0b11));
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
