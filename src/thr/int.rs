use core::task::Waker;

use crate::thr::prelude::*;
use crate::thr::wake::WakeInt;
use crate::thr::NvicBlock;

/// An interrupt token.
pub trait IntToken: ThrToken {
    /// NVIC block the interrupt belongs to.
    type NvicBlock: NvicBlock;

    /// The number of the interrupt.
    const INT_NUM: u16;

    /// Wakes up the thread.
    ///
    /// # Safety
    ///
    /// This function doesn't check for the interrupt token ownership.
    #[inline]
    unsafe fn wakeup_unchecked() {
        WakeInt::new(Self::INT_NUM).wakeup();
    }

    /// Returns a handle for waking up a thread.
    ///
    /// # Safety
    ///
    /// This function doesn't check for the interrupt token ownership.
    #[inline]
    unsafe fn waker_unchecked() -> Waker {
        WakeInt::new(Self::INT_NUM).to_waker()
    }
}
