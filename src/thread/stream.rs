use core::iter::FusedIterator;
use cpu::wait_for_interrupt;
use futures::executor;
use thread::notify::nop::NOTIFY_NOP;

/// A stream combinator which converts an asynchronous stream to a **blocking
/// iterator**.
pub struct StreamTrunkWait<T: Stream> {
  executor: executor::Spawn<T>,
  exhausted: bool,
}

/// Platform stream extensions.
pub trait PltStream: Stream {
  /// Creates an iterator which blocks the current thread until each item of
  /// this stream is resolved.
  fn trunk_wait(self) -> StreamTrunkWait<Self>
  where
    Self: Sized;
}

impl<T: Stream> PltStream for T {
  #[inline(always)]
  fn trunk_wait(self) -> StreamTrunkWait<Self>
  where
    Self: Sized,
  {
    StreamTrunkWait::new(self)
  }
}

impl<T: Stream> StreamTrunkWait<T> {
  #[inline(always)]
  fn new(stream: T) -> Self {
    Self {
      executor: executor::spawn(stream),
      exhausted: false,
    }
  }
}

impl<T: Stream> Iterator for StreamTrunkWait<T> {
  type Item = Result<T::Item, T::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.exhausted {
      return None;
    }
    loop {
      match self.executor.poll_stream_notify(&NOTIFY_NOP, 0) {
        Ok(Async::NotReady) => wait_for_interrupt(),
        Ok(Async::Ready(Some(value))) => break Some(Ok(value)),
        Ok(Async::Ready(None)) => {
          self.exhausted = true;
          break None;
        }
        Err(err) => {
          self.exhausted = true;
          break Some(Err(err));
        }
      }
    }
  }
}

impl<T: Stream> FusedIterator for StreamTrunkWait<T> {}
