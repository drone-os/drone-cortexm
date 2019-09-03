//! The Supervisor module.
//!
//! Supervisor is an abstraction for the `SVC` assembly instruction, which means
//! **S**uper**V**isor **C**all, and the `SV_CALL` exception.
//!
//! # Usage
//!
//! ```
//! # #![feature(const_fn)]
//! # fn main() {}
//! use drone_cortex_m::{sv, thr};
//!
//! sv! {
//!     /// The supervisor.
//!     pub struct Sv;
//!
//!     /// Array of services.
//!     static SERVICES;
//!
//!     // The list of attached services goes here.
//!     // SwitchContextService;
//!     // SwitchBackService;
//! }
//!
//! thr::vtable! {
//!     use Thr;
//!     pub struct Vtable;
//!     pub struct Handlers;
//!     pub struct Thrs;
//!     static THREADS;
//!
//!     // Define an external function handler for the SV_CALL exception.
//!     fn SV_CALL;
//! }
//!
//! thr! {
//!     use THREADS;
//!     pub struct Thr {}
//!     pub struct ThrLocal {}
//! }
//!
//! #[no_mangle]
//! pub static VTABLE: Vtable = Vtable::new(Handlers {
//!     reset,
//!     // Attach the SV_CALL handler for the supervisor `Sv`.
//!     sv_call: drone_cortex_m::sv::sv_handler::<Sv>,
//! });
//!
//! unsafe extern "C" fn reset() -> ! {
//!     loop {}
//! }
//! ```
//!
//! # Predefined Services
//!
//! If [`SwitchContextService`](sv::SwitchContextService) and
//! [`SwitchBackService`](sv::SwitchBackService) are defined for the supervisor,
//! [`switch_context`](sv::Switch::switch_context) and
//! [`switch_back`](sv::Switch::switch_back) functions become available to
//! switch the program stack.
//!
//! ```no_run
//! # #![feature(new_uninit)]
//! use drone_cortex_m::sv::{Switch, SwitchBackService, SwitchContextService};
//!
//! use drone_cortex_m::sv;
//!
//! sv! {
//!     /// The supervisor.
//!     pub struct Sv;
//!
//!     /// Array of services.
//!     static SERVICES;
//!
//!     SwitchContextService;
//!     SwitchBackService;
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

#![cfg_attr(feature = "std", allow(unused_variables))]

mod switch;

pub use self::switch::{Switch, SwitchBackService, SwitchContextService};

use core::mem::size_of;

/// Generic supervisor.
pub trait Supervisor: Sized + 'static {
    /// Returns a pointer to the first service in the services array.
    fn first() -> *const Self;
}

/// A supervisor call.
pub trait SvCall<T: SvService>: Supervisor {
    /// Calls the supervisor service `service`. Translates to `SVC num`
    /// instruction, where `num` corresponds to the service `T`.
    unsafe fn call(service: &mut T);
}

/// Generic supervisor service.
pub trait SvService: Sized + Send + 'static {
    /// Called when `SVC num` instruction was invoked and `num` corresponds to
    /// the service.
    unsafe extern "C" fn handler(&mut self);
}

/// Calls `SVC num` instruction.
#[inline(always)]
pub unsafe fn sv_call<T: SvService>(service: &mut T, num: u8) {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    {
        if size_of::<T>() == 0 {
            asm!("
                svc $0
            "   :
                : "i"(num)
                :
                : "volatile"
            );
        } else {
            asm!("
                svc $0
            "   :
                : "i"(num), "{r12}"(service)
                :
                : "volatile"
            );
        }
    }
}

/// This function is called by [`sv_handler`] for the supervisor service
/// `T`. Parameter `T` is based on the number `num` in the `SVC num`
/// instruction.
pub unsafe extern "C" fn service_handler<T: SvService>(mut frame: *mut *mut u8) {
    if size_of::<T>() == 0 {
        T::handler(&mut *(frame as *mut T));
    } else {
        frame = frame.add(4); // Stacked R12
        T::handler(&mut *(*frame as *mut T));
    }
}

/// `SV_CALL` exception handler for the supervisor `T`.
#[naked]
pub unsafe extern "C" fn sv_handler<T: Supervisor>() {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    {
        use core::intrinsics::unreachable;
        asm!("
            tst lr, #4
            ite eq
            mrseq r0, msp
            mrsne r0, psp
            ldr r1, [r0, #24]
            ldrb r1, [r1, #-2]
            ldr pc, [r2, r1, lsl #2]
        "   :
            : "{r2}"(T::first())
            : "r0", "r1", "cc"
            : "volatile"
        );
        unreachable();
    }
}
