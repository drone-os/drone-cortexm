use cpu::wait_for_interrupt;
use futures::executor;
use thread::notify::nop::NOTIFY_NOP;

/// Platform future extensions.
pub trait FuturePlfm: Future {
  /// Blocks the current thread until the future is resolved.
  fn trunk_wait(self) -> Result<Self::Item, Self::Error>;
}

impl<T: Future> FuturePlfm for T {
  fn trunk_wait(self) -> Result<Self::Item, Self::Error> {
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
