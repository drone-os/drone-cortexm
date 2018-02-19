use cpu::wait_for_interrupt;
use futures::executor;
use thread::notify::nop::NOTIFY_NOP;

/// Platform future extensions.
pub trait PltFuture: Future {
  /// Blocks the current thread until the future is resolved.
  fn wait(self) -> Result<Self::Item, Self::Error>;
}

impl<T: Future> PltFuture for T {
  fn wait(self) -> Result<Self::Item, Self::Error> {
    let mut executor = executor::spawn(self);
    loop {
      match executor.poll_future_notify(&NOTIFY_NOP, 0) {
        Ok(Async::NotReady) => wait_for_interrupt(),
        Ok(Async::Ready(value)) => break Ok(value),
        Err(err) => break Err(err),
      }
    }
  }
}
