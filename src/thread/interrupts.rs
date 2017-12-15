//! Interrupt mappings.

#[allow(unused_imports)]
use super::interrupt;

include!(concat!(env!("OUT_DIR"), "/svd_interrupts.rs"));

/// Non maskable interrupt.
pub trait IrqNmi<T: Thread>: ThreadBinding<T> {}

/// All classes of fault.
pub trait IrqHardFault<T: Thread>: ThreadBinding<T> {}

/// Memory management.
pub trait IrqMemManage<T: Thread>: ThreadBinding<T> {}

/// Pre-fetch fault, memory access fault.
pub trait IrqBusFault<T: Thread>: ThreadBinding<T> {}

/// Undefined instruction or illegal state.
pub trait IrqUsageFault<T: Thread>: ThreadBinding<T> {}

/// System service call via SWI instruction.
pub trait IrqSvCall<T: Thread>: ThreadBinding<T> {}

/// Monitor.
pub trait IrqDebug<T: Thread>: ThreadBinding<T> {}

/// Pendable request for system service.
pub trait IrqPendSv<T: Thread>: ThreadBinding<T> {}

/// System tick timer.
pub trait IrqSysTick<T: Thread>: ThreadBinding<T> {}
