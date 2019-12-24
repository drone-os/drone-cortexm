use crate::thr::wake::WakeRoot;
use core::{
    future::Future,
    iter::FusedIterator,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use futures::stream::Stream;

/// An extension trait for [`Future`] that provides
/// [`root_wait`](FutureRootExt::root_wait) method.
pub trait FutureRootExt: Future {
    /// Runs a future to completion on the lowest priority thread.
    ///
    /// This method will block the caller until the given future has completed.
    ///
    /// **WARNING** This method will block currently preempted threads. It is
    /// recommended to use this method only on the lowest priority thread.
    fn root_wait(self) -> Self::Output;
}

/// An extension trait for [`Stream`] that provides
/// [`root_wait`](StreamRootExt::root_wait) method.
pub trait StreamRootExt<'a>: Stream {
    /// Turn a stream into a blocking iterator.
    ///
    /// When `next` is called on the resulting [`StreamRootWait`], the caller
    /// will be blocked until the next element of the `Stream` becomes
    /// available.
    ///
    /// **WARNING** The resulting [`StreamRootWait`] will be blocking preempted
    /// threads. It is recommended to use this method only on the lowest
    /// priority thread.
    fn root_wait(self) -> StreamRootWait<'a, Self>
    where
        Self: Sized;
}

/// An iterator that blocks on values from a stream until they become available.
///
/// **WARNING** The `next` method will be blocking preempted threads. It is
/// recommended to use this iterator only on the lowest priority thread.
pub struct StreamRootWait<'a, T: Stream> {
    stream: T,
    exhausted: bool,
    _marker: PhantomData<&'a &'a mut ()>,
}

impl<T: Future> FutureRootExt for T {
    fn root_wait(mut self) -> Self::Output {
        let waker = WakeRoot::new().to_waker();
        let mut cx = Context::from_waker(&waker);
        loop {
            match unsafe { Pin::new_unchecked(&mut self) }.poll(&mut cx) {
                Poll::Pending => WakeRoot::wait(),
                Poll::Ready(value) => break value,
            }
        }
    }
}

impl<'a, T: Stream> StreamRootExt<'a> for T {
    #[inline]
    fn root_wait(self) -> StreamRootWait<'a, Self>
    where
        Self: Sized,
    {
        StreamRootWait { stream: self, exhausted: false, _marker: PhantomData }
    }
}

impl<'a, T: Stream> Iterator for StreamRootWait<'a, T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }
        let waker = WakeRoot::new().to_waker();
        let mut cx = Context::from_waker(&waker);
        loop {
            match unsafe { Pin::new_unchecked(&mut self.stream) }.poll_next(&mut cx) {
                Poll::Pending => WakeRoot::wait(),
                Poll::Ready(Some(item)) => break Some(item),
                Poll::Ready(None) => {
                    self.exhausted = true;
                    break None;
                }
            }
        }
    }
}

impl<'a, T: Stream> FusedIterator for StreamRootWait<'a, T> {}
