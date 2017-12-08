use reg::prelude::*;
use reg::stk;

/// SysTick timer.
pub struct SysTick {
  ctrl: stk::Ctrl<Fbt>,
  load: stk::Load<Sbt>,
  val: stk::Val<Sbt>,
}

/// SysTick timer items.
pub macro SysTick($bindings:ident) {
  $crate::peripherals::timer::SysTick::compose(
    $bindings.stk_ctrl,
    $bindings.stk_load,
    $bindings.stk_val,
  )
}

impl SysTick {
  /// Composes a new `SysTick` from pieces.
  #[inline(always)]
  pub fn compose(
    stk_ctrl: stk::Ctrl<Sbt>,
    stk_load: stk::Load<Sbt>,
    stk_val: stk::Val<Sbt>,
  ) -> Self {
    Self {
      ctrl: stk_ctrl.into(),
      load: stk_load,
      val: stk_val,
    }
  }

  /// Decomposes the `SpiDma` into pieces.
  #[inline(always)]
  pub fn decompose(self) -> (stk::Ctrl<Fbt>, stk::Load<Sbt>, stk::Val<Sbt>) {
    (self.ctrl, self.load, self.val)
  }

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
  pub fn timeout<T: Thread>(
    mut self,
    duration: u32,
    mut ctrl_val: stk::ctrl::Val,
    thread: &T,
  ) -> impl Future<Item = Self, Error = ()> {
    ctrl_val = disable(&mut self.ctrl.hold(ctrl_val)).val();
    self.ctrl.store_val(ctrl_val);
    self.schedule(duration);
    let ctrl = self.ctrl.fork();
    let future = thread.future_fn(move || {
      self.ctrl.store_val(ctrl_val);
      Ok(self)
    });
    ctrl_val = enable(&mut ctrl.hold(ctrl_val)).val();
    ctrl.store_val(ctrl_val);
    future
  }

  /// Returns a reference to the binding.
  #[inline(always)]
  pub fn ctrl(&self) -> &stk::Ctrl<Fbt> {
    &self.ctrl
  }

  /// Returns a reference to the binding.
  #[inline(always)]
  pub fn load(&self) -> &stk::Load<Sbt> {
    &self.load
  }

  /// Returns a reference to the binding.
  #[inline(always)]
  pub fn val(&self) -> &stk::Val<Sbt> {
    &self.val
  }
}

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
#[inline(always)]
fn enable<'a, 'b, T: RegTag>(
  ctrl: &'a mut stk::ctrl::Hold<'b, T>,
) -> &'a mut stk::ctrl::Hold<'b, T> {
  ctrl.set_enable().set_tickint()
}

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
#[inline(always)]
fn disable<'a, 'b, T: RegTag>(
  ctrl: &'a mut stk::ctrl::Hold<'b, T>,
) -> &'a mut stk::ctrl::Hold<'b, T> {
  ctrl.clear_enable().clear_tickint()
}
