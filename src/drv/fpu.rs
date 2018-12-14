//! Floating point unit.

use map::res::fpu::FpuRes;
use reg::prelude::*;

/// FPU driver.
pub struct Fpu(FpuRes);

/// Creates a new `Fpu`.
#[macro_export]
macro_rules! drv_fpu {
  ($reg:ident) => {
    $crate::drv::fpu::Fpu::new(res_fpu!($reg))
  };
}

impl Fpu {
  /// Creates a new `Fpu`.
  #[inline(always)]
  pub fn new(fpu: FpuRes) -> Self {
    Fpu(fpu)
  }

  /// Releases the underlying registers.
  #[inline(always)]
  pub fn free(self) -> FpuRes {
    self.0
  }
}

impl Fpu {
  /// Enables the FPU.
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
