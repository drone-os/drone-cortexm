//! Timers and watchdogs.

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
mod sys_tick;

pub use self::sys_tick::SysTick;

use drone::sync::spsc::unit;
use drone::thread::RoutineFuture;
use reg::prelude::*;

/// Generic timer.
pub trait Timer: Sized {
  /// Timer control register value.
  type CtrlVal: RegVal;

  /// Counter type.
  type Counter;

  /// Returns a future that completes once `duration` ticks have elapsed.
  fn sleep(
    &mut self,
    duration: Self::Counter,
    ctrl_val: Self::CtrlVal,
  ) -> RoutineFuture<(), ()>;

  /// Returns a stream that resolves every `duration` ticks.
  fn interval(
    &mut self,
    duration: Self::Counter,
    ctrl_val: Self::CtrlVal,
  ) -> unit::Receiver<()>;

  /// Stops the timer.
  fn stop(&mut self, ctrl_val: Self::CtrlVal);

  /// Stops the timer by setting the control register to the reset value.
  fn reset(&mut self);
}
