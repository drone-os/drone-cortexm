//! Core ARM Cortex-M interrupt mappings.

use crate::thr::{prelude::*, IntBundle};

macro_rules! exception {
  ($doc:expr, $name:ident,) => {
    #[doc = $doc]
    #[marker]
    pub trait $name<T: ThrTag>: ThrToken<T> {}
  };
}

macro_rules! int_bundle {
  ($name:ident, $number:expr, $doc:expr) => {
    #[doc = $doc]
    pub struct $name;

    impl IntBundle for $name {
      const BUNDLE_NUM: usize = $number;
    }
  };
}

exception! {
  "Non maskable interrupt.",
  IntNmi,
}

exception! {
  "All classes of fault.",
  IntHardFault,
}

exception! {
  "Memory management.",
  IntMemManage,
}

exception! {
  "Pre-fetch fault, memory access fault.",
  IntBusFault,
}

exception! {
  "Undefined instruction or illegal state.",
  IntUsageFault,
}

exception! {
  "System service call via SWI instruction.",
  IntSvCall,
}

exception! {
  "Monitor.",
  IntDebug,
}

exception! {
  "Pendable request for system service.",
  IntPendSv,
}

exception! {
  "System tick timer.",
  IntSysTick,
}

int_bundle!(IntBundle0, 0, "NVIC register bundle 0.");
int_bundle!(IntBundle1, 1, "NVIC register bundle 1.");
int_bundle!(IntBundle2, 2, "NVIC register bundle 2.");
int_bundle!(IntBundle3, 3, "NVIC register bundle 3.");
int_bundle!(IntBundle4, 4, "NVIC register bundle 4.");
int_bundle!(IntBundle5, 5, "NVIC register bundle 5.");
int_bundle!(IntBundle6, 6, "NVIC register bundle 6.");
int_bundle!(IntBundle7, 7, "NVIC register bundle 7.");
