use futures::executor::Notify;

pub(in thread) struct NotifyNop;

pub(in thread) const NOTIFY_NOP: &NotifyNop = &NotifyNop;

impl Notify for NotifyNop {
  #[inline(always)]
  fn notify(&self, _: usize) {}
}
