//! Core ARM Cortex-M exception mappings.

use crate::thr::{prelude::*, IntBundle};

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
exception!(IntSvCall, "System service call via SWI instruction.");
exception!(IntDebug, "Monitor.");
exception!(IntPendSv, "Pendable request for system service.");
exception!(IntSysTick, "System tick timer.");

macro_rules! int_bundle {
    ($name:ident, $number:expr, $doc:expr) => {
        #[doc = $doc]
        pub struct $name;

        impl IntBundle for $name {
            const BUNDLE_NUM: usize = $number;
        }
    };
}

int_bundle!(IntBundle0, 0, "NVIC register bundle 0.");
int_bundle!(IntBundle1, 1, "NVIC register bundle 1.");
int_bundle!(IntBundle2, 2, "NVIC register bundle 2.");
int_bundle!(IntBundle3, 3, "NVIC register bundle 3.");
int_bundle!(IntBundle4, 4, "NVIC register bundle 4.");
int_bundle!(IntBundle5, 5, "NVIC register bundle 5.");
int_bundle!(IntBundle6, 6, "NVIC register bundle 6.");
int_bundle!(IntBundle7, 7, "NVIC register bundle 7.");
