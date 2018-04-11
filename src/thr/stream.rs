use core::iter::FusedIterator;
use cpu::wait_for_int;
use futures::prelude::*;
use thr::wake::WakeNop;

/// A stream combinator which converts an asynchronous stream to a **blocking
/// iterator**.
pub struct StreamTrunkWait<T: Stream> {
  stream: T,
  exhausted: bool,
}

/// Platform stream extensions.
pub trait StreamPlat: Stream {
  /// Creates an iterator which blocks the current thread until each item of
  /// this stream is resolved.
  fn trunk_wait(self) -> StreamTrunkWait<Self>
  where
    Self: Sized;
}

impl<T: Stream> StreamPlat for T {
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
      stream,
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
      match poll_stream(&mut self.stream) {
        Ok(Async::Pending) => wait_for_int(),
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

fn poll_stream<T: Stream>(stream: &mut T) -> Poll<Option<T::Item>, T::Error> {
  let waker = WakeNop::new().waker();
  let mut map = task::LocalMap::new();
  let mut cx = task::Context::without_spawn(&mut map, &waker);
  stream.poll_next(&mut cx)
}
