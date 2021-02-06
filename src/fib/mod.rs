//! The Fibers module.
//!
//! **NOTE** This module documentation should be viewed as a continuation of
//! [the `drone_core` documentation](drone_core::fib).
//!
//! # Stackful Fibers
//!
//! This module implements stackful fibers that are similar to native threads in
//! the Rust stdlib. They can run synchronous code inside and yield with a
//! blocking call. A stackful fiber can be created with
//! [`fib::new_proc`](crate::fib::new_proc),
//! [`fib::new_proc_unchecked`](crate::fib::new_proc_unchecked),
//! [`fib::new_proc_unprivileged`](crate::fib::new_proc_unprivileged),
//! [`fib::new_proc_unprivileged_unchecked`](crate::fib::new_proc_unprivileged_unchecked):
//!
//! ```
//! # #![feature(const_fn_fn_ptr_basics)]
//! # #![feature(naked_functions)]
//! use drone_cortexm::{fib, sv};
//!
//! use drone_cortexm::sv::{SwitchBackService, SwitchContextService};
//!
//! // Stackful fibers need a supervisor.
//! sv::pool! {
//!     pool => Services;
//!     supervisor => pub Sv;
//!     services => {
//!         // These services are required for stackful fibers.
//!         SwitchContextService;
//!         SwitchBackService;
//!     }
//! }
//!
//! # fn main() {
//! // This is `impl Fiber<Input = bool, Yield = i32, Return = usize>`
//! let a = fib::new_proc::<Sv, bool, i32, usize, _>(0x800, |input, yielder| {
//!     // do some work and yield
//!     yielder.proc_yield(1);
//!     // do some work and yield
//!     yielder.proc_yield(2);
//!     // do some work and return
//!     3
//! });
//! # }
//! ```
//!
//! A stackful fiber can be attached to a thread with
//! [`token.add_proc(...)`](ThrFiberProc::add_proc),
//! [`token.add_proc_unchecked(...)`](ThrFiberProc::add_proc_unchecked),
//! [`token.add_proc_unprivileged(...)`](ThrFiberProc::add_proc_unprivileged),
//! [`token.add_proc_unprivileged_unchecked(...)`](ThrFiberProc::add_proc_unprivileged_unchecked).
//! Note that fibers that are directly attached to threads can't have input,
//! yield and return values other than `()`.
//!
//! ```
//! # #![feature(const_fn_fn_ptr_basics)]
//! # #![feature(generators)]
//! # #![feature(naked_functions)]
//! # use drone_core::token::Token;
//! # use drone_cortexm::{sv, sv::SwitchBackService, sv::SwitchContextService};
//! # drone_core::thr::pool! {
//! #     pool => ThrPool;
//! #     thread => Thr {};
//! #     local => ThrLocal {};
//! #     index => Thrs;
//! #     threads => { sys_tick };
//! # }
//! # sv::pool! {
//! #     pool => Services;
//! #     supervisor => pub Sv;
//! #     services => { SwitchContextService; SwitchBackService };
//! # }
//! # impl drone_cortexm::thr::ThrSv for SysTick {
//! #     type Sv = Sv;
//! # }
//! # fn main() {
//! #     let thr = unsafe { Thrs::take() };
//! use drone_cortexm::thr::prelude::*;
//!
//! // this is `impl Fiber<Input = (), Yield = (), Return = ()>`
//! thr.sys_tick.add_proc(0x800, |yielder| {
//!     // do some work and yield
//!     yielder.proc_yield(());
//!     // do some work and yield
//!     yielder.proc_yield(());
//!     // do some work and return
//! });
//! # }
//! ```

mod proc;

#[doc(no_inline)]
pub use drone_core::fib::*;

pub use self::proc::{
    new_proc, new_proc_unchecked, new_proc_unprivileged, new_proc_unprivileged_unchecked,
    FiberProc, ThrFiberProc, Yielder,
};
