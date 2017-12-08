//! Timers and watchdogs.

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
mod sys_tick;

pub use self::sys_tick::SysTick;
