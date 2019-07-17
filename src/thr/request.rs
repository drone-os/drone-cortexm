use crate::thr::{prelude::*, wake::WakeInt};
use core::{
    fmt::Display,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// Thread execution requests.
pub trait ThrRequest<T: ThrTag>: IntToken<T> {
    /// Executes the future `f` within the thread.
    fn exec<F, O: ExecOutput>(self, f: F)
    where
        T: ThrAttach,
        F: Future<Output = O> + Send + 'static;

    /// Add an executor for the future `f` within the thread.
    fn add_exec<F, O: ExecOutput>(self, f: F)
    where
        T: ThrAttach,
        F: Future<Output = O> + Send + 'static;

    /// Requests the interrupt.
    #[inline]
    fn trigger(self) {
        WakeInt::new(Self::INT_NUM).wake();
    }
}

/// A trait for implementing arbitrary output types for the futures passed to
/// [`ThrRequest::exec`] and [`ThrRequest::add_exec`].
pub trait ExecOutput: Sized + Send {
    /// A return type of [`ExecOutput::terminate`]. Should be either `()` or
    /// `!`.
    type Terminate;

    /// The output handler.
    fn terminate(self) -> Self::Terminate;
}

impl<T: ThrTag, U: IntToken<T>> ThrRequest<T> for U {
    fn exec<F, O: ExecOutput>(self, fut: F)
    where
        T: ThrAttach,
        F: Future<Output = O> + Send + 'static,
    {
        self.add_exec(fut);
        self.trigger();
    }

    fn add_exec<F, O: ExecOutput>(self, mut fut: F)
    where
        T: ThrAttach,
        F: Future<Output = O> + Send + 'static,
    {
        fn poll<F: Future>(fut: Pin<&mut F>, int_num: usize) -> Poll<F::Output> {
            let waker = WakeInt::new(int_num).to_waker();
            let mut cx = Context::from_waker(&waker);
            fut.poll(&mut cx)
        }
        self.add(move || {
            loop {
                match poll(unsafe { Pin::new_unchecked(&mut fut) }, Self::INT_NUM) {
                    Poll::Pending => {
                        yield;
                    }
                    Poll::Ready(output) => {
                        output.terminate();
                        break;
                    }
                }
            }
        });
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
    panic!("Root future exited with error: {}", err);
}
