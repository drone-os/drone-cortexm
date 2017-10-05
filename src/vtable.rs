//! The vector table of interrupt service routines.

pub use drone_cortex_m_macros::vtable_imp;

/// Pointer to an exception routine.
pub type Handler = unsafe extern "C" fn();

/// Pointer to a reset routine.
pub type ResetHandler = unsafe extern "C" fn() -> !;

/// Reserved pointer in a vector table.
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Reserved {
  /// The only allowed zero-value.
  Vector = 0,
}

/// Initialize a vector table.
///
/// # Arguments
///
/// * `nmi` - Non maskable interrupt.
/// * `hard_fault` - All classes of fault.
/// * `mem_manage` - Memory management.
/// * `bus_fault` - Pre-fetch fault, memory access fault.
/// * `usage_fault` - Undefined instruction or illegal state.
/// * `sv_call` - System service call via SWI instruction.
/// * `debug` - Monitor.
/// * `pend_sv` - Pendable request for system service.
/// * `sys_tick` - System tick timer.
/// * `irqN` - External interrupt `N`. The number of external interrupts depends
///   on the MCU model.
///
/// # Examples
///
/// ```rust
/// vtable! {
///   /// Non maskable interrupt.
///   nmi;
///   /// All classes of fault.
///   hard_fault;
///   /// System tick timer.
///   sys_tick;
/// }
/// ```
pub macro vtable($($tokens:tt)*) {
  $crate::vtable::vtable_imp!($($tokens)*);
}
