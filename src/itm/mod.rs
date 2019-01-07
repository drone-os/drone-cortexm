//! Instrumentation Trace Macrocell.

pub use self::port::Port;

use core::{
  alloc::Layout,
  fmt::{self, Write},
};
use drone_core::heap::Pool;
use map::reg::{itm, scb};
use reg::prelude::*;

pub mod port;
#[macro_use]
pub mod macros;

const TEXT_PORT: usize = 0;
const HEAP_PORT: usize = 31;

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
pub fn write_fmt(args: fmt::Arguments) {
  Port::new(TEXT_PORT).write_fmt(args).unwrap();
}

/// Waits until all pending packets will be transmitted.
pub fn flush() {
  let tcr = unsafe { itm::Tcr::<Urt>::take() };
  while tcr.load().busy() {}
}

/// Checks if a trace-probe is connected.
#[inline(always)]
pub fn is_enabled() -> bool {
  let demcr = unsafe { scb::Demcr::<Urt>::take() };
  demcr.load().trcena()
}

/// Writes allocation statistics to the ITM port #31.
#[inline(always)]
pub fn instrument_alloc(layout: Layout, pool: &Pool) {
  #[inline(never)]
  fn instrument(layout: Layout, pool: &Pool) {
    Port::new(HEAP_PORT)
      .write(1_u8)
      .write(layout.size() as u32)
      .write(pool.size() as u32);
  }
  if is_enabled() {
    instrument(layout, pool);
  }
}

/// Writes deallocation statistics to the ITM port #31.
#[inline(always)]
pub fn instrument_dealloc(layout: Layout, pool: &Pool) {
  #[inline(never)]
  fn instrument(layout: Layout, pool: &Pool) {
    Port::new(HEAP_PORT)
      .write(0_u8)
      .write((layout.size() as u32).to_be())
      .write((pool.size() as u32).to_be());
  }
  if is_enabled() {
    instrument(layout, pool);
  }
}
