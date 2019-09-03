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
//! [`fib::new_stack`](crate::fib::new_stack),
//! [`fib::new_stack_unchecked`](crate::fib::new_stack_unchecked),
//! [`fib::new_stack_unprivileged`](crate::fib::new_stack_unprivileged),
//! [`fib::new_stack_unprivileged_unchecked`](crate::fib::new_stack_unprivileged_unchecked):
//!
//! ```
//! use drone_cortex_m::{fib, sv};
//!
//! use drone_cortex_m::sv::{SwitchBackService, SwitchContextService};
//!
//! // Stackful fibers need a supervisor.
//! sv! {
//!     pub struct Sv;
//!     static SERVICES;
//!
//!     // These services are required for stackful fibers.
//!     SwitchContextService;
//!     SwitchBackService;
//! }
//!
//! # fn main() {
//! // This is `impl Fiber<Input = bool, Yield = i32, Return = usize>`
//! let a = fib::new_stack::<Sv, bool, i32, usize, _>(0x800, |input, yielder| {
//!     // do some work and yield
//!     yielder.stack_yield(1);
//!     // do some work and yield
//!     yielder.stack_yield(2);
//!     // do some work and return
//!     3
//! });
//! # }
//! ```
//!
//! A stackful fiber can be attached to a thread with
//! [`token.add_stack(...)`](fib::ThrFiberStack::add_stack),
//! [`token.add_stack_unchecked(...)`](fib::ThrFiberStack::add_stack_unchecked),
//! [`token.add_stack_unprivileged(...)`](fib::ThrFiberStack::add_stack_unprivileged),
//! [`token.add_stack_unprivileged_unchecked(...)`](fib::ThrFiberStack::add_stack_unprivileged_unchecked).
//! Note that fibers that are directly attached to threads can't have input,
//! yield and return values other than `()`.
//!
//! ```
//! # #![feature(generators)]
//! # use drone_core::token::Token;
//! # use drone_cortex_m::{sv, sv::SwitchBackService, sv::SwitchContextService};
//! # static mut THREADS: [Thr; 1] = [Thr::new(0)];
//! # drone_core::thr!(use THREADS; struct Thr {} struct ThrLocal {});
//! # #[derive(Clone, Copy)] struct SysTick;
//! # struct Thrs { sys_tick: SysTick }
//! # sv!(pub struct Sv; static SERVICES; SwitchContextService; SwitchBackService;);
//! # unsafe impl Token for Thrs {
//! #     unsafe fn take() -> Self { Self { sys_tick: SysTick::take() } }
//! # }
//! # unsafe impl Token for SysTick {
//! #     unsafe fn take() -> Self { Self }
//! # }
//! # unsafe impl drone_core::thr::ThrToken for SysTick {
//! #     type Thr = Thr;
//! #     const THR_NUM: usize = 0;
//! # }
//! # impl drone_cortex_m::thr::ThrSv for SysTick {
//! #     type Sv = Sv;
//! # }
//! # fn main() {
//! #     let thr = unsafe { Thrs::take() };
//! use drone_cortex_m::thr::prelude::*;
//!
//! // this is `impl Fiber<Input = (), Yield = (), Return = ()>`
//! thr.sys_tick.add_stack(0x800, |yielder| {
//!     // do some work and yield
//!     yielder.stack_yield(());
//!     // do some work and yield
//!     yielder.stack_yield(());
//!     // do some work and return
//! });
//! # }
//! ```

mod stack;

#[doc(no_inline)]
pub use drone_core::fib::*;

pub use self::stack::{
    new_stack, new_stack_unchecked, new_stack_unprivileged, new_stack_unprivileged_unchecked,
    FiberStack, ThrFiberStack, Yielder,
};
