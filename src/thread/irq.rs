//! Interrupt mappings.

#[allow(unused_imports)]
use super::interrupt;
#[allow(unused_imports)]
use thread::prelude::*;

include!(concat!(env!("OUT_DIR"), "/svd_irq.rs"));

/// NVIC register bundle.
pub trait IrqBundle {
  /// A number of NVIC register.
  const BUNDLE_NUM: usize;
}

/// An interrupt.
pub trait IrqToken<T: ThdTag>: ThdToken<T> {
  /// A number of NVIC register.
  type Bundle: IrqBundle;

  /// An interrupt position within the vector table.
  const IRQ_NUM: usize;
}

macro_rules! exception {
  ($name:ident, $doc:expr) => {
    #[doc = $doc]
    pub trait $name<T: ThdTag>: ThdToken<T> {}
  }
}

macro_rules! irq_bundle {
  ($name:ident, $number:expr, $doc:expr) => {
    #[doc = $doc]
    pub struct $name;

    impl IrqBundle for $name {
      const BUNDLE_NUM: usize = $number;
    }
  }
}

exception!(IrqNmi, "Non maskable interrupt.");
exception!(IrqHardFault, "All classes of fault.");
exception!(IrqMemManage, "Memory management.");
exception!(IrqBusFault, "Pre-fetch fault, memory access fault.");
exception!(IrqUsageFault, "Undefined instruction or illegal state.");
exception!(IrqSvCall, "System service call via SWI instruction.");
exception!(IrqDebug, "Monitor.");
exception!(IrqPendSv, "Pendable request for system service.");
exception!(IrqSysTick, "System tick timer.");

irq_bundle!(IrqBundle0, 0, "NVIC register bundle 0.");
irq_bundle!(IrqBundle1, 1, "NVIC register bundle 1.");
irq_bundle!(IrqBundle2, 2, "NVIC register bundle 2.");
irq_bundle!(IrqBundle3, 3, "NVIC register bundle 3.");
irq_bundle!(IrqBundle4, 4, "NVIC register bundle 4.");
irq_bundle!(IrqBundle5, 5, "NVIC register bundle 5.");
irq_bundle!(IrqBundle6, 6, "NVIC register bundle 6.");
irq_bundle!(IrqBundle7, 7, "NVIC register bundle 7.");
