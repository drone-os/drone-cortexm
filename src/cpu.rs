//! A module for working with CPU.

use map::reg::scb;
use reg::prelude::*;

/// Wait for interrupt.
#[inline(always)]
pub fn wait_for_int() {
  unsafe { asm!("wfi" :::: "volatile") };
}

/// Wait for event.
#[inline(always)]
pub fn wait_for_event() {
  unsafe { asm!("wfe" :::: "volatile") };
}

/// Send event.
#[inline(always)]
pub fn send_event() {
  unsafe { asm!("sev" :::: "volatile") };
}

/// Makes a system reset request.
#[allow(clippy::empty_loop)]
#[inline]
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
    scb::Aircr::<Urt>::new()
      .store(|r| r.write_vectkey(0x05FA).set_sysresetreq());
    loop {}
  }
}

/// Spins a specified amount of CPU cycles.
#[inline]
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
