//! Common utility functions for working with ARM Cortex-M processors.

#![cfg_attr(feature = "std", allow(unused_mut))]

/// Wait for interrupt.
///
/// It is a hint instruction. It suspends execution, in the lowest power state
/// available consistent with a fast wakeup without the need for software
/// restoration, until a reset, asynchronous exception or other event occurs.
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
///
/// It is a hint instruction. If the Event Register is clear, it suspends
/// execution in the lowest power state available consistent with a fast wakeup
/// without the need for software restoration, until a reset, exception or other
/// event occurs.
///
/// See also [`send_event`].
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
///
/// It is a hint instruction. It causes an event to be signaled to all CPUs
/// within the multiprocessor system.
///
/// See also [`wait_for_event`].
#[inline]
pub fn send_event() {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    unsafe {
        asm!("sev" :::: "volatile");
    }
}

/// Requests system reset.
///
/// Generates a system reset request to the microcontroller's system reset
/// control logic. Because the system reset control logic is not a part of the
/// processor design, the exact timing of the reset is device-specific.
///
/// The debug logic is not affected.
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

/// Spins the `cycles` number of processor cycles in a loop.
#[allow(unused_assignments, unused_variables)]
#[inline(always)]
pub fn spin(mut cycles: u32) {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    unsafe {
        asm!("
        0:
            subs $0, $0, #3
            bhi 0b
        "   : "+r"(cycles)
            :
            : "cc"
            : "volatile"
        );
    }
}
