//! Timers and watchdogs.

mod sys_tick;

pub use self::sys_tick::{SysTick, SysTickTokens};

use drone_core::sync::spsc::unit;
use drone_core::thread::RoutineFuture;
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
}
