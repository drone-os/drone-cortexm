//! Timers and watchdogs.

mod sys_tick;

pub use self::sys_tick::{SysTick, SysTickDiverged};

use core::future::Future;
use drone_core::bitfield::Bitfield;
use failure::Fail;
use futures::stream::Stream;

/// Error returned from [`Timer::interval`](Timer::interval) on overflow.
#[derive(Debug, Fail)]
#[fail(display = "Timer stream overflow.")]
pub struct TimerOverflow;

/// Timer driver.
#[allow(missing_docs)]
pub trait Timer: Sized + Send + 'static {
  type Duration;
  type CtrlVal: Bitfield;
  type SleepFuture: Future<Output = ()> + Send;
  type IntervalStream: Stream<Item = Result<(), TimerOverflow>> + Send;
  type IntervalSkipStream: Stream<Item = ()> + Send;

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
