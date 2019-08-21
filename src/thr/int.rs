use crate::thr::prelude::*;

/// NVIC register bundle.
pub trait IntBundle {
    /// A number of NVIC register.
    const BUNDLE_NUM: usize;
}

/// An interrupt.
pub trait IntToken: ThrToken {
    /// A number of NVIC register.
    type Bundle: IntBundle;

    /// An interrupt position within the vector table.
    const INT_NUM: usize;
}
