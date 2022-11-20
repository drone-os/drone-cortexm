//! Generic timer.

use core::fmt;
use core::future::Future;
use core::num::NonZeroUsize;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::stream::Stream;

/// Error returned from [`Timer::interval`] on overflow.
#[derive(Debug)]
pub struct Overflow;

/// Generic timer driver.
pub trait Timer: Send {
    /// Timer stop handler.
    type Stop: Stop;

    /// Returns a future that resolves when `duration` time is elapsed.
    fn sleep(&mut self, duration: u32) -> Sleep<'_, Self::Stop>;

    /// Returns a stream of pulses that are generated on each `duration`
    /// interval. Fails on overflow.
    fn interval(
        &mut self,
        duration: u32,
    ) -> Interval<'_, Self::Stop, Result<NonZeroUsize, Overflow>>;

    /// Returns a stream of pulses that are generated on each `duration`
    /// interval. Overflows are ignored.
    fn interval_skip(&mut self, duration: u32) -> Interval<'_, Self::Stop, NonZeroUsize>;
}

/// Timer stop handler.
pub trait Stop: Send {
    /// Stops the timer.
    fn stop(&mut self);
}

/// Future created from [`Timer::sleep`].
#[must_use]
pub struct Sleep<'a, T: Stop> {
    stop: &'a mut T,
    future: Pin<Box<dyn Future<Output = ()> + Send + 'a>>,
}

/// Stream created from [`Timer::interval`] or  [`Timer::interval_skip`].
#[must_use]
pub struct Interval<'a, T: Stop, I> {
    stop: &'a mut T,
    stream: Pin<Box<dyn Stream<Item = I> + Send + 'a>>,
}

impl<'a, T: Stop> Sleep<'a, T> {
    /// Creates a new [`Sleep`].
    #[inline]
    pub fn new(stop: &'a mut T, future: Pin<Box<dyn Future<Output = ()> + Send + 'a>>) -> Self {
        Self { stop, future }
    }
}

impl<'a, T: Stop> Future for Sleep<'a, T> {
    type Output = ();

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.future.as_mut().poll(cx)
    }
}

impl<'a, T: Stop> Drop for Sleep<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.stop.stop();
    }
}

impl<'a, T: Stop, I> Interval<'a, T, I> {
    /// Creates a new [`Interval`].
    #[inline]
    pub fn new(stop: &'a mut T, stream: Pin<Box<dyn Stream<Item = I> + Send + 'a>>) -> Self {
        Self { stop, stream }
    }

    /// Stops the timer and the stream.
    #[inline]
    pub fn stop(mut self: Pin<&mut Self>) {
        self.stop.stop();
    }
}

impl<'a, T: Stop, I> Stream for Interval<'a, T, I> {
    type Item = I;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<I>> {
        self.stream.as_mut().poll_next(cx)
    }
}

impl<'a, T: Stop, I> Drop for Interval<'a, T, I> {
    #[inline]
    fn drop(&mut self) {
        self.stop.stop();
    }
}

impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Timer stream overflow.")
    }
}
