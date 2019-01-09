//! Collection of macros.

/// Macro for printing through ITM.
#[macro_export]
macro_rules! print {
  ($str:expr) => {
    if $crate::itm::is_enabled() {
      $crate::itm::write_str($str);
    }
  };

  ($($arg:tt)*) => {
    if $crate::itm::is_enabled() {
      $crate::itm::write_fmt(format_args!($($arg)*));
    }
  };
}

/// Macro for printing through ITM, with a newline.
#[macro_export]
macro_rules! println {
  () => {
    $crate::print!("\n");
  };

  ($fmt:expr) => {
    $crate::print!(concat!($fmt, "\n"));
  };

  ($fmt:expr, $($arg:tt)*) => {
    $crate::print!(concat!($fmt, "\n"), $($arg)*);
  };
}
