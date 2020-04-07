//! Drivers for core ARM Cortex-M peripherals.
//!
//! This module provides drivers for peripherals present in each Cortex-M
//! chip. It doesn't include device-specific drivers.
//!
//! **NOTE** A device-specific Drone crate may re-export this module with its
//! own additions, in which case it should be used instead.

#[cfg(feature = "floating-point-unit")]
pub mod fpu;
pub mod sys_tick;
pub mod timer;
