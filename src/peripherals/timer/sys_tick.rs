//! SysTick timer.

pub use self::bind as Driver;

use peripherals;
use reg::prelude::*;
use reg::stk;

/// SysTick timer driver.
pub struct Driver {
  atoms: Atoms,
}

/// SysTick timer atoms.
#[allow(missing_docs)]
pub struct Atoms {
  pub stk_calib: stk::Calib<Sr>,
  pub stk_ctrl: stk::Ctrl<Sr>,
  pub stk_load: stk::Load<Sr>,
  pub stk_val: stk::Val<Sr>,
}

impl peripherals::Driver for Driver {
  type Atoms = Atoms;

  #[inline(always)]
  fn into_atoms(self) -> Self::Atoms {
    self.atoms
  }
}

impl peripherals::Atoms for Atoms {
  type Driver = Driver;

  #[inline(always)]
  fn into_driver(self) -> Self::Driver {
    Driver { atoms: self }
  }
}

impl Driver {
  /// Schedules SysTick event.
  #[inline]
  pub fn schedule(&self, duration: u32) {
    self.atoms.stk_load.reset(|r| r.write_reload(duration));
    self.atoms.stk_val.reset(|r| r.write_current(0));
    self.atoms.stk_ctrl.reset(|r| r.set_enable().set_tickint());
  }
}

/// Binds SysTick timer.
pub macro bind() {
  {
    use $crate::peripherals::prelude::*;
    use $crate::peripherals::timer::sys_tick;
    use $crate::reg::prelude::*;
    use $crate::reg::stk;
    let atoms = sys_tick::Atoms {
      stk_calib: stk::Calib!(Sr),
      stk_ctrl: stk::Ctrl!(Sr),
      stk_load: stk::Load!(Sr),
      stk_val: stk::Val!(Sr),
    };
    atoms.into_driver()
  }
}
