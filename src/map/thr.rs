//! Core ARM Cortex-M interrupt mappings.

use crate::thr::NvicBlock;

macro_rules! nvic_block {
    ($name:ident, $number:expr, $doc:expr) => {
        #[doc = $doc]
        pub struct $name;

        impl NvicBlock for $name {
            const BLOCK_NUM: usize = $number;
        }
    };
}

macro_rules! nvic_block_cortexm33 {
    ($name:ident, $number:expr, $doc:expr) => {
        #[cfg(any(
            drone_cortexm = "cortexm33_r0p2",
            drone_cortexm = "cortexm33_r0p3",
            drone_cortexm = "cortexm33_r0p4",
            drone_cortexm = "cortexm33f_r0p2",
            drone_cortexm = "cortexm33f_r0p3",
            drone_cortexm = "cortexm33f_r0p4",
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
nvic_block_cortexm33!(NvicBlock8, 8, "NVIC register block 8.");
nvic_block_cortexm33!(NvicBlock9, 9, "NVIC register block 9.");
nvic_block_cortexm33!(NvicBlock10, 10, "NVIC register block 10.");
nvic_block_cortexm33!(NvicBlock11, 11, "NVIC register block 11.");
nvic_block_cortexm33!(NvicBlock12, 12, "NVIC register block 12.");
nvic_block_cortexm33!(NvicBlock13, 13, "NVIC register block 13.");
nvic_block_cortexm33!(NvicBlock14, 14, "NVIC register block 14.");
nvic_block_cortexm33!(NvicBlock15, 15, "NVIC register block 15.");
