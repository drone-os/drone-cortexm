//! The vector table of interrupt service routines.

use alloc::boxed::FnBox;
use core::mem;
use core::ops::{Generator, GeneratorState};
use drone::prelude::*;

/// Pointer to an exception routine.
pub type Handler = unsafe extern "C" fn();

/// Pointer to a reset routine.
pub type ResetHandler = unsafe extern "C" fn() -> !;

/// Data attached to a routine.
pub enum RoutineData {
  /// No associated data.
  Empty,
  /// Attached generator.
  Generator(Box<Generator<Yield = (), Return = ()>>),
  /// Attached closure.
  Callback(Box<FnBox()>),
}

/// Reserved pointer in a vector table.
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Reserved {
  /// The only allowed zero-value.
  Vector = 0,
}

#[doc(hidden)]
pub fn routine_handler(data: &mut RoutineData) {
  match mem::replace(data, RoutineData::Empty) {
    RoutineData::Generator(mut generator) => match generator.resume() {
      GeneratorState::Yielded(()) => {
        *data = RoutineData::Generator(generator);
      }
      GeneratorState::Complete(()) => {}
    },
    RoutineData::Callback(callback) => {
      callback();
    }
    RoutineData::Empty => {}
  }
}

#[doc(hidden)]
pub fn routine_append<G>(data: &mut RoutineData, g: G)
where
  G: Generator<Yield = (), Return = ()> + Send + 'static,
{
  *data = RoutineData::Generator(Box::new(g));
}

#[doc(hidden)]
pub fn routine_append_callback<F>(data: &mut RoutineData, f: F)
where
  F: FnOnce() + Send + 'static,
{
  *data = RoutineData::Callback(Box::new(f));
}

#[doc(hidden)]
pub fn routine_clear(data: &mut RoutineData) {
  *data = RoutineData::Empty;
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
///   #[doc = "Non maskable interrupt."]
///   nmi,
///   #[doc = "All classes of fault."]
///   hard_fault,
///   #[doc = "System tick timer."]
///   sys_tick,
/// }
/// ```
#[macro_export]
macro_rules! vtable {
  ($($(#[$meta:meta])* $vector:ident,)*) => {
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
        static mut DATA: $crate::vtable::RoutineData =
          $crate::vtable::RoutineData::Empty;

        /// The routine handler.
        ///
        /// # Safety
        ///
        /// Must be called only by hardware.
        pub unsafe extern "C" fn handler() {
          $crate::vtable::routine_handler(&mut DATA);
        }

        /// Appends `Generator` routine handler.
        ///
        /// Generator `g` will be resumed each time the handler is called. Will
        /// be removed once got complete.
        pub fn append<G>(g: G)
        where
          G: ::core::ops::Generator<Yield = (), Return = ()> + Send + 'static,
        {
          $crate::vtable::routine_append(unsafe { &mut DATA }, g);
        }

        /// Appends `FnOnce` routine handler.
        ///
        /// Will be invoked once when the handler is called, and then removed.
        pub fn append_callback<F>(f: F)
        where
          F: FnOnce() + Send + 'static,
        {
          $crate::vtable::routine_append_callback(unsafe { &mut DATA }, f);
        }

        /// Clears all attached handlers.
        pub fn clear() {
          $crate::vtable::routine_clear(unsafe { &mut DATA });
        }
      }
    )*
  };
}

#[cfg(test)]
pub mod tests {
  use alloc::arc::Arc;
  use drone::ALLOCATOR;
  use drone::sync::Mutex;

  vtable!(nmi,);

  #[allow(dead_code)]
  static VECTOR_TABLE: VectorTable = VectorTable::new(main);

  extern "C" fn main() -> ! {
    loop {}
  }

  #[test]
  fn generator() {
    let heap = [0; 1024];
    unsafe {
      ALLOCATOR.lock().init(&heap as *const i32 as usize, 1024);
    }
    let x = Arc::new(Mutex::new(0));
    let y = Arc::clone(&x);
    nmi::append(move || loop {
      {
        let mut counter = x.lock();
        *counter += 1;
      }
      yield;
    });
    assert_eq!(*y.lock(), 0);
    unsafe {
      nmi::handler();
    }
    assert_eq!(*y.lock(), 1);
    unsafe {
      nmi::handler();
    }
    assert_eq!(*y.lock(), 2);
    nmi::clear();
  }

  #[test]
  fn callback() {
    let heap = [0; 1024];
    unsafe {
      ALLOCATOR.lock().init(&heap as *const i32 as usize, 1024);
    }
    let x = Arc::new(Mutex::new(0));
    let y = Arc::clone(&x);
    nmi::append_callback(move || {
      let mut counter = x.lock();
      *counter += 1;
    });
    assert_eq!(*y.lock(), 0);
    unsafe {
      nmi::handler();
    }
    assert_eq!(*y.lock(), 1);
    unsafe {
      nmi::handler();
    }
    assert_eq!(*y.lock(), 1);
    nmi::clear();
  }
}
