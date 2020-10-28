//! The Supervisor module.
//!
//! Supervisor is an abstraction for the `SVC` assembly instruction, which means
//! **S**uper**V**isor **C**all, and the `SV_CALL` exception.
//!
//! # Usage
//!
//! ```
//! # #![feature(const_fn_fn_ptr_basics)]
//! # fn main() {}
//! use drone_cortexm::{sv, thr};
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
//! thr! {
//!     thread => pub Thr {};
//!     local => pub ThrLocal {};
//!     vtable => pub Vtable;
//!     index => pub Thrs;
//!     init => pub ThrsInit;
//!     threads => {
//!         exceptions => {
//!             // Define an external function handler for the SV_CALL exception.
//!             naked(sv::sv_handler::<Sv>) sv_call,
//!         },
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
//! # #![feature(new_uninit)]
//! use drone_cortexm::sv::{Switch, SwitchBackService, SwitchContextService};
//!
//! use drone_cortexm::sv;
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

#![cfg_attr(feature = "std", allow(unreachable_code, unused_variables))]

mod switch;

pub use self::switch::{Switch, SwitchBackService, SwitchContextService};

use core::{intrinsics::unreachable, mem::size_of};

/// Generic supervisor.
pub trait Supervisor: Sized + 'static {
    /// Returns a pointer to the first service in the services array.
    fn first() -> *const Self;
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
pub unsafe fn sv_call<T: SvService>(service: &mut T, num: u8) {
    #[cfg(feature = "std")]
    return unimplemented!();
    if size_of::<T>() == 0 {
        llvm_asm!("
            svc $0
        "   :
            : "i"(num)
            :
            : "volatile"
        );
    } else {
        llvm_asm!("
            svc $0
        "   :
            : "i"(num), "{r12}"(service)
            :
            : "volatile"
        );
    }
}

/// This function is called by [`sv_handler`] for the supervisor service
/// `T`. Parameter `T` is based on the number `num` in the `SVC num`
/// instruction.
///
/// # Safety
///
/// This function should not be called directly.
pub unsafe extern "C" fn service_handler<T: SvService>(mut frame: *mut *mut u8) {
    if size_of::<T>() == 0 {
        unsafe { T::handler(&mut *(frame as *mut T)) };
    } else {
        unsafe {
            frame = frame.add(4); // Stacked R12
            T::handler(&mut *(*frame as *mut T));
        }
    }
}

/// `SV_CALL` exception handler for the supervisor `T`.
///
/// # Safety
///
/// This function should be called only by NVIC as part of a vector table.
#[naked]
pub unsafe extern "C" fn sv_handler<T: Supervisor>() {
    #[cfg(feature = "std")]
    return unimplemented!();
    llvm_asm!("
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
    unsafe { unreachable() };
}
