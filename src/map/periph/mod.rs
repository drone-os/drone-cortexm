//! Core ARM Cortex-M peripheral mappings.

#[cfg(feature = "floating-point-unit")]
pub mod fpu;
pub mod sys_tick;
