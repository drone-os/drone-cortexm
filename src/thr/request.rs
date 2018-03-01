use fib;
use futures::executor::{self, Notify};
use thr::notify::int::NOTIFY_INT;
use thr::prelude::*;

/// Thread execution requests.
pub trait ThrRequest<T: ThrTrigger>: IntToken<T> {
  /// Executes the future `f` within the thread.
  fn exec<F>(&self, f: F)
  where
    F: IntoFuture<Item = (), Error = !>,
    F::Future: Send + 'static;

  /// Requests the interrupt.
  #[inline(always)]
  fn trigger(&self) {
    NOTIFY_INT.notify(Self::INT_NUM);
  }
}

impl<T: ThrTrigger, U: IntToken<T>> ThrRequest<T> for U {
  #[cfg_attr(feature = "clippy", allow(while_let_loop))]
  fn exec<F>(&self, f: F)
  where
    F: IntoFuture<Item = (), Error = !>,
    F::Future: Send + 'static,
  {
    let mut executor = executor::spawn(f.into_future());
    fib::spawn(self, move || loop {
      match executor.poll_future_notify(&NOTIFY_INT, U::INT_NUM) {
        Ok(Async::NotReady) => {}
        Ok(Async::Ready(())) => break,
      }
      yield;
    });
    self.trigger();
  }
}
