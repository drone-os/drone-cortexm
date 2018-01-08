use futures::executor::{self, Notify};
use thread::notify::irq::NOTIFY_IRQ;
use thread::prelude::*;

/// Platform thread token extensions.
pub trait PThreadToken<T: Thread, U: ThreadNumber> {
  /// Executes the future `f` within the thread.
  fn exec<F>(&self, f: F)
  where
    U: IrqNumber,
    F: IntoFuture<Item = (), Error = !>,
    F::Future: Send + 'static;
}

impl<T: Thread, U: ThreadNumber> PThreadToken<T, U> for ThreadToken<T, U> {
  #[cfg_attr(feature = "clippy", allow(while_let_loop))]
  fn exec<F>(&self, f: F)
  where
    U: IrqNumber,
    F: IntoFuture<Item = (), Error = !>,
    F::Future: Send + 'static,
  {
    let mut executor = executor::spawn(f.into_future());
    self.routine(move || loop {
      match executor.poll_future_notify(&NOTIFY_IRQ, U::IRQ_NUMBER) {
        Ok(Async::NotReady) => {}
        Ok(Async::Ready(())) => break,
      }
      yield;
    });
    NOTIFY_IRQ.notify(U::IRQ_NUMBER);
  }
}
