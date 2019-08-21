//! A module for working with CPU.

/// Wait for interrupt.
#[inline]
pub fn wait_for_int() {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    unsafe {
        asm!("wfi" :::: "volatile");
    }
}

/// Wait for event.
#[inline]
pub fn wait_for_event() {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    unsafe {
        asm!("wfe" :::: "volatile");
    }
}

/// Send event.
#[inline]
pub fn send_event() {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    unsafe {
        asm!("sev" :::: "volatile");
    }
}

/// Makes a system reset request.
#[allow(clippy::empty_loop)]
#[inline]
pub fn self_reset() -> ! {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    unsafe {
        use crate::{map::reg::scb, reg::prelude::*};
        use drone_core::token::Token;
        asm!("
            dmb
            cpsid f
        "   :
            :
            :
            : "volatile"
        );
        scb::Aircr::<Urt>::take().store(|r| r.write_vectkey(0x05FA).set_sysresetreq());
        loop {}
    }
}

/// Spins a specified amount of CPU cycles.
#[cfg_attr(feature = "std", allow(unused_mut))]
#[allow(unused_assignments, unused_variables)]
#[inline(always)]
pub fn spin(mut cycles: u32) {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    unsafe {
        asm!("
        0:
            subs $0, $0, #2
            bhi 0b
        "   : "+r"(cycles)
            :
            : "cc"
            : "volatile"
        );
    }
}
