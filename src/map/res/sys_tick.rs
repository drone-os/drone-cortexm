//! SysTick resource.

use drone_core::res;
use map;

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
