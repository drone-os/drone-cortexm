use crate::processor;
use core::{
    ptr,
    task::{RawWaker, RawWakerVTable, Waker},
};

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake, drop);

pub struct WakeRoot(());

impl WakeRoot {
    pub fn new() -> Self {
        Self(())
    }

    pub fn wait() {
        processor::wait_for_event();
    }

    pub fn to_waker(&self) -> Waker {
        unsafe { Waker::from_raw(raw_waker()) }
    }
}

fn raw_waker() -> RawWaker {
    RawWaker::new(ptr::null(), &VTABLE)
}

unsafe fn clone(_data: *const ()) -> RawWaker {
    raw_waker()
}

unsafe fn wake(_data: *const ()) {
    // In r0p0, r1p0, r1p1 and r2p0 versions of Cortex-M3 the event register is not
    // set for the exception entry, exception exit or debug events.
    #[cfg(any(
        feature = "cortex_m3_r0p0",
        feature = "cortex_m3_r1p0",
        feature = "cortex_m3_r1p1",
        feature = "cortex_m3_r2p0",
    ))]
    processor::send_event();
}
