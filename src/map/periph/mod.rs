//! Core ARM Cortex-M peripheral mappings.

#[cfg(feature = "floating-point-unit")]
pub mod fpu;
#[cfg(feature = "memory-protection-unit")]
pub mod mpu;
pub mod sys_tick;
pub mod thr;

#[cfg(feature = "floating-point-unit")]
pub use self::fpu::Fpu;
#[cfg(feature = "memory-protection-unit")]
pub use self::mpu::Mpu;
pub use self::sys_tick::SysTick;
pub use self::thr::Thr;
