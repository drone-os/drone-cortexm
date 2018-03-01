use core::ptr::write_volatile;
use futures::executor::Notify;

pub(in thr) struct NotifyInt;

pub(in thr) const NOTIFY_INT: &NotifyInt = &NotifyInt;

const NVIC_STIR: usize = 0xE000_EF00;

impl Notify for NotifyInt {
  #[inline(always)]
  fn notify(&self, number: usize) {
    unsafe { write_volatile(NVIC_STIR as *mut _, number) };
  }
}
