use core::{
    ptr::write_volatile,
    task::{RawWaker, RawWakerVTable, Waker},
};

const NVIC_STIR: usize = 0xE000_EF00;

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake, drop);

#[repr(transparent)]
pub struct WakeInt(u16);

impl WakeInt {
    pub fn new(int_num: u16) -> Self {
        Self(int_num)
    }

    pub fn wakeup(&self) {
        unsafe { write_volatile(NVIC_STIR as *mut usize, self.0 as usize) };
    }

    pub fn to_waker(&self) -> Waker {
        unsafe { Waker::from_raw(self.to_raw_waker()) }
    }

    fn to_raw_waker(&self) -> RawWaker {
        RawWaker::new(self.0 as *const (), &VTABLE)
    }
}

unsafe fn clone(data: *const ()) -> RawWaker {
    WakeInt::new(data as u16).to_raw_waker()
}

unsafe fn wake(data: *const ()) {
    WakeInt::new(data as u16).wakeup();
}
