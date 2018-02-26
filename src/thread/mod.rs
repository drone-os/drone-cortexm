//! The vector table.
//!
//! # Configuration
//!
//! The vector table is configured by [`vtable!`] macro.
//!
//! ```rust
//! vtable! {
//!   /// The vector table.
//!   VectorTable;
//!   /// Array of threads.
//!   THREADS;
//!   ThreadLocal;
//!
//!   /// Non maskable interrupt.
//!   NMI;
//!   /// All classes of fault.
//!   HARD_FAULT;
//!   /// System tick timer.
//!   SYS_TICK;
//!   /// RCC global interrupt.
//!   5: rcc; // Give IRQ5 a name
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

pub mod irq;
pub mod prelude;
pub mod vtable;

mod control;
mod future;
mod notify;
mod request;
mod stream;

pub use self::control::ThreadControl;
pub use self::future::PltFuture;
pub use self::request::ThreadRequest;
pub use self::stream::{PltStream, StreamTrunkWait};
pub use drone_stm32_macros::interrupt;
