//! Generic timer.

use core::fmt;
use core::future::Future;
use core::num::NonZeroUsize;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::stream::Stream;

/// Error returned from [`Timer::interval`] on overflow.
#[derive(Debug)]
pub struct TimerOverflow;

/// Generic timer driver.
pub trait Timer: Send {
    /// Timer stop handler.
    type Stop: TimerStop;

    /// Returns a future that resolves when `duration` time is elapsed.
    fn sleep(&mut self, duration: u32) -> TimerSleep<'_, Self::Stop>;

    /// Returns a stream of pulses that are generated on each `duration`
    /// interval. Fails on overflow.
    fn interval(
        &mut self,
        duration: u32,
    ) -> TimerInterval<'_, Self::Stop, Result<NonZeroUsize, TimerOverflow>>;

    /// Returns a stream of pulses that are generated on each `duration`
    /// interval. Overflows are ignored.
    fn interval_skip(&mut self, duration: u32) -> TimerInterval<'_, Self::Stop, NonZeroUsize>;
}

/// Timer stop handler.
pub trait TimerStop: Send {
    /// Stops the timer.
    fn stop(&mut self);
}

/// Future created from [`Timer::sleep`].
pub struct TimerSleep<'a, T: TimerStop> {
    stop: &'a mut T,
    future: Pin<Box<dyn Future<Output = ()> + Send + 'a>>,
}

/// Stream created from [`Timer::interval`] or  [`Timer::interval_skip`].
pub struct TimerInterval<'a, T: TimerStop, I> {
    stop: &'a mut T,
    stream: Pin<Box<dyn Stream<Item = I> + Send + 'a>>,
}

impl<'a, T: TimerStop> TimerSleep<'a, T> {
    /// Creates a new [`TimerSleep`].
    pub fn new(stop: &'a mut T, future: Pin<Box<dyn Future<Output = ()> + Send + 'a>>) -> Self {
        Self { stop, future }
    }
}

impl<'a, T: TimerStop> Future for TimerSleep<'a, T> {
    type Output = ();

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.future.as_mut().poll(cx)
    }
}

impl<'a, T: TimerStop> Drop for TimerSleep<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.stop.stop();
    }
}

impl<'a, T: TimerStop, I> TimerInterval<'a, T, I> {
    /// Creates a new [`TimerInterval`].
    pub fn new(stop: &'a mut T, stream: Pin<Box<dyn Stream<Item = I> + Send + 'a>>) -> Self {
        Self { stop, stream }
    }

    /// Stops the timer and the stream.
    #[inline]
    pub fn stop(mut self: Pin<&mut Self>) {
        self.stop.stop();
    }
}

impl<'a, T: TimerStop, I> Stream for TimerInterval<'a, T, I> {
    type Item = I;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<I>> {
        self.stream.as_mut().poll_next(cx)
    }
}

impl<'a, T: TimerStop, I> Drop for TimerInterval<'a, T, I> {
    #[inline]
    fn drop(&mut self) {
        self.stop.stop();
    }
}

impl fmt::Display for TimerOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Timer stream overflow.")
    }
}
