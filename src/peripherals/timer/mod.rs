//! Timers and watchdogs.

mod sys_tick;

pub use self::sys_tick::{SysTick, SysTickTokens};

use drone_core::sync::spsc::unit;
use drone_core::thread::RoutineFuture;
use reg::prelude::*;

/// Generic timer.
pub trait Timer
where
  Self: Sized,
  Self::Tokens: From<Self>,
{
  /// Generic timer input tokens.
  type InputTokens;

  /// Generic timer tokens.
  type Tokens;

  /// Duration type.
  type Duration;

  /// Timer control register.
  type Ctrl: for<'a> WRegShared<'a, Ftt>;

  /// Creates a new `Timer` driver from provided `tokens`.
  fn new(tokens: Self::InputTokens) -> Self;

  /// Returns a future that completes once `duration` ticks have elapsed.
  fn sleep(
    &mut self,
    duration: Self::Duration,
    ctrl_val: <Self::Ctrl as Reg<Ftt>>::Val,
  ) -> RoutineFuture<(), ()>;

  /// Returns a stream that resolves every `duration` ticks.
  fn interval(
    &mut self,
    duration: Self::Duration,
    ctrl_val: <Self::Ctrl as Reg<Ftt>>::Val,
  ) -> unit::Receiver<()>;

  /// Stops the timer.
  fn stop(&mut self, ctrl_val: <Self::Ctrl as Reg<Ftt>>::Val);
}
