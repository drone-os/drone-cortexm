//! Interrupt mappings.

#[allow(unused_imports)]
use super::interrupt;
use drone_core::thread::ThreadNumber;
#[allow(unused_imports)]
use thread::prelude::*;

include!(concat!(env!("OUT_DIR"), "/svd_irq.rs"));

/// An interrupt.
pub trait IrqNumber: ThreadNumber {
  /// An interrupt position within the vector table.
  const IRQ_NUMBER: usize;
}

/// Non maskable interrupt.
pub trait IrqNmi: ThreadNumber {}

/// All classes of fault.
pub trait IrqHardFault: ThreadNumber {}

/// Memory management.
pub trait IrqMemManage: ThreadNumber {}

/// Pre-fetch fault, memory access fault.
pub trait IrqBusFault: ThreadNumber {}

/// Undefined instruction or illegal state.
pub trait IrqUsageFault: ThreadNumber {}

/// System service call via SWI instruction.
pub trait IrqSvCall: ThreadNumber {}

/// Monitor.
pub trait IrqDebug: ThreadNumber {}

/// Pendable request for system service.
pub trait IrqPendSv: ThreadNumber {}

/// System tick timer.
pub trait IrqSysTick: ThreadNumber {}
