use fiber;
use futures::executor::{self, Notify};
use thread::notify::irq::NOTIFY_IRQ;
use thread::prelude::*;

/// Thread execution requests.
pub trait ThdRequest<T: ThdTrigger>: IrqToken<T> {
  /// Executes the future `f` within the thread.
  fn exec<F>(&self, f: F)
  where
    F: IntoFuture<Item = (), Error = !>,
    F::Future: Send + 'static;

  /// Requests the interrupt.
  #[inline(always)]
  fn trigger(&self) {
    NOTIFY_IRQ.notify(Self::IRQ_NUM);
  }
}

impl<T: ThdTrigger, U: IrqToken<T>> ThdRequest<T> for U {
  #[cfg_attr(feature = "clippy", allow(while_let_loop))]
  fn exec<F>(&self, f: F)
  where
    F: IntoFuture<Item = (), Error = !>,
    F::Future: Send + 'static,
  {
    let mut executor = executor::spawn(f.into_future());
    fiber::spawn(self, move || loop {
      match executor.poll_future_notify(&NOTIFY_IRQ, U::IRQ_NUM) {
        Ok(Async::NotReady) => {}
        Ok(Async::Ready(())) => break,
      }
      yield;
    });
    self.trigger();
  }
}
