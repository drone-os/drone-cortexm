//! The vector table.
//!
//! # Configuration
//!
//! The vector table is configured by [`vtable!`] macro.
//!
//! ```rust
//! vtable! {
//!   //! The vector table.
//!
//!   /// Non maskable interrupt.
//!   nmi;
//!   /// All classes of fault.
//!   hard_fault;
//!   /// System tick timer.
//!   sys_tick;
//!   /// RCC global interrupt.
//!   5: rcc; // Give IRQ5 a name
//! }
//! ```
//!
//! # Preconfigured exceptions
//!
//! * `nmi` - Non maskable interrupt.
//! * `hard_fault` - All classes of fault.
//! * `mem_manage` - Memory management.
//! * `bus_fault` - Pre-fetch fault, memory access fault.
//! * `usage_fault` - Undefined instruction or illegal state.
//! * `sv_call` - System service call via SWI instruction.
//! * `debug` - Monitor.
//! * `pend_sv` - Pendable request for system service.
//! * `sys_tick` - System tick timer.
//!
//! [`vtable!`]: ../macro.vtable.html

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
