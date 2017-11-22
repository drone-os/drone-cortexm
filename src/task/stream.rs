use super::NOP_NOTIFY;
use futures::executor;
use mcu::wait_for_interrupt;

/// A stream combinator which converts an asynchronous stream to a **blocking
/// iterator**.
pub struct StreamWait<T> {
  executor: executor::Spawn<T>,
}

/// Drone stream.
pub trait DroneStream: Stream {
  /// Creates an iterator which blocks the current thread until each item of
  /// this stream is resolved.
  fn wait(self) -> StreamWait<Self>
  where
    Self: Sized;
}

impl<T> DroneStream for T
where
  T: Stream,
{
  fn wait(self) -> StreamWait<Self>
  where
    Self: Sized,
  {
    StreamWait::new(self)
  }
}

impl<T> StreamWait<T>
where
  T: Stream,
{
  fn new(stream: T) -> Self {
    let executor = executor::spawn(stream);
    Self { executor }
  }
}

impl<T> Iterator for StreamWait<T>
where
  T: Stream,
{
  type Item = Result<T::Item, T::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.executor.poll_stream_notify(&&NOP_NOTIFY, 0) {
        Ok(Async::NotReady) => wait_for_interrupt(),
        Ok(Async::Ready(ready)) => break ready.map(Ok),
        Err(err) => break Some(Err(err)),
      }
    }
  }
}
