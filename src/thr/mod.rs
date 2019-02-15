//! The vector table.
//!
//! # Configuration
//!
//! The vector table is configured by [`vtable!`] macro.
//!
//! ```rust
//! vtable! {
//!   /// The vector table.
//!   pub struct Vtable;
//!   pub struct Handlers;
//!   static THREADS;
//!   extern struct Thr;
//!
//!   /// Non maskable interrupt.
//!   pub NMI;
//!   /// All classes of fault.
//!   pub HARD_FAULT;
//!   /// System tick timer.
//!   pub SYS_TICK;
//!   /// RCC global interrupt.
//!   pub 5: rcc; // Give INT5 a name
//! }
//! ```
//!
//! # Preconfigured exceptions
//!
//! * `NMI` - Non maskable interrupt.
//! * `HARD_FAULT` - All classes of fault.
//! * `MEM_MANAGE` - Memory management.
//! * `BUS_FAULT` - Pre-fetch fault, memory access fault.
//! * `USAGE_FAULT` - Undefined instruction or illegal state.
//! * `SV_CALL` - System service call via SWI instruction.
//! * `DEBUG` - Monitor.
//! * `PEND_SV` - Pendable request for system service.
//! * `SYS_TICK` - System tick timer.
//!
//! [`vtable!`]: ../macro.vtable.html

pub mod prelude;
pub mod vtable;

mod config;
mod future;
mod int;
mod request;
mod stream;
mod wake;

pub use self::{
  config::ThrConfig,
  future::FutureExt,
  int::{IntBundle, IntToken},
  request::{ExecOutput, ThrRequest},
  stream::{StreamExt, StreamTrunkWait},
};
pub use drone_cortex_m_macros::thr_int as int;

use drone_core::{
  thr::{thread_resume, ThrTag, ThrToken},
  token::Tokens,
};

/// A thread handler function, which should be passed to hardware.
///
/// # Safety
///
/// Must be called only by hardware.
pub unsafe extern "C" fn thr_handler<T: ThrToken<U>, U: ThrTag>() {
  thread_resume::<T, U>();
}

/// A set of thread tokens.
///
/// # Safety
///
/// Must contain only thread tokens.
pub unsafe trait ThrTokens: Tokens {}
