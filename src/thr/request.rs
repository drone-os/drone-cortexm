use fib;
use futures::prelude::*;
use thr::prelude::*;
use thr::wake::WakeInt;

/// Thread execution requests.
pub trait ThrRequest<T: ThrTrigger>: IntToken<T> {
  /// Executes the future `f` within the thread.
  fn exec<F>(&self, f: F)
  where
    F: IntoFuture<Item = (), Error = !>,
    F::Future: Send + 'static;

  /// Requests the interrupt.
  #[inline(always)]
  fn trigger(&self) {
    WakeInt::new(Self::INT_NUM).wake();
  }
}

impl<T: ThrTrigger, U: IntToken<T>> ThrRequest<T> for U {
  #[cfg_attr(feature = "cargo-clippy", allow(while_let_loop))]
  fn exec<F>(&self, f: F)
  where
    F: IntoFuture<Item = (), Error = !>,
    F::Future: Send + 'static,
  {
    let mut fut = f.into_future();
    fib::add(self, move || loop {
      match poll_future(&mut fut, U::INT_NUM) {
        Ok(Async::Pending) => {}
        Ok(Async::Ready(())) => break,
      }
      yield;
    });
    self.trigger();
  }
}

fn poll_future<F>(fut: &mut F, int_num: usize) -> Poll<(), !>
where
  F: Future<Item = (), Error = !>,
{
  let waker = WakeInt::new(int_num).into_waker();
  let mut map = task::LocalMap::new();
  let mut cx = task::Context::without_spawn(&mut map, &waker);
  fut.poll(&mut cx)
}
