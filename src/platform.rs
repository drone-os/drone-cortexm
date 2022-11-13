//! ARM Cortex-M CPU management.

#![cfg_attr(feature = "host", allow(unused_variables, unreachable_code))]

#[cfg(not(feature = "host"))]
use core::arch::asm;
#[doc(no_inline)]
pub use drone_core::platform::*;

/// Waits for interrupt.
///
/// It is a hint instruction. It suspends execution, in the lowest power state
/// available consistent with a fast wakeup without the need for software
/// restoration, until a reset, asynchronous exception or other event occurs.
#[inline]
pub fn wait_for_int() {
    #[cfg(feature = "host")]
    return unimplemented!();
    #[cfg(not(feature = "host"))]
    unsafe {
        asm!("wfi", options(nomem, nostack, preserves_flags));
    }
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
    #[cfg(feature = "host")]
    return unimplemented!();
    #[cfg(not(feature = "host"))]
    unsafe {
        asm!("wfe", options(nomem, nostack, preserves_flags));
    }
}

/// Sends an event.
///
/// It is a hint instruction. It causes an event to be signaled to all CPUs
/// within the multiprocessor system.
///
/// See also [`wait_for_event`].
#[inline]
pub fn send_event() {
    #[cfg(feature = "host")]
    return unimplemented!();
    #[cfg(not(feature = "host"))]
    unsafe {
        asm!("sev", options(nomem, nostack, preserves_flags));
    }
}

/// Spins the `cycles` number of processor cycles in a tight loop.
#[inline(always)]
pub fn spin(cycles: u32) {
    #[cfg(feature = "host")]
    return unimplemented!();
    #[cfg(not(feature = "host"))]
    unsafe {
        asm!(
            "0:  subs {0}, {0}, #3",
            "    bhi 0b",
            inlateout(reg) cycles => _,
            options(nomem, nostack),
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
    unsafe {
        core::ptr::write_volatile(
            FPU_CPACR as *mut u32,
            if full_access {
                0xF // full access
            } else {
                0x5 // privileged access only
            } << 20,
        );
    }
}

#[no_mangle]
extern "C" fn drone_save_and_disable_interrupts() -> u32 {
    #[cfg(feature = "host")]
    return unimplemented!();
    #[cfg(not(feature = "host"))]
    unsafe {
        let status: u32;
        asm!(
            "mrs {status}, PRIMASK",
            "cpsid i",
            status = out(reg) status,
            options(nomem, nostack, preserves_flags),
        );
        status
    }
}

#[no_mangle]
extern "C" fn drone_restore_interrupts(status: u32) {
    #[cfg(feature = "host")]
    return unimplemented!();
    #[cfg(not(feature = "host"))]
    unsafe {
        asm!(
            "msr PRIMASK, {status}",
            status = in(reg) status,
            options(nomem, nostack, preserves_flags),
        );
    }
}

#[no_mangle]
extern "C" fn drone_reset() -> ! {
    #[cfg(feature = "host")]
    return unimplemented!();
    #[cfg(not(feature = "host"))]
    unsafe {
        use crate::map::reg::scb;
        use crate::reg::prelude::*;
        use drone_core::token::Token;
        asm!("dmb", "cpsid f", options(nomem, nostack, preserves_flags),);
        scb::Aircr::<Urt>::take().store(|r| r.write_vectkey(0x05FA).set_sysresetreq());
        #[allow(clippy::empty_loop)]
        loop {}
    }
}

#[no_mangle]
extern "C" fn drone_data_mem_init(load: *const usize, base: *mut usize, end: *const usize) {
    #[cfg(feature = "host")]
    return unimplemented!();
    #[cfg(not(feature = "host"))]
    unsafe {
        asm!(
            "   b 1f",
            "0: ldm {load}!, {{{tmp}}}",
            "   stm {base}!, {{{tmp}}}",
            "1: cmp {base}, {end}",
            "   blo 0b",
            load = inout(reg) load => _,
            base = inout(reg) base => _,
            end = in(reg) end,
            tmp = out(reg) _,
            options(nostack),
        );
    }
}

#[no_mangle]
extern "C" fn drone_zeroed_mem_init(base: *mut usize, end: *const usize) {
    #[cfg(feature = "host")]
    return unimplemented!();
    #[cfg(not(feature = "host"))]
    unsafe {
        asm!(
            "   b 1f",
            "0: stm {base}!, {{{zero}}}",
            "1: cmp {base}, {end}",
            "   blo 0b",
            base = inout(reg) base => _,
            end = in(reg) end,
            zero = in(reg) 0,
            options(nostack),
        );
    }
}
