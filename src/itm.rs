//! Instrumentation Trace Macrocell.

pub mod macros;
pub mod port;

pub use self::port::Port;

use crate::{
    cpu,
    map::reg::{dwt, itm, scb, tpiu},
    reg::prelude::*,
};
use core::{
    alloc::Layout,
    fmt::{self, Write},
};
use drone_core::{heap::Pool, token::Token};

const TEXT_PORT: usize = 0;
#[cfg(not(feature = "std"))]
const HEAP_TRACE_PORT: usize = 31;
#[cfg(not(feature = "std"))]
const HEAP_TRACE_KEY: u32 = 0xC5AC_CE55;

/// Prints `str` to the ITM port #0.
///
/// See [`print!`](print!) and [`println!`](println!) macros.
#[inline(never)]
pub fn write_str(string: &str) {
    Port::new(TEXT_PORT).write_str(string).unwrap();
}

/// Prints `core::fmt::Arguments` to the ITM port #0.
///
/// See [`print!`](print!) and [`println!`](println!) macros.
#[inline(never)]
pub fn write_fmt(args: fmt::Arguments<'_>) {
    Port::new(TEXT_PORT).write_fmt(args).unwrap();
}

/// Waits until all pending packets transmitted.
#[inline(always)]
pub fn flush() {
    #[inline(never)]
    fn flush() {
        let tcr = unsafe { itm::Tcr::<Urt>::take() };
        while tcr.load().busy() {}
        let acpr = unsafe { tpiu::Acpr::<Urt>::take() };
        cpu::spin(acpr.load().swoscaler());
    }
    if is_enabled() {
        flush();
    }
}

/// Checks if a trace-probe is connected.
#[inline(always)]
pub fn is_enabled() -> bool {
    let demcr = unsafe { scb::Demcr::<Urt>::take() };
    demcr.load().trcena()
}

/// Sets a new rate of SWO output.
pub fn update_rate(swoscaler: usize) {
    let mut acpr = unsafe { tpiu::Acpr::<Urt>::take() };
    acpr.store(|r| r.write_swoscaler(swoscaler as u32));
    sync();
}

/// Sends ITM synchronization packet.
pub fn sync() {
    let mut cyccnt = unsafe { dwt::Cyccnt::<Urt>::take() };
    cyccnt.store(|r| r.write_cyccnt(0xFFFF_FFFF));
}

/// Logs the allocation to the ITM port #31.
#[inline(always)]
pub fn trace_alloc(layout: Layout, _pool: &Pool) {
    #[cfg(not(feature = "std"))]
    #[inline(never)]
    fn instrument(layout: Layout) {
        unsafe { asm!("cpsid i" :::: "volatile") };
        Port::new(HEAP_TRACE_PORT)
            .write(0xCDAB_u16)
            .write((layout.size() as u32) ^ HEAP_TRACE_KEY);
        unsafe { asm!("cpsie i" :::: "volatile") };
    }
    #[cfg(feature = "std")]
    fn instrument(_layout: Layout) {
        unimplemented!();
    }
    if is_enabled() {
        instrument(layout);
    }
}

/// Logs the deallocation to the ITM port #31.
#[inline(always)]
pub fn trace_dealloc(layout: Layout, _pool: &Pool) {
    #[cfg(not(feature = "std"))]
    #[inline(never)]
    fn instrument(layout: Layout) {
        unsafe { asm!("cpsid i" :::: "volatile") };
        Port::new(HEAP_TRACE_PORT)
            .write(0xBADC_u16)
            .write((layout.size() as u32) ^ HEAP_TRACE_KEY);
        unsafe { asm!("cpsie i" :::: "volatile") };
    }
    #[cfg(feature = "std")]
    fn instrument(_layout: Layout) {
        unimplemented!();
    }
    if is_enabled() {
        instrument(layout);
    }
}

/// Logs the reallocation to the ITM port #31.
#[inline(always)]
pub fn trace_grow_in_place(layout: Layout, new_size: usize) {
    #[cfg(not(feature = "std"))]
    #[inline(never)]
    fn instrument(layout: Layout, new_size: usize) {
        unsafe { asm!("cpsid i" :::: "volatile") };
        Port::new(HEAP_TRACE_PORT)
            .write(0xDEBC_u16)
            .write((layout.size() as u32) ^ HEAP_TRACE_KEY)
            .write((new_size as u32) ^ HEAP_TRACE_KEY);
        unsafe { asm!("cpsie i" :::: "volatile") };
    }
    #[cfg(feature = "std")]
    fn instrument(_layout: Layout, _new_size: usize) {
        unimplemented!();
    }
    if is_enabled() {
        instrument(layout, new_size);
    }
}

/// Logs the reallocation to the ITM port #31.
#[inline(always)]
pub fn trace_shrink_in_place(layout: Layout, new_size: usize) {
    #[cfg(not(feature = "std"))]
    #[inline(never)]
    fn instrument(layout: Layout, new_size: usize) {
        unsafe { asm!("cpsid i" :::: "volatile") };
        Port::new(HEAP_TRACE_PORT)
            .write(0xCBED_u16)
            .write((layout.size() as u32) ^ HEAP_TRACE_KEY)
            .write((new_size as u32) ^ HEAP_TRACE_KEY);
        unsafe { asm!("cpsie i" :::: "volatile") };
    }
    #[cfg(feature = "std")]
    fn instrument(_layout: Layout, _new_size: usize) {
        unimplemented!();
    }
    if is_enabled() {
        instrument(layout, new_size);
    }
}
