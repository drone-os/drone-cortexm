//! Drivers for core ARM Cortex-M peripherals.
//!
//! This module provides drivers for peripherals present in each Cortex-M
//! chip. It doesn't include device-specific drivers.

pub mod sys_tick;
pub mod timer;
