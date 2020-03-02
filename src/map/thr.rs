//! Core ARM Cortex-M exception mappings.

use crate::thr::{prelude::*, NvicBlock};

macro_rules! exception {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[marker]
        pub trait $name: ThrToken {}
    };
}

exception!(IntNmi, "Non maskable interrupt.");
exception!(IntHardFault, "All classes of fault.");
exception!(IntMemManage, "Memory management.");
exception!(IntBusFault, "Pre-fetch fault, memory access fault.");
exception!(IntUsageFault, "Undefined instruction or illegal state.");
#[cfg(all(
    feature = "security-extension",
    any(
        cortex_m_core = "cortex_m33_r0p2",
        cortex_m_core = "cortex_m33_r0p3",
        cortex_m_core = "cortex_m33_r0p4",
        cortex_m_core = "cortex_m33f_r0p2",
        cortex_m_core = "cortex_m33f_r0p3",
        cortex_m_core = "cortex_m33f_r0p4",
    )
))]
exception!(IntSecureFault, "Security check violation.");
exception!(IntSvCall, "System service call via SWI instruction.");
exception!(IntDebug, "Monitor.");
exception!(IntPendSv, "Pendable request for system service.");
exception!(IntSysTick, "System tick timer.");

macro_rules! nvic_block {
    ($name:ident, $number:expr, $doc:expr) => {
        #[doc = $doc]
        pub struct $name;

        impl NvicBlock for $name {
            const BLOCK_NUM: usize = $number;
        }
    };
}

macro_rules! nvic_block_cortex_m33 {
    ($name:ident, $number:expr, $doc:expr) => {
        #[cfg(any(
            cortex_m_core = "cortex_m33_r0p2",
            cortex_m_core = "cortex_m33_r0p3",
            cortex_m_core = "cortex_m33_r0p4",
            cortex_m_core = "cortex_m33f_r0p2",
            cortex_m_core = "cortex_m33f_r0p3",
            cortex_m_core = "cortex_m33f_r0p4",
        ))]
        nvic_block!($name, $number, $doc);
    };
}

nvic_block!(NvicBlock0, 0, "NVIC register block 0.");
nvic_block!(NvicBlock1, 1, "NVIC register block 1.");
nvic_block!(NvicBlock2, 2, "NVIC register block 2.");
nvic_block!(NvicBlock3, 3, "NVIC register block 3.");
nvic_block!(NvicBlock4, 4, "NVIC register block 4.");
nvic_block!(NvicBlock5, 5, "NVIC register block 5.");
nvic_block!(NvicBlock6, 6, "NVIC register block 6.");
nvic_block!(NvicBlock7, 7, "NVIC register block 7.");
nvic_block_cortex_m33!(NvicBlock8, 8, "NVIC register block 8.");
nvic_block_cortex_m33!(NvicBlock9, 9, "NVIC register block 9.");
nvic_block_cortex_m33!(NvicBlock10, 10, "NVIC register block 10.");
nvic_block_cortex_m33!(NvicBlock11, 11, "NVIC register block 11.");
nvic_block_cortex_m33!(NvicBlock12, 12, "NVIC register block 12.");
nvic_block_cortex_m33!(NvicBlock13, 13, "NVIC register block 13.");
nvic_block_cortex_m33!(NvicBlock14, 14, "NVIC register block 14.");
nvic_block_cortex_m33!(NvicBlock15, 15, "NVIC register block 15.");
