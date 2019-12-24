use crate::{
    fib,
    thr::{prelude::*, wake::WakeInt},
};
use core::{
    fmt::Display,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// Execution methods for interrupt tokens.
pub trait ThrExec: IntToken {
    /// Adds an executor for the future `fut` to the fiber chain and triggers
    /// the thread immediately.
    fn exec<F, O: ExecOutput>(self, fut: F)
    where
        F: Future<Output = O> + Send + 'static;

    /// Adds an executor for the future `fut` to the fiber chain.
    ///
    /// The future `fut` will start polling on the next thread wake-up.
    fn add_exec<F, O: ExecOutput>(self, fut: F)
    where
        F: Future<Output = O> + Send + 'static;

    /// Generates the interrupt.
    ///
    /// This method will wake-up the thread.
    fn trigger(self);
}

/// A trait for implementing arbitrary output types for futures passed to
/// [`ThrExec::exec`] and [`ThrExec::add_exec`].
pub trait ExecOutput: Sized + Send {
    /// The return type of [`ExecOutput::terminate`]. Should be either `()` or
    /// `!`.
    type Terminate;

    /// A result handler for an executor. The returned value will not be used,
    /// so the only useful types are `()` and `!`. The handler may choose to
    /// panic on an erroneous value.
    fn terminate(self) -> Self::Terminate;
}

impl<T: IntToken> ThrExec for T {
    #[inline]
    fn exec<F, O: ExecOutput>(self, fut: F)
    where
        F: Future<Output = O> + Send + 'static,
    {
        self.add_exec(fut);
        self.trigger();
    }

    fn add_exec<F, O: ExecOutput>(self, mut fut: F)
    where
        F: Future<Output = O> + Send + 'static,
    {
        fn poll<F: Future>(fut: Pin<&mut F>, int_num: usize) -> Poll<F::Output> {
            let waker = WakeInt::new(int_num).to_waker();
            let mut cx = Context::from_waker(&waker);
            fut.poll(&mut cx)
        }
        self.add_fn(move || match poll(unsafe { Pin::new_unchecked(&mut fut) }, Self::INT_NUM) {
            Poll::Pending => fib::Yielded(()),
            Poll::Ready(output) => {
                output.terminate();
                fib::Complete(())
            }
        });
    }

    #[inline]
    fn trigger(self) {
        WakeInt::new(Self::INT_NUM).wake();
    }
}

impl ExecOutput for () {
    type Terminate = ();

    #[inline]
    fn terminate(self) {}
}

impl<E: Send + Display> ExecOutput for Result<(), E> {
    type Terminate = ();

    #[inline]
    fn terminate(self) {
        match self {
            Ok(()) => {}
            Err(err) => terminate_err(err),
        }
    }
}

impl ExecOutput for ! {
    type Terminate = !;

    #[inline]
    fn terminate(self) -> ! {
        match self {}
    }
}

impl<E: Send + Display> ExecOutput for Result<!, E> {
    type Terminate = !;

    #[inline]
    fn terminate(self) -> ! {
        let Err(err) = self;
        terminate_err(err);
    }
}

fn terminate_err<E: Display>(err: E) -> ! {
    panic!("root future error: {}", err);
}
