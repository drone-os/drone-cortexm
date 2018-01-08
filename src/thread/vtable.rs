//! The vector table support.

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
