use futures::executor::Notify;

pub(in thr) struct NotifyNop;

pub(in thr) const NOTIFY_NOP: &NotifyNop = &NotifyNop;

impl Notify for NotifyNop {
  #[inline(always)]
  fn notify(&self, _: usize) {}
}
