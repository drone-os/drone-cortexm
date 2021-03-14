//! The Supervisor module.
//!
//! Supervisor is an abstraction for the `SVC` assembly instruction, which means
//! **S**uper**V**isor **C**all, and the `SV_CALL` exception.
//!
//! # Usage
//!
//! ```
//! # #![feature(const_fn_fn_ptr_basics)]
//! # #![feature(naked_functions)]
//! # fn main() {}
//! use drone_cortexm::{sv, sv::Supervisor, thr};
//!
//! sv::pool! {
//!     /// Pool of services.
//!     pool => SERVICES;
//!
//!     /// Supervisor type.
//!     supervisor => pub Sv;
//!
//!     // Attached services.
//!     services => {
//!         // SwitchContextService;
//!         // SwitchBackService;
//!     }
//! }
//!
//! thr::nvic! {
//!     thread => pub Thr {};
//!     local => pub ThrLocal {};
//!     index => pub Thrs;
//!     vtable => pub Vtable;
//!     init => pub ThrsInit;
//!     supervisor => Sv;
//!     threads => {
//!         exceptions => {
//!             // Define an external function handler for the SV_CALL exception.
//!             naked(Sv::handler) sv_call;
//!         };
//!     };
//! }
//!
//! #[no_mangle]
//! pub static VTABLE: Vtable = Vtable::new(reset);
//!
//! unsafe extern "C" fn reset() -> ! {
//!     loop {}
//! }
//! ```
//!
//! # Predefined Services
//!
//! If [`SwitchContextService`] and [`SwitchBackService`] are defined for the
//! supervisor, [`Switch::switch_context`] and [`Switch::switch_back`] functions
//! become available to switch the program stack.
//!
//! ```no_run
//! # #![feature(const_fn_fn_ptr_basics)]
//! # #![feature(naked_functions)]
//! # #![feature(new_uninit)]
//! use drone_cortexm::sv::{Switch, SwitchBackService, SwitchContextService};
//!
//! use drone_cortexm::sv;
//!
//! sv::pool! {
//!     /// Pool of services.
//!     pool => SERVICES;
//!
//!     /// The supervisor type.
//!     supervisor => pub Sv;
//!
//!     // Attached services.
//!     services => {
//!         SwitchContextService;
//!         SwitchBackService;
//!     }
//! }
//!
//! # fn main() {
//! unsafe {
//!     // Allocate the stack.
//!     let stack = Box::<[u8]>::new_uninit_slice(0x800).assume_init();
//!     // `stack_ptr` will store the current stack pointer.
//!     let mut stack_ptr = stack.as_ptr();
//!     let mut data = Box::<u32>::new(0);
//!     let mut data_ptr = &mut *data as *mut u32;
//!     Sv::switch_context(data_ptr, &mut stack_ptr);
//!     // -------------------
//!     // Using the new stack.
//!     // -------------------
//!     Sv::switch_back(&mut data_ptr);
//! }
//! # }
//! ```

#![cfg_attr(feature = "std", allow(unreachable_code, unused_variables))]

mod switch;

pub use self::switch::{Switch, SwitchBackService, SwitchContextService};

/// Defines the supervisor type.
///
/// See [the module level documentation](self) for details.
#[doc(inline)]
pub use drone_cortexm_macros::sv_pool as pool;

use core::mem::size_of;

/// Generic supervisor.
pub trait Supervisor: Sized + 'static {
    /// `SV_CALL` exception handler for the supervisor.
    ///
    /// # Safety
    ///
    /// This function should be called only by NVIC as part of the vector table.
    unsafe extern "C" fn handler();
}

/// A supervisor call.
pub trait SvCall<T: SvService>: Supervisor {
    /// Calls the supervisor service `service`. Translates to `SVC num`
    /// instruction, where `num` corresponds to the service `T`.
    ///
    /// # Safety
    ///
    /// Safety is implementation defined.
    unsafe fn call(service: &mut T);
}

/// Generic supervisor service.
pub trait SvService: Sized + Send + 'static {
    /// Called when `SVC num` instruction was invoked and `num` corresponds to
    /// the service.
    ///
    /// # Safety
    ///
    /// This function should not be called directly.
    unsafe extern "C" fn handler(&mut self);
}

/// Calls `SVC num` instruction.
///
/// # Safety
///
/// This function should not be called directly.
#[inline(always)]
pub unsafe fn sv_call<T: SvService, const NUM: u8>(service: &mut T) {
    #[cfg(feature = "std")]
    return unimplemented!();
    #[cfg(not(feature = "std"))]
    unsafe {
        if size_of::<T>() == 0 {
            asm!(
                "svc {}",
                const NUM,
                options(nomem, preserves_flags),
            );
        } else {
            asm!(
                "svc {}",
                const NUM,
                in("r12") service,
                options(nomem, preserves_flags),
            );
        }
    }
}

/// This function is called by [`Sv::handler`] for the supervisor service
/// `T`. Parameter `T` is based on the number `num` in the `SVC num`
/// instruction.
///
/// # Safety
///
/// This function should not be called directly.
pub unsafe extern "C" fn service_handler<T: SvService>(mut frame: *mut *mut u8) {
    if size_of::<T>() == 0 {
        unsafe { T::handler(&mut *frame.cast::<T>()) };
    } else {
        unsafe {
            frame = frame.add(4); // Stacked R12
            T::handler(&mut *(*frame).cast::<T>());
        }
    }
}
