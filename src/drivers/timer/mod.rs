//! Timers and watchdogs.

mod sys_tick;

pub use self::sys_tick::{SysTick, SysTickRes};

use reg::prelude::*;

/// Error returned from [`Timer::interval`].
///
/// [`Timer::interval`]: struct.Timer.html#method.interval
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimerOverflow;

/// Generic timer.
pub trait Timer: Sized + Send + Sync + 'static {
  /// Duration type.
  type Duration;

  /// Timer control register value.
  type CtrlVal: RegVal;

  /// Return value for [`sleep`] method.
  ///
  /// [`sleep`]: #method.sleep
  type SleepFuture: Future<Item = (), Error = !> + Send;

  /// Return value for [`interval`] method.
  ///
  /// [`interval`]: #method.interval
  type IntervalStream: Stream<Item = (), Error = TimerOverflow> + Send;

  /// Return value for [`interval_skip`] method.
  ///
  /// [`interval_skip`]: #method.interval_skip
  type IntervalSkipStream: Stream<Item = (), Error = !> + Send;

  /// Returns a future that completes once `duration` ticks have elapsed.
  fn sleep(
    &mut self,
    duration: Self::Duration,
    ctrl_val: Self::CtrlVal,
  ) -> Self::SleepFuture;

  /// Returns a stream that resolves every `duration` ticks.
  fn interval(
    &mut self,
    duration: Self::Duration,
    ctrl_val: Self::CtrlVal,
  ) -> Self::IntervalStream;

  /// Returns a stream that resolves every `duration` ticks. Skips overflow.
  fn interval_skip(
    &mut self,
    duration: Self::Duration,
    ctrl_val: Self::CtrlVal,
  ) -> Self::IntervalSkipStream;

  /// Stops the timer.
  fn stop(&mut self, ctrl_val: Self::CtrlVal);
}
