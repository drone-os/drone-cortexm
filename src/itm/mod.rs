//! The Instrumentation Trace Macrocell.
//!
//! This module provides interface to transmit log data through the SWO pin.
//!
//! ITM ports #0 and #1 are reserved for STDOUT and STDERR respectively.

#![cfg_attr(feature = "std", allow(unreachable_code, unused_variables))]

mod macros;
mod port;

/// Updates the SWO prescaler register to match the baud-rate defined at
/// `Drone.toml`.
#[doc(inline)]
pub use drone_cortex_m_macros::itm_update_prescaler as update_prescaler;

pub use self::port::Port;

use crate::{
    map::reg::{dwt, itm, tpiu},
    processor,
    reg::prelude::*,
};
use core::{
    alloc::Layout,
    fmt::{self, Write},
    ptr::read_volatile,
};
use drone_core::{heap::Pool, token::Token};

/// Port number of the standard output stream.
pub const STDOUT_PORT: usize = 0;

/// Port number of the standard error stream.
pub const STDERR_PORT: usize = 1;

/// Port number of the heap trace stream.
pub const HEAP_TRACE_PORT: usize = 31;

/// XOR pattern for heap trace output.
pub const HEAP_TRACE_KEY: u32 = 0xC5AC_CE55;

const ITM_TER: usize = 0xE000_0E00;
const ITM_TCR: usize = 0xE000_0E80;

/// Returns `true` if a debug probe is connected and listening to the ITM
/// output.
#[inline]
pub fn is_enabled() -> bool {
    #[cfg(feature = "std")]
    return false;
    unsafe { read_volatile(ITM_TCR as *const u32) & 1 != 0 }
}

/// Returns `true` if a debug probe is connected and listening to the ITM port
/// `port` output.
#[inline]
pub fn is_port_enabled(port: usize) -> bool {
    #[cfg(feature = "std")]
    return false;
    unsafe { read_volatile(ITM_TER as *const u32) & 1 << port != 0 }
}

/// Writes `string` to the stimulus port number `address`.
///
/// The presence of a debug probe is not checked, so it is recommended to use it
/// together with [`is_port_enabled`].
///
/// # Examples
///
/// ```
/// use drone_cortex_m::itm;
///
/// if itm::is_port_enabled(11) {
///     itm::write_str(11, "hello there!\n");
/// }
/// ```
#[inline(never)]
pub fn write_str(address: usize, string: &str) {
    // Can never be `Err(_)`
    Port::new(address).write_str(string).unwrap_or(())
}

/// Writes `args` to the stimulus port number `address`.
///
/// The presence of a debug probe is not checked, so it is recommended to use it
/// together with [`is_port_enabled`].
///
/// # Examples
///
/// ```
/// use drone_cortex_m::itm;
///
/// let a = 0;
///
/// if itm::is_port_enabled(11) {
///     itm::write_fmt(11, format_args!("a = {}\n", a));
/// }
/// ```
#[inline(never)]
pub fn write_fmt(address: usize, args: fmt::Arguments<'_>) {
    // Can never be `Err(_)`
    Port::new(address).write_fmt(args).unwrap_or(())
}

/// Blocks until all pending packets are transmitted.
///
/// This function is a no-op if no debug probe is connected and listening.
#[inline(always)]
pub fn flush() {
    #[inline(never)]
    fn flush() {
        let tcr = unsafe { itm::Tcr::<Urt>::take() };
        while tcr.load().busy() {}
        let acpr = unsafe { tpiu::Acpr::<Urt>::take() };
        processor::spin(acpr.load().swoscaler() * 64);
    }
    if is_enabled() {
        flush();
    }
}

/// Generates an ITM synchronization packet.
#[inline]
pub fn sync() {
    let mut cyccnt = unsafe { dwt::Cyccnt::<Urt>::take() };
    cyccnt.store(|r| r.write_cyccnt(0xFFFF_FFFF));
}

/// Updates the SWO prescaler register.
#[inline]
pub fn update_prescaler(swoscaler: u32) {
    let mut acpr = unsafe { tpiu::Acpr::<Urt>::take() };
    acpr.store(|r| r.write_swoscaler(swoscaler));
    sync();
}

/// Logs an allocation to the ITM port #31.
///
/// This function is a no-op if no debug probe is connected and listening.
#[inline(always)]
pub fn trace_alloc(layout: Layout, _pool: &Pool) {
    #[inline(never)]
    fn instrument(layout: Layout) {
        #[cfg(feature = "std")]
        return unimplemented!();
        unsafe { asm!("cpsid i" :::: "volatile") };
        Port::new(HEAP_TRACE_PORT)
            .write(0xCDAB_u16)
            .write((layout.size() as u32) ^ HEAP_TRACE_KEY);
        unsafe { asm!("cpsie i" :::: "volatile") };
    }
    if is_port_enabled(HEAP_TRACE_PORT) {
        instrument(layout);
    }
}

/// Logs a deallocation to the ITM port #31.
///
/// This function is a no-op if no debug probe is connected and listening.
#[inline(always)]
pub fn trace_dealloc(layout: Layout, _pool: &Pool) {
    #[inline(never)]
    fn instrument(layout: Layout) {
        #[cfg(feature = "std")]
        return unimplemented!();
        unsafe { asm!("cpsid i" :::: "volatile") };
        Port::new(HEAP_TRACE_PORT)
            .write(0xBADC_u16)
            .write((layout.size() as u32) ^ HEAP_TRACE_KEY);
        unsafe { asm!("cpsie i" :::: "volatile") };
    }
    if is_port_enabled(HEAP_TRACE_PORT) {
        instrument(layout);
    }
}

/// Logs growing in place to the ITM port #31.
///
/// This function is a no-op if no debug probe is connected and listening.
#[inline(always)]
pub fn trace_grow_in_place(layout: Layout, new_size: usize) {
    #[inline(never)]
    fn instrument(layout: Layout, new_size: usize) {
        #[cfg(feature = "std")]
        return unimplemented!();
        unsafe { asm!("cpsid i" :::: "volatile") };
        Port::new(HEAP_TRACE_PORT)
            .write(0xDEBC_u16)
            .write((layout.size() as u32) ^ HEAP_TRACE_KEY)
            .write((new_size as u32) ^ HEAP_TRACE_KEY);
        unsafe { asm!("cpsie i" :::: "volatile") };
    }
    if is_port_enabled(HEAP_TRACE_PORT) {
        instrument(layout, new_size);
    }
}

/// Logs shrinking in place to the ITM port #31.
///
/// This function is a no-op if no debug probe is connected and listening.
#[inline(always)]
pub fn trace_shrink_in_place(layout: Layout, new_size: usize) {
    #[inline(never)]
    fn instrument(layout: Layout, new_size: usize) {
        #[cfg(feature = "std")]
        return unimplemented!();
        unsafe { asm!("cpsid i" :::: "volatile") };
        Port::new(HEAP_TRACE_PORT)
            .write(0xCBED_u16)
            .write((layout.size() as u32) ^ HEAP_TRACE_KEY)
            .write((new_size as u32) ^ HEAP_TRACE_KEY);
        unsafe { asm!("cpsie i" :::: "volatile") };
    }
    if is_port_enabled(HEAP_TRACE_PORT) {
        instrument(layout, new_size);
    }
}
