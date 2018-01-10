//! Timers and watchdogs.

mod sys_tick;

pub use self::sys_tick::{SysTick, SysTickTokens};

use drone_core::thread::{RoutineFuture, RoutineStreamUnit};
use reg::prelude::*;

/// Error returned from [`Timer::interval`].
///
/// [`Timer::interval`]: trait.Timer.html#method.interval
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimerOverflow;

/// Generic timer.
pub trait Timer
where
  Self: Sized + Send + Sync + 'static,
  Self::Tokens: From<Self>,
{
  /// Generic timer input tokens.
  type InputTokens;

  /// Generic timer tokens.
  type Tokens;

  /// Duration type.
  type Duration;

  /// Timer control register.
  type Ctrl: for<'a> WRegShared<'a, Frt>;

  /// Creates a new `Timer` driver from provided `tokens`.
  fn new(tokens: Self::InputTokens) -> Self;

  /// Returns a future that completes once `duration` ticks have elapsed.
  fn sleep(
    &mut self,
    duration: Self::Duration,
    ctrl_val: <Self::Ctrl as Reg<Frt>>::Val,
  ) -> RoutineFuture<(), !>;

  /// Returns a stream that resolves every `duration` ticks.
  fn interval(
    &mut self,
    duration: Self::Duration,
    ctrl_val: <Self::Ctrl as Reg<Frt>>::Val,
  ) -> RoutineStreamUnit<TimerOverflow>;

  /// Returns a stream that resolves every `duration` ticks. Skips overflow.
  fn interval_skip(
    &mut self,
    duration: Self::Duration,
    ctrl_val: <Self::Ctrl as Reg<Frt>>::Val,
  ) -> RoutineStreamUnit<!>;

  /// Stops the timer.
  fn stop(&mut self, ctrl_val: <Self::Ctrl as Reg<Frt>>::Val);
}
