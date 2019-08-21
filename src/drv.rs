//! Core ARM Cortex-M drivers.

#[cfg(feature = "fpu")]
pub mod fpu;
pub mod sys_tick;
pub mod thr;
pub mod timer;
