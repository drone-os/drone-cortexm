//! The Threads module.
//!
//! **NOTE** This module documentation should be viewed as a continuation of
//! [the `drone_core` documentation](drone_core::thr).
//!
//! # Vector Table
//!
//! ```
//! # #![feature(const_fn)]
//! # #![feature(marker_trait_attr)]
//! # drone_cortex_m::thr::int!(pub trait Rcc: 5);
//! # drone_cortex_m::thr::int!(pub trait Adc1: 18);
//! # fn main() {}
//! use drone_cortex_m::{map::thr::*, thr};
//!
//! thr::vtable! {
//!     // Path to the thread object.
//!     use Thr;
//!
//!     /// The vector table.
//!     pub struct Vtable;
//!
//!     // Arguments to the vector table constructor. Contains at least the
//!     // `reset` handler.
//!     /// Explicit vector table handlers.
//!     pub struct Handlers;
//!
//!     /// Thread tokens.
//!     pub struct Thrs;
//!
//!     /// The array of threads.
//!     static THREADS;
//!
//!     // Exceptions start here.
//!
//!     // Define a normal thread for the non-interrupt exception NMI. This will
//!     // define a thread token `Nmi`, add `nmi` field to the `Thrs` index, and
//!     // reserve an item in the `THREADS` array.
//!     /// Non maskable interrupt.
//!     pub NMI;
//!     /// All classes of fault.
//!     pub HARD_FAULT;
//!     // Define a function handler for the non-interrupt exception SV_CALL. This
//!     // will only add a field to the `Handlers` structure.
//!     /// System service call.
//!     fn SV_CALL;
//!     /// System tick timer.
//!     pub SYS_TICK;
//!
//!     // Interrupts start here.
//!
//!     // Define a normal thread for the interrupt #5 with the name RCC. The name
//!     // can be arbitrary.
//!     /// RCC global interrupt.
//!     pub 5: RCC;
//!     // Define an external thread for the interrupt #18 with the name ADC1. This
//!     // will add a field to the `Handlers` structure, define a thread token
//!     // `Adc1`, add `adc1` field to the `Thrs` index, and reserve an item in the
//!     // `THREADS` array.
//!     /// ADC1 global interrupt.
//!     pub extern 18: ADC1;
//! }
//!
//! // See the `drone_core` documentation of `thr!` macro.
//! thr! {
//!     use THREADS;
//!     pub struct Thr {}
//!     pub struct ThrLocal {}
//! }
//!
//! // The reset handler should always be provided externally.
//! unsafe extern "C" fn reset() -> ! {
//!     loop {}
//! }
//!
//! // Define external handlers for the exceptions defined using `fn` or
//! // `extern` keyword.
//! unsafe extern "C" fn sv_call() {}
//! unsafe extern "C" fn adc1() {}
//!
//! // Define and export the actual vector table with all handlers attached.
//! #[no_mangle]
//! pub static VTABLE: Vtable = Vtable::new(Handlers { reset, sv_call, adc1 });
//! ```
//!
//! The list of all available non-interrupt exceptions:
//!
//! * `NMI` - Non maskable interrupt.
//! * `HARD_FAULT` - All classes of fault.
//! * `MEM_MANAGE` - Memory management.
//! * `BUS_FAULT` - Pre-fetch fault, memory access fault.
//! * `USAGE_FAULT` - Undefined instruction or illegal state.
//! * `SECURE_FAULT` - Security check violation. (Available when `security_extension` feature is enabled)
//! * `SV_CALL` - System service call via SWI instruction.
//! * `DEBUG` - Monitor.
//! * `PEND_SV` - Pendable request for system service.
//! * `SYS_TICK` - System tick timer.
//!
//! # Interrupt Mappings
//!
//! All available interrupts should be mapped with this macro:
//!
//! ```
//! # #![feature(marker_trait_attr)]
//! # fn main() {}
//! use drone_cortex_m::thr::int;
//!
//! int! {
//!     /// RCC global interrupt.
//!     pub trait RCC: 5;
//! }
//! ```

pub mod prelude;
pub mod vtable;

mod exec;
mod init;
mod nvic;
mod root;
mod wake;

#[doc(no_inline)]
pub use drone_core::thr::*;

/// Initializes the thread system and returns a set of thread tokens.
///
/// # Examples
///
/// ```no_run
/// # #![feature(const_fn)]
/// # drone_cortex_m::thr::vtable! {
/// #     use Thr;
/// #     struct Vtable;
/// #     struct Handlers;
/// #     struct Thrs;
/// #     static THREADS;
/// # }
/// # drone_cortex_m::thr! {
/// #     use THREADS;
/// #     struct Thr {}
/// #     struct ThrLocal {}
/// # }
/// # drone_cortex_m::cortex_m_reg_tokens!(struct Regs;);
/// # fn main() {
/// # let reg = unsafe { <Regs as drone_core::token::Token>::take() };
/// use drone_cortex_m::thr;
///
/// let (thr, _) = thr::init!(reg, Thrs);
/// # }
/// ```
#[doc(inline)]
pub use crate::thr_init as init;

/// Defines a vector table.
///
/// See [the module level documentation](self) for details.
#[doc(inline)]
pub use drone_cortex_m_macros::vtable;

/// Defines an interrupt.
///
/// See [the module level documentation](self) for details.
#[doc(inline)]
pub use drone_cortex_m_macros::int;

pub use self::{
    exec::{ExecOutput, ThrExec},
    init::{init, ThrInitPeriph},
    nvic::{NvicIabr, NvicIcer, NvicIcpr, NvicIser, NvicIspr, ThrNvic},
    root::{FutureRootExt, StreamRootExt, StreamRootWait},
};

use crate::sv::Supervisor;
use drone_core::{
    thr::{thread_resume, ThrToken},
    token::Token,
};

/// An interrupt token.
pub trait IntToken: ThrToken {
    /// NVIC bundle the interrupt belongs to.
    type Bundle: IntBundle;

    /// The number of the interrupt.
    const INT_NUM: usize;
}

/// NVIC registers bundle.
pub trait IntBundle {
    /// The number of NVIC bundle.
    const BUNDLE_NUM: usize;
}

/// A set of thread tokens.
///
/// # Safety
///
/// Must contain only thread tokens.
pub unsafe trait ThrTokens: Token {}

/// A trait to assign a supervisor to threads.
pub trait ThrSv: ThrToken {
    /// The supervisor.
    type Sv: Supervisor;
}

/// The thread handler function for a vector table.
///
/// # Safety
///
/// The function is not reentrant.
pub unsafe extern "C" fn thr_handler<T: ThrToken>() {
    thread_resume::<T>();
}
