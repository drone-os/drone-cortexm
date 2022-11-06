//! Core ARM Cortex-M peripheral mappings.

#[cfg(feature = "floating-point-unit")]
pub mod fpu;
#[cfg(feature = "memory-protection-unit")]
pub mod mpu;
pub mod sys_tick;
pub mod thr;
