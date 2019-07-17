use crate::thr::wake::WakeTrunk;
use core::{
    iter::FusedIterator,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use futures::stream::Stream;

/// A stream combinator which converts an asynchronous stream to a **blocking
/// iterator**.
pub struct StreamTrunkWait<'a, T: Stream> {
    stream: T,
    exhausted: bool,
    _marker: PhantomData<&'a &'a mut ()>,
}

/// Stream extensions.
pub trait StreamExt<'a>: Stream {
    /// Creates an iterator which blocks the current thread until each item of
    /// this stream is resolved.
    fn trunk_wait(self) -> StreamTrunkWait<'a, Self>
    where
        Self: Sized;
}

impl<'a, T: Stream> StreamExt<'a> for T {
    #[inline]
    fn trunk_wait(self) -> StreamTrunkWait<'a, Self>
    where
        Self: Sized,
    {
        StreamTrunkWait {
            stream: self,
            exhausted: false,
            _marker: PhantomData,
        }
    }
}

impl<'a, T: Stream> Iterator for StreamTrunkWait<'a, T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }
        let waker = WakeTrunk::new().to_waker();
        let mut cx = Context::from_waker(&waker);
        loop {
            match unsafe { Pin::new_unchecked(&mut self.stream) }.poll_next(&mut cx) {
                Poll::Pending => WakeTrunk::wait(),
                Poll::Ready(Some(item)) => break Some(item),
                Poll::Ready(None) => {
                    self.exhausted = true;
                    break None;
                }
            }
        }
    }
}

impl<'a, T: Stream> FusedIterator for StreamTrunkWait<'a, T> {}
