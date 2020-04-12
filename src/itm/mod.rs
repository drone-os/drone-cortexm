//! The Instrumentation Trace Macrocell.
//!
//! This module provides interface to transmit log data through the SWO pin.
//!
//! ITM ports #0 and #1 are reserved for STDOUT and STDERR respectively.

#![cfg_attr(feature = "std", allow(unreachable_code, unused_variables))]

mod port;

pub use self::port::Port;

use crate::{
    map::reg::{dwt, itm, tpiu},
    processor,
    reg::prelude::*,
};
use core::ptr::read_volatile;
use drone_core::token::Token;

const ITM_TER: usize = 0xE000_0E00;
const ITM_TCR: usize = 0xE000_0E80;

/// Returns `true` if the debug probe is connected and listening to the ITM
/// output.
#[inline]
pub fn is_enabled() -> bool {
    #[cfg(feature = "std")]
    return false;
    unsafe { read_volatile(ITM_TCR as *const u32) & 1 != 0 }
}

/// Returns `true` if the debug probe is connected and listening to the output
/// of ITM port number `port`.
#[inline]
pub fn is_port_enabled(port: usize) -> bool {
    #[cfg(feature = "std")]
    return false;
    unsafe { read_volatile(ITM_TER as *const u32) & 1 << port != 0 }
}

/// Writes `bytes` to the ITM port number `port`.
#[inline]
pub fn write_bytes(port: u8, bytes: &[u8]) {
    #[cfg(feature = "std")]
    return;
    Port::new(port as usize).write_bytes(bytes);
}

/// Writes `bytes` to the ITM port number `port`, ensuring no other writes can
/// be made in between.
#[inline]
pub fn write_bytes_exclusive(port: u8, bytes: &[u8]) {
    #[cfg(feature = "std")]
    return;
    unsafe { asm!("cpsid i" :::: "volatile") };
    write_bytes(port, bytes);
    unsafe { asm!("cpsie i" :::: "volatile") };
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
    #[cfg(feature = "std")]
    return;
    let mut cyccnt = unsafe { dwt::Cyccnt::<Urt>::take() };
    cyccnt.store(|r| r.write_cyccnt(0xFFFF_FFFF));
}

/// Updates the SWO prescaler register.
///
/// # Examples
///
/// ```
/// # #![feature(proc_macro_hygiene)]
/// # drone_core::config_override! { "
/// # [memory]
/// # flash = { size = \"128K\", origin = 0x08000000 }
/// # ram = { size = \"20K\", origin = 0x20000000 }
/// # [heap]
/// # size = \"0\"
/// # pools = []
/// # [probe]
/// # gdb-client-command = \"gdb-multiarch\"
/// # [probe.swo]
/// # reset-freq = 8000000
/// # baud-rate = 115200
/// # " }
/// use drone_core::log;
/// use drone_cortex_m::itm;
///
/// itm::update_prescaler(72_000_000 / log::baud_rate!() - 1);
/// ```
#[inline]
pub fn update_prescaler(swoscaler: u32) {
    #[cfg(feature = "std")]
    return;
    let mut acpr = unsafe { tpiu::Acpr::<Urt>::take() };
    acpr.store(|r| r.write_swoscaler(swoscaler));
    sync();
}

#[doc(hidden)]
#[macro_export]
macro_rules! itm_set_log {
    () => {
        const _: () = {
            $crate::reg::assert_taken!("dwt_cyccnt");
            $crate::reg::assert_taken!("itm_tpr");
            $crate::reg::assert_taken!("itm_tcr");
            $crate::reg::assert_taken!("itm_lar");
            $crate::reg::assert_taken!("tpiu_acpr");
            $crate::reg::assert_taken!("tpiu_sppr");
            $crate::reg::assert_taken!("tpiu_ffcr");

            #[no_mangle]
            extern "C" fn drone_log_is_enabled(port: u8) -> bool {
                $crate::itm::is_port_enabled(port as usize)
            }

            #[no_mangle]
            extern "C" fn drone_log_write_bytes(
                port: u8,
                exclusive: bool,
                buffer: *const u8,
                count: usize,
            ) {
                let bytes = unsafe { ::core::slice::from_raw_parts(buffer, count) };
                if exclusive {
                    $crate::itm::write_bytes_exclusive(port, bytes);
                } else {
                    $crate::itm::write_bytes(port, bytes);
                }
            }

            #[no_mangle]
            extern "C" fn drone_log_flush() {
                $crate::itm::flush();
            }
        };
    };
}

/// Sets ITM as default logger.
///
/// # Examples
///
/// ```
/// # #![feature(proc_macro_hygiene)]
/// use drone_cortex_m::{cortex_m_reg_tokens, itm};
///
/// cortex_m_reg_tokens! {
///     struct Regs;
///     !dwt_cyccnt;
///     !itm_tpr; !itm_tcr; !itm_lar;
///     !tpiu_acpr; !tpiu_sppr; !tpiu_ffcr;
/// }
///
/// itm::set_log!();
/// ```
#[doc(inline)]
pub use crate::itm_set_log as set_log;
