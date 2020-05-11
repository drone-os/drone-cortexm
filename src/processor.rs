//! Common utility functions for working with ARM Cortex-M processors.

#![cfg_attr(feature = "std", allow(unreachable_code, unused_mut))]

/// Waits for interrupt.
///
/// It is a hint instruction. It suspends execution, in the lowest power state
/// available consistent with a fast wakeup without the need for software
/// restoration, until a reset, asynchronous exception or other event occurs.
#[inline]
pub fn wait_for_int() {
    #[cfg(feature = "std")]
    return unimplemented!();
    unsafe { llvm_asm!("wfi" :::: "volatile") };
}

/// Waits for event.
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
    return unimplemented!();
    unsafe { llvm_asm!("wfe" :::: "volatile") };
}

/// Sends event.
///
/// It is a hint instruction. It causes an event to be signaled to all CPUs
/// within the multiprocessor system.
///
/// See also [`wait_for_event`].
#[inline]
pub fn send_event() {
    #[cfg(feature = "std")]
    return unimplemented!();
    unsafe { llvm_asm!("sev" :::: "volatile") };
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
    return unimplemented!();
    unsafe {
        use crate::{map::reg::scb, reg::prelude::*};
        use drone_core::token::Token;
        llvm_asm!("
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
    return unimplemented!();
    unsafe {
        llvm_asm!("
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

/// Enables the FPU.
///
/// The FPU is disabled from reset. You must enable it before you can use any
/// floating-point instructions.
///
/// # Safety
///
/// * The processor must be in privileged mode
/// * The function rewrites contents of FPU_CPACR register without taking into
///   account register tokens
#[cfg(feature = "floating-point-unit")]
#[inline]
pub unsafe fn fpu_init(full_access: bool) {
    const FPU_CPACR: usize = 0xE000_ED88;
    core::ptr::write_volatile(
        FPU_CPACR as *mut u32,
        if full_access {
            0xF // full access
        } else {
            0x5 // privileged access only
        } << 20,
    );
}
