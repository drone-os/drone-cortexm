//! Registers for Drone threads.

use drone_core::periph;

periph::singular! {
    /// Extracts Drone thread register tokens.
    pub macro periph_thr;

    /// Registers for Drone threads.
    pub struct Thr;

    crate::map::reg;
    crate::map::periph::thr;

    SCB {
        CCR;
    }
}
