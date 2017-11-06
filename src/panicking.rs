//! Panicking support.

use {itm, mcu};
use core::fmt;

/// Panic handler.
///
/// It attempts to write a panic message to ITM and resets the device.
#[cfg_attr(feature = "clippy", allow(empty_loop))]
#[linkage = "weak"]
#[lang = "panic_fmt"]
unsafe extern "C" fn begin(
  args: fmt::Arguments,
  file: &'static str,
  line: u32,
  _col: u32,
) -> ! {
  print!("panicked at '");
  itm::write_fmt(args);
  println!("', {}:{}", file, line);
  itm::flush();
  mcu::reset_request();
  loop {}
}
