//! Timers and watchdogs.

mod sys_tick;

pub use self::sys_tick::{SysTick, SysTickRes};

use drivers::prelude::*;
use drone_core::bitfield::Bitfield;

/// Error returned from [`Timer::interval`](Timer::interval) on overflow.
#[derive(Debug, Fail)]
#[fail(display = "Timer stream overflow.")]
pub struct TimerOverflow;

/// Timer driver.
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

impl<T: TimerRes> Driver for Timer<T> {
  type Resource = T;

  #[inline(always)]
  fn from_res(res: T::Input) -> Self {
    Timer(res.into())
  }

  #[inline(always)]
  fn into_res(self) -> T {
    self.0
  }
}

impl<T: TimerRes> Timer<T> {
  /// Returns a future that completes once `dur` ticks have elapsed.
  #[inline(always)]
  pub fn sleep(
    &mut self,
    dur: T::Duration,
    ctrl_val: T::CtrlVal,
  ) -> T::SleepFuture {
    self.0.sleep(dur, ctrl_val)
  }

  /// Returns a stream that resolves every `dur` ticks.
  #[inline(always)]
  pub fn interval(
    &mut self,
    dur: T::Duration,
    ctrl_val: T::CtrlVal,
  ) -> T::IntervalStream {
    self.0.interval(dur, ctrl_val)
  }

  /// Returns a stream that resolves every `dur` ticks. Skips overflow.
  #[inline(always)]
  pub fn interval_skip(
    &mut self,
    dur: T::Duration,
    ctrl_val: T::CtrlVal,
  ) -> T::IntervalSkipStream {
    self.0.interval_skip(dur, ctrl_val)
  }

  /// Stops the timer.
  #[inline(always)]
  pub fn stop(&mut self, ctrl_val: T::CtrlVal) {
    self.0.stop(ctrl_val)
  }
}
