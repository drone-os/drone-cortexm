use super::{Timer, TimerOverflow};
use drone_core::drivers::{Driver, Resource};
use drone_core::fiber::{FiberFuture, FiberStreamUnit};
use reg::prelude::*;
use reg::stk;
use thread::irq::IrqSysTick;
use thread::prelude::*;

/// Creates a new `SysTick`.
#[macro_export]
macro_rules! drv_sys_tick {
  ($regs:ident, $thrd:ident) => {
    $crate::drivers::timer::SysTick::from_res(
      $crate::drivers::timer::SysTickRes {
        sys_tick: $thrd.sys_tick.into(),
        stk_ctrl: $regs.stk_ctrl,
        stk_load: $regs.stk_load,
        stk_val: $regs.stk_val,
      }
    )
  }
}

/// SysTick driver.
pub struct SysTick<I: IrqSysTick<Ltt>>(SysTickRes<I, Frt>);

/// SysTick resource.
#[allow(missing_docs)]
pub struct SysTickRes<I: IrqSysTick<Ltt>, Rt: RegTag> {
  pub sys_tick: I,
  pub stk_ctrl: stk::Ctrl<Rt>,
  pub stk_load: stk::Load<Srt>,
  pub stk_val: stk::Val<Srt>,
}

impl<I: IrqSysTick<Ltt>> Resource for SysTickRes<I, Frt> {
  type Input = SysTickRes<I, Srt>;
}

impl<I: IrqSysTick<Ltt>> From<SysTickRes<I, Srt>> for SysTickRes<I, Frt> {
  #[inline(always)]
  fn from(res: SysTickRes<I, Srt>) -> Self {
    Self {
      sys_tick: res.sys_tick,
      stk_ctrl: res.stk_ctrl.into(),
      stk_load: res.stk_load,
      stk_val: res.stk_val,
    }
  }
}

impl<I: IrqSysTick<Ltt>> Driver for SysTick<I> {
  type Resource = SysTickRes<I, Frt>;

  #[inline(always)]
  fn from_res(res: SysTickRes<I, Srt>) -> Self {
    SysTick(res.into())
  }

  #[inline(always)]
  fn into_res(self) -> SysTickRes<I, Frt> {
    self.0
  }
}

impl<I: IrqSysTick<Ltt>> Timer for SysTick<I> {
  type Duration = u32;
  type CtrlVal = stk::ctrl::Val;
  type SleepFuture = FiberFuture<(), !>;
  type IntervalStream = FiberStreamUnit<TimerOverflow>;
  type IntervalSkipStream = FiberStreamUnit<!>;

  #[inline(always)]
  fn sleep(
    &mut self,
    duration: Self::Duration,
    mut ctrl_val: Self::CtrlVal,
  ) -> Self::SleepFuture {
    ctrl_val = disable(&mut self.0.stk_ctrl.hold(ctrl_val)).val();
    self.0.stk_ctrl.store_val(ctrl_val);
    schedule(&self.0.stk_load, &self.0.stk_val, duration);
    let ctrl = self.0.stk_ctrl.fork();
    let future = self.0.sys_tick.future_fn(move || {
      ctrl.store_val(ctrl_val);
      Ok(())
    });
    ctrl_val = enable(&mut self.0.stk_ctrl.hold(ctrl_val)).val();
    self.0.stk_ctrl.store_val(ctrl_val);
    future
  }

  #[inline(always)]
  fn interval(
    &mut self,
    duration: Self::Duration,
    ctrl_val: Self::CtrlVal,
  ) -> Self::IntervalStream {
    self.interval_stream(duration, ctrl_val, |irq| {
      irq.stream(
        || Err(TimerOverflow),
        || loop {
          yield Some(());
        },
      )
    })
  }

  #[inline(always)]
  fn interval_skip(
    &mut self,
    duration: Self::Duration,
    ctrl_val: Self::CtrlVal,
  ) -> Self::IntervalSkipStream {
    self.interval_stream(duration, ctrl_val, |irq| {
      irq.stream_skip(|| loop {
        yield Some(());
      })
    })
  }

  #[inline(always)]
  fn stop(&mut self, mut ctrl_val: Self::CtrlVal) {
    ctrl_val = disable(&mut self.0.stk_ctrl.hold(ctrl_val)).val();
    self.0.stk_ctrl.store_val(ctrl_val);
  }
}

impl<I: IrqSysTick<Ltt>> SysTick<I> {
  fn interval_stream<F, S>(
    &mut self,
    duration: u32,
    mut ctrl_val: stk::ctrl::Val,
    f: F,
  ) -> S
  where
    F: FnOnce(I) -> S,
    S: Stream,
  {
    ctrl_val = disable(&mut self.0.stk_ctrl.hold(ctrl_val)).val();
    self.0.stk_ctrl.store_val(ctrl_val);
    schedule(&self.0.stk_load, &self.0.stk_val, duration);
    let stream = f(self.0.sys_tick);
    ctrl_val = enable(&mut self.0.stk_ctrl.hold(ctrl_val)).val();
    self.0.stk_ctrl.store_val(ctrl_val);
    stream
  }
}

#[allow(missing_docs)]
impl<I: IrqSysTick<Ltt>> SysTick<I> {
  #[inline(always)]
  pub fn irq(&self) -> I {
    self.0.sys_tick
  }

  #[inline(always)]
  pub fn ctrl(&self) -> &stk::Ctrl<Frt> {
    &self.0.stk_ctrl
  }

  #[inline(always)]
  pub fn load(&self) -> &stk::Load<Srt> {
    &self.0.stk_load
  }

  #[inline(always)]
  pub fn val(&self) -> &stk::Val<Srt> {
    &self.0.stk_val
  }
}

#[inline(always)]
fn schedule(stk_load: &stk::Load<Srt>, stk_val: &stk::Val<Srt>, duration: u32) {
  stk_load.store(|r| r.write_reload(duration));
  stk_val.store(|r| r.write_current(0));
}

#[inline(always)]
fn enable<'a, 'b>(
  ctrl: &'a mut stk::ctrl::Hold<'b, Frt>,
) -> &'a mut stk::ctrl::Hold<'b, Frt> {
  ctrl.set_enable().set_tickint()
}

#[inline(always)]
fn disable<'a, 'b>(
  ctrl: &'a mut stk::ctrl::Hold<'b, Frt>,
) -> &'a mut stk::ctrl::Hold<'b, Frt> {
  ctrl.clear_enable().clear_tickint()
}
