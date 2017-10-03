//! The vector table of interrupt service routines.

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

#[doc(hidden)]
#[macro_export]
macro_rules! vtable_struct {
  ($($irq:ident,)*) => {
    /// The vector table.
    #[allow(dead_code)]
    pub struct VectorTable {
      reset: $crate::vtable::ResetHandler,
      nmi: Option<$crate::vtable::Handler>,
      hard_fault: Option<$crate::vtable::Handler>,
      mem_manage: Option<$crate::vtable::Handler>,
      bus_fault: Option<$crate::vtable::Handler>,
      usage_fault: Option<$crate::vtable::Handler>,
      _reserved0: [$crate::vtable::Reserved; 4],
      sv_call: Option<$crate::vtable::Handler>,
      debug: Option<$crate::vtable::Handler>,
      _reserved1: [$crate::vtable::Reserved; 1],
      pend_sv: Option<$crate::vtable::Handler>,
      sys_tick: Option<$crate::vtable::Handler>,
      $(
        $irq: Option<$crate::vtable::Handler>,
      )*
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! vtable_default {
  ($reset:ident, $($irq:ident,)*) => {
    VectorTable {
      reset: $reset,
      nmi: None,
      hard_fault: None,
      mem_manage: None,
      bus_fault: None,
      usage_fault: None,
      _reserved0: [$crate::vtable::Reserved::Vector; 4],
      sv_call: None,
      debug: None,
      _reserved1: [$crate::vtable::Reserved::Vector; 1],
      pend_sv: None,
      sys_tick: None,
      $(
        $irq: None,
      )*
    }
  };
}

include!(concat!(env!("OUT_DIR"), "/vtable.rs"));

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
#[macro_export]
macro_rules! vtable {
  ($($(#[$meta:meta])* $vector:ident $(as $alias:ident)*;)*) => {
    vtable_struct_with_irq!();

    impl VectorTable {
      /// Constructs a `VectorTable`.
      pub const fn new(reset: $crate::vtable::ResetHandler) -> VectorTable {
        VectorTable {
          $(
            $vector: Some($vector::handler),
          )*
          ..vtable_default_with_irq!(reset)
        }
      }
    }

    $(
      $(#[$meta])*
      pub mod $vector {
        /// The beginning of the routine chain.
        pub static mut ROUTINE: ::drone::routine::Routine =
          ::drone::routine::Routine::new();

        /// The routine handler.
        ///
        /// # Safety
        ///
        /// Should be called only by hardware.
        pub unsafe extern "C" fn handler() {
          ROUTINE.invoke();
        }
      }

      $(#[$meta])*
      pub fn $vector() -> &'static mut ::drone::routine::Routine {
        unsafe { &mut $vector::ROUTINE }
      }

      $(
        pub use self::$vector as $alias;
      )*
    )*
  };
}

#[cfg(test)]
mod tests {
  vtable! {
    #[allow(dead_code)]
    nmi;
    #[allow(dead_code)]
    debug as monitor;
  }

  #[test]
  fn vtable_new() {
    unsafe extern "C" fn reset() -> ! {
      loop {}
    }
    VectorTable::new(reset);
  }
}
