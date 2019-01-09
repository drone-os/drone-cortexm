//! SysTick resource.

use crate::map;
use drone_core::res;

res::one! {
  /// SysTick resource.
  pub struct SysTick;
  map::reg; map::res::sys_tick;

  SCB {
    ICSR {
      PENDSTCLR;
      PENDSTSET;
    }
  }

  STK {
    CTRL;
    LOAD;
    VAL;
  }
}
