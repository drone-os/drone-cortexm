//! A module for working with CPU.

use reg::prelude::*;
use reg::scb::Aircr;

/// Wait for Interrupt.
#[inline(always)]
pub fn wait_for_interrupt() {
  unsafe { asm!("wfi" :::: "volatile") };
}

/// Makes a system reset request.
#[cfg_attr(feature = "clippy", allow(empty_loop))]
#[inline(always)]
pub fn self_reset() -> ! {
  unsafe {
    asm!("
      dmb
      cpsid f
    " :
      :
      :
      : "volatile"
    );
    Aircr::<Urt>::new().store(|r| r.write_vectkey(0x05FA).set_sysresetreq());
    loop {}
  }
}

/// Spins a specified amount of CPU cycles.
#[inline(always)]
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
