//! SysTick timer.

use drone_core::periph;

periph::singular! {
    /// Extracts SysTick register tokens.
    pub macro periph_sys_tick;

    /// SysTick peripheral.
    pub struct SysTick;

    crate::map::reg;
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
