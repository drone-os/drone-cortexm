//! Core ARM Cortex-M drivers.

#[cfg(target_feature = "vfp2")]
pub mod fpu;
pub mod thr;
pub mod timer;
