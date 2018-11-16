//! Timers and watchdogs.

mod sys_tick;

pub use self::sys_tick::{SysTick, SysTickRes};

use drone_core::bitfield::Bitfield;
use drone_core::drv::Resource;
use futures::prelude::*;

/// Error returned from [`Timer::interval`](Timer::interval) on overflow.
#[derive(Debug, Fail)]
#[fail(display = "Timer stream overflow.")]
pub struct TimerOverflow;

/// Timer driver.
#[derive(Driver)]
pub struct Timer<T: TimerRes>(T);

/// Timer resource.
#[allow(missing_docs)]
pub trait TimerRes: Resource {
  type Duration;
  type CtrlVal: Bitfield;
  type SleepFuture: Future<Item = (), Error = !> + Send;
  type IntervalStream: Stream<Item = (), Error = TimerOverflow> + Send;
  type IntervalSkipStream: Stream<Item = (), Error = !> + Send;

  fn sleep(
    &mut self,
    dur: Self::Duration,
    ctrl_val: Self::CtrlVal,
  ) -> Self::SleepFuture;

  fn interval(
    &mut self,
    dur: Self::Duration,
    ctrl_val: Self::CtrlVal,
  ) -> Self::IntervalStream;

  fn interval_skip(
    &mut self,
    dur: Self::Duration,
    ctrl_val: Self::CtrlVal,
  ) -> Self::IntervalSkipStream;

  fn stop(&mut self, ctrl_val: Self::CtrlVal);
}

impl<T: TimerRes> Timer<T> {
  /// Returns a future that completes once `dur` ticks have elapsed.
  #[inline]
  pub fn sleep(
    &mut self,
    dur: T::Duration,
    ctrl_val: T::CtrlVal,
  ) -> T::SleepFuture {
    self.0.sleep(dur, ctrl_val)
  }

  /// Returns a stream that resolves every `dur` ticks.
  #[inline]
  pub fn interval(
    &mut self,
    dur: T::Duration,
    ctrl_val: T::CtrlVal,
  ) -> T::IntervalStream {
    self.0.interval(dur, ctrl_val)
  }

  /// Returns a stream that resolves every `dur` ticks. Skips overflow.
  #[inline]
  pub fn interval_skip(
    &mut self,
    dur: T::Duration,
    ctrl_val: T::CtrlVal,
  ) -> T::IntervalSkipStream {
    self.0.interval_skip(dur, ctrl_val)
  }

  /// Stops the timer.
  #[inline]
  pub fn stop(&mut self, ctrl_val: T::CtrlVal) {
    self.0.stop(ctrl_val)
  }
}
