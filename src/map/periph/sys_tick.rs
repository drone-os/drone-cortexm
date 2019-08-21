//! SysTick timer.

use crate::map;
use drone_core::periph;

periph::singular! {
    /// Acquires SysTick.
    pub macro periph_sys_tick;

    /// SysTick.
    pub struct SysTickPeriph;

    map::reg;
    crate::map::periph::sys_tick;

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
