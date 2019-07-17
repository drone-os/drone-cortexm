//! Core ARM Cortex-M drivers.

#[cfg(target_feature = "vfp2")]
pub mod fpu;
pub mod sys_tick;
pub mod thr;

pub use drone_core::drv::*;
