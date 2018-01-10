use core::ptr::write_volatile;
use futures::executor::Notify;

pub(in thread) struct NotifyIrq;

pub(in thread) const NOTIFY_IRQ: &NotifyIrq = &NotifyIrq;

const NVIC_STIR: usize = 0xE000_EF00;

impl Notify for NotifyIrq {
  #[inline(always)]
  fn notify(&self, number: usize) {
    unsafe { write_volatile(NVIC_STIR as *mut _, number) };
  }
}
