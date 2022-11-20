//! Drivers for core ARM Cortex-M peripherals.
//!
//! This module provides drivers for peripherals present in each Cortex-M core.
//! It doesn't include MCU-specific drivers.

pub mod sys_tick;
pub mod timer;

pub use self::sys_tick::SysTick;
pub use self::timer::Timer;
