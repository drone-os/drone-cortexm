use crate::platform;
use core::ptr;
use core::task::{RawWaker, RawWakerVTable, Waker};

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake, drop);

pub struct WakeRoot(());

#[allow(clippy::unused_self)]
impl WakeRoot {
    pub fn new() -> Self {
        Self(())
    }

    pub fn wait() {
        platform::wait_for_event();
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
        drone_cortexm = "cortexm3_r0p0",
        drone_cortexm = "cortexm3_r1p0",
        drone_cortexm = "cortexm3_r1p1",
        drone_cortexm = "cortexm3_r2p0",
    ))]
    platform::send_event();
}
