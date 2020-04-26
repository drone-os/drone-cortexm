//! Core ARM Cortex-M register and exception mappings.
//!
//! This module provides mappings for registers and exceptions present in each
//! Cortex-M chip. It doesn't include device-specific mappings.
//!
//! **NOTE** A device-specific Drone crate may re-export this module with its
//! own additions, in which case it should be used instead.

pub mod periph;
pub mod reg;
pub mod thr;

/// Defines an index of core ARM Cortex-M register tokens.
#[doc(inline)]
pub use crate::cortexm_reg_tokens;
