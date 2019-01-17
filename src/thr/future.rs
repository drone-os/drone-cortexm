use crate::thr::wake::WakeTrunk;
use core::{future::Future, pin::Pin, task::Poll};

/// Future extensions.
pub trait FutureExt: Future {
  /// Blocks the current thread until the future is resolved.
  fn trunk_wait(self) -> Self::Output;
}

impl<T: Future> FutureExt for T {
  fn trunk_wait(mut self) -> Self::Output {
    let lw = WakeTrunk::new().into_local_waker();
    loop {
      match unsafe { Pin::new_unchecked(&mut self) }.poll(&lw) {
        Poll::Pending => WakeTrunk::wait(),
        Poll::Ready(value) => break value,
      }
    }
  }
}
