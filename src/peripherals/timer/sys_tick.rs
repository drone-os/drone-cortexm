//! SysTick timer.

pub use self::build as Driver;

use reg::prelude::*;
use reg::stk;

/// SysTick timer.
pub struct Driver {
  ctrl: stk::Ctrl<Drt>,
  load: stk::Load<Srt>,
  val: stk::Val<Srt>,
}

/// SysTick timer.
#[allow(missing_docs)]
pub struct Builder {
  pub stk_ctrl: stk::Ctrl<Srt>,
  pub stk_load: stk::Load<Srt>,
  pub stk_val: stk::Val<Srt>,
}

impl Builder {
  /// Creates a new `Driver`.
  #[inline(always)]
  pub fn build(self) -> Driver {
    Driver {
      ctrl: self.stk_ctrl.upgrade(),
      load: self.stk_load,
      val: self.stk_val,
    }
  }
}

/// SysTick timer.
pub macro build($bindings:ident) {
  $crate::peripherals::timer::sys_tick::Builder {
    stk_ctrl: $bindings.stk_ctrl,
    stk_load: $bindings.stk_load,
    stk_val: $bindings.stk_val,
  }.build()
}

impl Driver {
  /// Schedules SysTick event. The event will be triggering in periods of
  /// `duration` ticks.
  #[inline(always)]
  pub fn schedule(&self, duration: u32) {
    self.load.reset(|r| r.write_reload(duration));
    self.val.reset(|r| r.write_current(0));
  }

  /// Starts the timer.
  #[inline(always)]
  pub fn start(&mut self, ctrl: stk::ctrl::Val) {
    let mut ctrl = self.ctrl.hold(ctrl);
    self.ctrl.store_val(enable(&mut ctrl).val());
  }

  /// Stops the timer.
  #[inline(always)]
  pub fn stop(&mut self, ctrl: stk::ctrl::Val) {
    let mut ctrl = self.ctrl.hold(ctrl);
    self.ctrl.store_val(disable(&mut ctrl).val());
  }

  /// Returns a future, which resolves after `duration` ticks.
  #[inline]
  pub fn timeout<T>(
    mut self,
    duration: u32,
    ctrl: stk::ctrl::Val,
    thread: &T,
  ) -> impl Future<Item = Self, Error = ()>
  where
    T: Thread,
  {
    let (disable, enable) = {
      let mut ctrl = self.ctrl.hold(ctrl);
      let disable = disable(&mut ctrl).val();
      let enable = enable(&mut ctrl).val();
      (disable, enable)
    };
    self.ctrl.store_val(disable);
    self.schedule(duration);
    let ctrl = self.ctrl.clone();
    let future = thread.future_fn(move || {
      self.ctrl.store_val(disable);
      Ok(self)
    });
    ctrl.store_val(enable);
    future
  }

  /// Returns a reference to the binding.
  #[inline(always)]
  pub fn ctrl(&self) -> &stk::Ctrl<Drt> {
    &self.ctrl
  }

  /// Returns a reference to the binding.
  #[inline(always)]
  pub fn load(&self) -> &stk::Load<Srt> {
    &self.load
  }

  /// Returns a reference to the binding.
  #[inline(always)]
  pub fn val(&self) -> &stk::Val<Srt> {
    &self.val
  }
}

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
#[inline(always)]
fn enable<'a, 'b, T>(
  ctrl: &'a mut stk::ctrl::Hold<'b, T>,
) -> &'a mut stk::ctrl::Hold<'b, T>
where
  T: RegTag,
{
  ctrl.set_enable().set_tickint()
}

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
#[inline(always)]
fn disable<'a, 'b, T>(
  ctrl: &'a mut stk::ctrl::Hold<'b, T>,
) -> &'a mut stk::ctrl::Hold<'b, T>
where
  T: RegTag,
{
  ctrl.clear_enable().clear_tickint()
}
