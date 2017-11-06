//! Instrumentation Trace Macrocell.

pub use self::port::Port;
use core::fmt::{self, Write};
use core::ptr::{read_volatile, write_volatile};
use mcu;

const POST_FLUSH_WAIT: u32 = 0x400;
const DBGMCU_CR: usize = 0xE004_2004;
const DEMCR: usize = 0xE000_EDFC;
const TPIU_SPPR: usize = 0xE004_00F0;
const TPIU_FFCR: usize = 0xE004_0304;
const ITMLA: usize = 0xE000_0FB0;
const ITMTC: usize = 0xE000_0E80;
const ITMTP: usize = 0xE000_0E40;

pub mod port;
#[macro_use]
pub mod macros;

/// Initializes ITM.
///
/// # Safety
///
/// Must be called exactly once and as early as possible.
pub unsafe fn init() {
  // Trace pin assignment control.
  write_volatile(DBGMCU_CR as *mut usize, 0x0000_0020);
  // Global enable for all DWT and ITM features.
  write_volatile(DEMCR as *mut usize, 0x0100_0000);
  // SerialWire Output (NRZ).
  write_volatile(TPIU_SPPR as *mut usize, 0b0000_0010);
  // Continuous Formatting.
  write_volatile(TPIU_FFCR as *mut usize, 0x0000_0100);
  // Unlock Write Access to the other ITM registers
  write_volatile(ITMLA as *mut usize, 0xC5AC_CE55);
  // Enable ITM and set ATB ID for CoreSight system.
  write_volatile(ITMTC as *mut usize, 0x0001_0001);
  // Bit mask to enable tracing on ITM stimulus ports.
  write_volatile(ITMTP as *mut usize, 0x0000_0001);
}

/// Prints `str` to the ITM port #0.
///
/// See [print](../macro.print.html) and [println](../macro.println.html)
/// macros.
pub fn write_str(string: &str) {
  Port::new(0).write_str(string).unwrap();
}

/// Prints `core::fmt::Arguments` to the ITM port #0.
///
/// See [print](../macro.print.html) and [println](../macro.println.html)
/// macros.
pub fn write_fmt(args: fmt::Arguments) {
  Port::new(0).write_fmt(args).unwrap();
}

/// Waits until all pending packets will be transmitted.
pub fn flush() {
  while unsafe { read_volatile(ITMTC as *const usize) } & 0b1 << 23 != 0 {}
  mcu::spin(POST_FLUSH_WAIT); // Additional wait due to asynchronous output
}
