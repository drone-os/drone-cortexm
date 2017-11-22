//! Timers and watchdogs.

pub mod sys_tick;

pub use self::sys_tick::Driver as SysTick;
