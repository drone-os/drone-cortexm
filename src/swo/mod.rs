//! Single Wire Output interface.
//!
//! This module provides interface for ITM (Instrumentation Trace Macrocell)
//! output through SWO (Single Wire Output) pin, and optionally the respective
//! implementation for `drone_core::log` facade (via `set_log!` macro).

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

/// Number of ports.
pub const PORTS_COUNT: u8 = 32;

const ITM_TER: usize = 0xE000_0E00;
const ITM_TCR: usize = 0xE000_0E80;

/// Returns `true` if the debug probe is connected and listening to the ITM
/// output.
#[inline]
pub fn is_enabled() -> bool {
    #[cfg(feature = "std")]
    return unimplemented!();
    unsafe { read_volatile(ITM_TCR as *const u32) & 1 != 0 }
}

/// Returns `true` if the debug probe is connected and listening to the output
/// of ITM port number `port`.
#[inline]
pub fn is_port_enabled(port: usize) -> bool {
    #[cfg(feature = "std")]
    return unimplemented!();
    unsafe { read_volatile(ITM_TER as *const u32) & 1 << port != 0 }
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
    return unimplemented!();
    let mut cyccnt = unsafe { dwt::Cyccnt::<Urt>::take() };
    cyccnt.store(|r| r.write_cyccnt(0xFFFF_FFFF));
}

/// Updates the SWO prescaler register.
///
/// # Examples
///
/// ```no_run
/// # #![feature(proc_macro_hygiene)]
/// # drone_core::config_override! { "
/// # [memory]
/// # flash = { size = \"128K\", origin = 0x08000000 }
/// # ram = { size = \"20K\", origin = 0x20000000 }
/// # [heap.main]
/// # size = \"0\"
/// # pools = []
/// # [linker]
/// # platform = \"arm\"
/// # [probe]
/// # gdb-client-command = \"gdb-multiarch\"
/// # [log.swo]
/// # reset-freq = 8000000
/// # baud-rate = 115200
/// # " }
/// use drone_core::log;
/// use drone_cortexm::swo;
///
/// swo::update_prescaler(72_000_000 / log::baud_rate!() - 1);
/// ```
#[inline]
pub fn update_prescaler(swoscaler: u32) {
    #[cfg(feature = "std")]
    return unimplemented!();
    let mut acpr = unsafe { tpiu::Acpr::<Urt>::take() };
    acpr.store(|r| r.write_swoscaler(swoscaler));
    sync();
}

#[doc(hidden)]
#[macro_export]
macro_rules! swo_set_log {
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
                $crate::swo::is_port_enabled(port as usize)
            }

            #[no_mangle]
            extern "C" fn drone_log_write_bytes(port: u8, buffer: *const u8, count: usize) {
                let bytes = unsafe { ::core::slice::from_raw_parts(buffer, count) };
                $crate::swo::Port::new(port).write_bytes(bytes);
            }

            #[no_mangle]
            extern "C" fn drone_log_write_u8(port: u8, value: u8) {
                $crate::swo::Port::new(port).write(value);
            }

            #[no_mangle]
            extern "C" fn drone_log_write_u16(port: u8, value: u16) {
                $crate::swo::Port::new(port).write(value);
            }

            #[no_mangle]
            extern "C" fn drone_log_write_u32(port: u8, value: u32) {
                $crate::swo::Port::new(port).write(value);
            }

            #[no_mangle]
            extern "C" fn drone_log_flush() {
                $crate::swo::flush();
            }
        };
    };
}

/// Sets SWO as default logger.
///
/// # Examples
///
/// ```
/// # #![feature(proc_macro_hygiene)]
/// use drone_cortexm::{cortexm_reg_tokens, swo};
///
/// cortexm_reg_tokens! {
///     index => Regs;
///     exclude => {
///         dwt_cyccnt,
///         itm_tpr, itm_tcr, itm_lar,
///         tpiu_acpr, tpiu_sppr, tpiu_ffcr,
///     }
/// }
///
/// swo::set_log!();
/// ```
#[doc(inline)]
pub use crate::swo_set_log as set_log;
