//! Supervisor.

use core::mem::{size_of, unreachable};
use drone_core::sv::{Supervisor, SvService};

/// Calls `SVC num` instruction.
#[inline(always)]
pub fn sv_call<T: SvService>(service: &mut T, num: u8) {
  unsafe {
    if size_of::<T>() == 0 {
      asm!("
        svc $0
      " :
        : "i"(num)
        :
        : "volatile");
    } else {
      asm!("
        svc $0
      " :
        : "i"(num), "{r12}"(service)
        :
        : "volatile");
    }
  }
}

/// Calls [`T::handler`](SvService::handler).
pub unsafe extern "C" fn service_handler<T>(mut frame: *mut *mut u8)
where
  T: SvService,
{
  if size_of::<T>() == 0 {
    T::handler(&mut *(frame as *mut T));
  } else {
    frame = frame.add(4); // Stacked R12
    T::handler(&mut *(*frame as *mut T));
  }
}

/// An `SVC` handler.
///
/// # Safety
///
/// Must be called only by hardware.
#[naked]
pub unsafe extern "C" fn sv_handler<T: Supervisor>() {
  asm!("
    tst lr, #4
    ite eq
    mrseq r0, msp
    mrsne r0, psp
    ldr r1, [r0, #24]
    ldrb r1, [r1, #-2]
    ldr pc, [r2, r1, lsl #2]
  " :
    : "{r2}"(T::first())
    : "r0", "r1", "cc"
    : "volatile");
  unreachable();
}
