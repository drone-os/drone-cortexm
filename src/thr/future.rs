use crate::thr::wake::WakeTrunk;
use core::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};

/// Future extensions.
pub trait FutureExt: Future {
  /// Blocks the current thread until the future is resolved.
  fn trunk_wait(self) -> Self::Output;
}

impl<T: Future> FutureExt for T {
  fn trunk_wait(mut self) -> Self::Output {
    let waker = WakeTrunk::new().to_waker();
    let mut cx = Context::from_waker(&waker);
    loop {
      match unsafe { Pin::new_unchecked(&mut self) }.poll(&mut cx) {
        Poll::Pending => WakeTrunk::wait(),
        Poll::Ready(value) => break value,
      }
    }
  }
}
