//! The vector table.
//!
//! # Configuration
//!
//! The vector table is configured by [`vtable!`] macro.
//!
//! ```rust
//! vtable! {
//!   /// The vector table.
//!   VectorTable;
//!   /// Array of threads.
//!   THREADS;
//!   ThreadLocal;
//!
//!   /// Non maskable interrupt.
//!   NMI;
//!   /// All classes of fault.
//!   HARD_FAULT;
//!   /// System tick timer.
//!   SYS_TICK;
//!   /// RCC global interrupt.
//!   5: rcc; // Give IRQ5 a name
//! }
//! ```
//!
//! # Preconfigured exceptions
//!
//! * `NMI` - Non maskable interrupt.
//! * `HARD_FAULT` - All classes of fault.
//! * `MEM_MANAGE` - Memory management.
//! * `BUS_FAULT` - Pre-fetch fault, memory access fault.
//! * `USAGE_FAULT` - Undefined instruction or illegal state.
//! * `SV_CALL` - System service call via SWI instruction.
//! * `DEBUG` - Monitor.
//! * `PEND_SV` - Pendable request for system service.
//! * `SYS_TICK` - System tick timer.
//!
//! [`vtable!`]: ../macro.vtable.html

mod thread_interrupt;
pub mod interrupts;

pub use self::thread_interrupt::ThreadInterrupt;
pub use drone_cortex_m_macros::interrupt;

/// Pointer to an exception handler.
pub type Handler = unsafe extern "C" fn();

/// Pointer to a reset handler.
pub type ResetHandler = unsafe extern "C" fn() -> !;

/// Reserved pointer in a vector table.
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Reserved {
  /// The only allowed zero-value.
  Vector = 0,
}
