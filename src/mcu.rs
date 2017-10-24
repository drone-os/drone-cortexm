//! A module for working with MCU.

use core::ptr::write_volatile;
use drone::thread::Executor;
use futures::{Async, Future};

const AIRCR: usize = 0xE000_ED0C;

/// Wait for Interrupt.
pub fn wait_for_interrupt() {
  unsafe { asm!("wfi" :::: "volatile") };
}

/// Performs a system reset request.
///
/// This function writes to the application interrupt and reset control register
/// (`AIRCR`).
pub fn reset_request() {
  unsafe { write_volatile(AIRCR as *mut usize, 0x05FA_0004) };
}

/// Spins a specified amount of CPU cycles.
pub fn spin(mut _cycles: u32) {
  unsafe {
    asm!("
      0:
        subs $0, $0, #2
        bhi 0b
    " : "+r"(_cycles)
      :
      : "cc"
      : "volatile");
  }
}

/// Blocks the current thread until the `future` is resolved.
pub fn wait_for<R, E, F>(future: F) -> Result<R, E>
where
  F: Future<Item = R, Error = E>,
{
  let mut executor = Executor::new(future);
  loop {
    match executor.poll() {
      Ok(Async::NotReady) => wait_for_interrupt(),
      Ok(Async::Ready(ready)) => break Ok(ready),
      Err(err) => break Err(err),
    }
  }
}
