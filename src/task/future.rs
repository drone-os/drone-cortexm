use super::NOP_NOTIFY;
use futures::executor;
use mcu::wait_for_interrupt;

/// Drone future.
pub trait DroneFuture: Future {
  /// Blocks the current thread until the future is resolved.
  fn wait(self) -> Result<Self::Item, Self::Error>;
}

impl<T: Future> DroneFuture for T {
  fn wait(self) -> Result<Self::Item, Self::Error> {
    let mut executor = executor::spawn(self);
    loop {
      match executor.poll_future_notify(&&NOP_NOTIFY, 0) {
        Ok(Async::NotReady) => wait_for_interrupt(),
        Ok(Async::Ready(value)) => break Ok(value),
        Err(err) => break Err(err),
      }
    }
  }
}
