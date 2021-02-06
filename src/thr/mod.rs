//! The Threads module.
//!
//! **NOTE** This module documentation should be viewed as a continuation of
//! [the `drone_core` documentation](drone_core::thr).
//!
//! # Vector Table
//!
//! ```
//! # #![feature(const_fn_fn_ptr_basics)]
//! # #![feature(marker_trait_attr)]
//! # fn main() {}
//! use drone_cortexm::{map::thr::*, thr};
//!
//! thr::nvic! {
//!     // See the `drone_core` documentation of `thr::pool!` macro for details.
//!     pool => pub ThrPool;
//!
//!     // See the `drone_core` documentation of `thr::pool!` macro for details.
//!     thread => pub Thr {};
//!
//!     // See the `drone_core` documentation of `thr::pool!` macro for details.
//!     local => pub ThrLocal {};
//!
//!     // See the `drone_core` documentation of `thr::pool!` macro for details.
//!     index => pub Thrs;
//!
//!     /// The vector table type.
//!     vtable => pub Vtable;
//!
//!     /// Threads initialization token.
//!     init => pub ThrsInit;
//!
//!     // Threads configuration.
//!     threads => {
//!         // Threads for exceptions.
//!         exceptions => {
//!             // Define a regular thread for the NMI exception. This creates a thread token
//!             // structure `Nmi`, a field `nmi` in the `Thrs` structure, and an element in the
//!             // array of `Thr`.
//!             /// Non maskable interrupt.
//!             pub nmi;
//!             /// All classes of fault.
//!             pub hard_fault;
//!             // Define a naked handler for the SV_CALL exception. This inserts the function
//!             // `sv_call_handler` directly to the vector table.
//!             /// System service call.
//!             pub naked(sv_call_handler) sv_call;
//!             /// System tick timer.
//!             pub sys_tick;
//!         };
//!         // Threads for interrupts.
//!         interrupts => {
//!             // Define a regular thread for the interrupt #5 with name `rcc`.
//!             /// RCC global interrupt.
//!             5: pub rcc;
//!             // Define an outer thread for the interrupt #18 with name `adc1`. This creates a
//!             // thread token structure `Adc1`, a field `adc1` in the `Thrs` structure, and an
//!             // element in the array of `Thr`. But unlike a regular thread, this outer thread
//!             // uses a custom handler `adc1_handler`.
//!             /// ADC1 global interrupt.
//!             18: pub outer(adc1_handler) adc1;
//!         };
//!     };
//! }
//!
//! // The reset handler should always be provided externally.
//! unsafe extern "C" fn reset() -> ! {
//!     loop {}
//! }
//!
//! // Define external handlers for the exceptions defined using `fn` or
//! // `extern` keyword.
//! unsafe extern "C" fn sv_call_handler() {}
//! unsafe fn adc1_handler(_thr: &Thr) {}
//!
//! // Define and export the actual vector table with all handlers attached.
//! #[no_mangle]
//! pub static VTABLE: Vtable = Vtable::new(reset);
//! ```
//!
//! The list of all available non-interrupt exceptions:
//!
//! * `nmi` - Non maskable interrupt.
//! * `hard_fault` - All classes of fault.
//! * `mem_manage` - Memory management.
//! * `bus_fault` - Pre-fetch fault, memory access fault.
//! * `usage_fault` - Undefined instruction or illegal state.
//! * `secure_fault` - Security check violation. (Available when
//!   `security-extension` feature is enabled)
//! * `sv_call` - System service call via SWI instruction.
//! * `debug` - Monitor.
//! * `pend_sv` - Pendable request for system service.
//! * `sys_tick` - System tick timer.
//! ```

pub mod prelude;

mod exec;
mod init;
mod nvic;
mod root;
mod wake;

#[doc(no_inline)]
pub use drone_core::thr::*;

pub use self::{
    exec::{ExecOutput, ThrExec},
    init::{init, init_extended, ThrInitExtended, ThrsInitToken},
    nvic::{NvicBlock, NvicIabr, NvicIcer, NvicIcpr, NvicIser, NvicIspr, ThrNvic},
    root::{FutureRootExt, StreamRootExt, StreamRootWait},
};

/// Defines a thread pool driven by NVIC.
///
/// See [the module level documentation](self) for details.
#[doc(inline)]
pub use drone_cortexm_macros::thr_nvic as nvic;

use crate::sv::Supervisor;
use drone_core::thr::ThrToken;

/// An interrupt token.
pub trait IntToken: ThrToken {
    /// NVIC block the interrupt belongs to.
    type NvicBlock: NvicBlock;

    /// The number of the interrupt.
    const INT_NUM: usize;
}

/// A trait to assign a supervisor to threads.
pub trait ThrSv: ThrToken {
    /// The supervisor.
    type Sv: Supervisor;
}
