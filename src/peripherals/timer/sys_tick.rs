use super::Timer;
use core::marker::PhantomData;
use drone::sync::spsc::unit;
use drone::thread::RoutineFuture;
use reg::prelude::*;
use reg::stk;
use thread::interrupts::IrqSysTick;

/// SysTick timer.
pub struct SysTick<T: Thread, I: IrqSysTick<T>> {
  _thread: PhantomData<&'static T>,
  irq: I,
  ctrl: stk::Ctrl<Fbt>,
  load: stk::Load<Sbt>,
  val: stk::Val<Sbt>,
}

/// SysTick timer items.
pub macro SysTick($threads:ident, $regs:ident) {
  $crate::peripherals::timer::SysTick::compose(
    $threads.sys_tick,
    $regs.stk_ctrl,
    $regs.stk_load,
    $regs.stk_val,
  )
}

#[allow(missing_docs)]
impl<T: Thread, I: IrqSysTick<T>> SysTick<T, I> {
  /// Composes a new `SysTick` from pieces.
  #[inline(always)]
  pub fn compose(
    irq: I,
    stk_ctrl: stk::Ctrl<Sbt>,
    stk_load: stk::Load<Sbt>,
    stk_val: stk::Val<Sbt>,
  ) -> Self {
    Self {
      _thread: PhantomData,
      irq,
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

  #[inline(always)]
  pub fn irq(&self) -> I {
    self.irq
  }

  #[inline(always)]
  pub fn ctrl(&self) -> &stk::Ctrl<Fbt> {
    &self.ctrl
  }

  #[inline(always)]
  pub fn load(&self) -> &stk::Load<Sbt> {
    &self.load
  }

  #[inline(always)]
  pub fn val(&self) -> &stk::Val<Sbt> {
    &self.val
  }
}

impl<T: Thread, I: IrqSysTick<T>> Timer for SysTick<T, I> {
  type Counter = u32;
  type CtrlVal = stk::ctrl::Val;

  #[inline]
  fn sleep(
    &mut self,
    duration: Self::Counter,
    mut ctrl_val: Self::CtrlVal,
  ) -> RoutineFuture<(), ()> {
    ctrl_val = disable(&mut self.ctrl.hold(ctrl_val)).val();
    self.ctrl.store_val(ctrl_val);
    schedule(&self.load, &self.val, duration);
    let ctrl = self.ctrl.fork();
    let future = self.irq.future_fn(move || {
      ctrl.store_val(ctrl_val);
      Ok(())
    });
    ctrl_val = enable(&mut self.ctrl.hold(ctrl_val)).val();
    self.ctrl.store_val(ctrl_val);
    future
  }

  #[inline]
  fn interval(
    &mut self,
    duration: Self::Counter,
    mut ctrl_val: Self::CtrlVal,
  ) -> unit::Receiver<()> {
    ctrl_val = disable(&mut self.ctrl.hold(ctrl_val)).val();
    self.ctrl.store_val(ctrl_val);
    schedule(&self.load, &self.val, duration);
    let stream = self.irq.stream_skip(|| loop {
      yield Some(());
    });
    ctrl_val = enable(&mut self.ctrl.hold(ctrl_val)).val();
    self.ctrl.store_val(ctrl_val);
    stream
  }

  #[inline(always)]
  fn stop(&mut self, mut ctrl_val: Self::CtrlVal) {
    ctrl_val = disable(&mut self.ctrl.hold(ctrl_val)).val();
    self.ctrl.store_val(ctrl_val);
  }

  #[inline(always)]
  fn reset(&mut self) {
    let ctrl_val = self.ctrl.default().val();
    self.stop(ctrl_val);
  }
}

#[inline(always)]
fn schedule(stk_load: &stk::Load<Sbt>, stk_val: &stk::Val<Sbt>, duration: u32) {
  stk_load.reset(|r| r.write_reload(duration));
  stk_val.reset(|r| r.write_current(0));
}

#[inline(always)]
fn enable<'a, 'b>(
  ctrl: &'a mut stk::ctrl::Hold<'b, Fbt>,
) -> &'a mut stk::ctrl::Hold<'b, Fbt> {
  ctrl.set_enable().set_tickint()
}

#[inline(always)]
fn disable<'a, 'b>(
  ctrl: &'a mut stk::ctrl::Hold<'b, Fbt>,
) -> &'a mut stk::ctrl::Hold<'b, Fbt> {
  ctrl.clear_enable().clear_tickint()
}
