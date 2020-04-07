//! Core ARM Cortex-M peripheral mappings.

#[cfg(all(
    feature = "floating-point-unit",
    any(
        cortex_m_core = "cortex_m4f_r0p0",
        cortex_m_core = "cortex_m4f_r0p1",
        cortex_m_core = "cortex_m33f_r0p2",
        cortex_m_core = "cortex_m33f_r0p3",
        cortex_m_core = "cortex_m33f_r0p4"
    )
))]
pub mod fpu;
pub mod sys_tick;
