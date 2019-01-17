use crate::thr::{prelude::*, wake::WakeInt};
use core::{future::Future, pin::Pin, task::Poll};

/// Thread execution requests.
pub trait ThrRequest<T: ThrTrigger>: IntToken<T> {
  /// Executes the future `f` within the thread.
  fn exec<F>(self, f: F)
  where
    T: ThrAttach,
    F: Future<Output = ()> + Send + 'static;

  /// Requests the interrupt.
  #[inline]
  fn trigger(self) {
    WakeInt::new(Self::INT_NUM).wake();
  }
}

impl<T: ThrTrigger, U: IntToken<T>> ThrRequest<T> for U {
  #[allow(clippy::while_let_loop)]
  fn exec<F>(self, mut fut: F)
  where
    T: ThrAttach,
    F: Future<Output = ()> + Send + 'static,
  {
    fn poll<F: Future>(fut: Pin<&mut F>, int_num: usize) -> Poll<F::Output> {
      let lw = WakeInt::new(int_num).into_local_waker();
      fut.poll(&lw)
    }
    self.add(move || loop {
      match poll(unsafe { Pin::new_unchecked(&mut fut) }, Self::INT_NUM) {
        Poll::Pending => yield,
        Poll::Ready(()) => break,
      }
    });
    self.trigger();
  }
}
