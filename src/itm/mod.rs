//! Instrumentation Trace Macrocell.

pub use self::port::Port;
use core::fmt::{self, Write};
use cpu;
use reg::itm::Tcr;
use reg::prelude::*;

const POST_FLUSH_WAIT: u32 = 0x400;

pub mod port;
#[macro_use]
pub mod macros;

/// Prints `str` to the ITM port #0.
///
/// See [`print!`](print!) and [`println!`](println!) macros.
pub fn write_str(string: &str) {
  Port::new(0).write_str(string).unwrap();
}

/// Prints `core::fmt::Arguments` to the ITM port #0.
///
/// See [`print!`](print!) and [`println!`](println!) macros.
pub fn write_fmt(args: fmt::Arguments) {
  Port::new(0).write_fmt(args).unwrap();
}

/// Waits until all pending packets will be transmitted.
pub fn flush() {
  let tcr = unsafe { Tcr::<Urt>::new() };
  while tcr.load().busy() {}
  cpu::spin(POST_FLUSH_WAIT); // Additional wait due to asynchronous output
}
