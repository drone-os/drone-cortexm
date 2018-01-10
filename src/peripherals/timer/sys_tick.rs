use super::{Timer, TimerOverflow};
use drone_core::thread::{RoutineFuture, RoutineStreamUnit};
use reg::prelude::*;
use reg::stk;
use thread::irq::IrqSysTick;
use thread::prelude::*;

/// SysTick timer.
pub struct SysTick<T: IrqSysTick<Ltt>> {
  tokens: SysTickTokens<T, Frt>,
}

/// SysTick timer tokens.
#[allow(missing_docs)]
pub struct SysTickTokens<T: IrqSysTick<Ltt>, R: RegTag> {
  pub sys_tick: T,
  pub stk_ctrl: stk::Ctrl<R>,
  pub stk_load: stk::Load<Srt>,
  pub stk_val: stk::Val<Srt>,
}

/// Creates a new `SysTick` driver from tokens.
#[macro_export]
macro_rules! peripheral_sys_tick {
  ($regs:ident, $thrd:ident) => {
    $crate::peripherals::timer::SysTick::new(
      $crate::peripherals::timer::SysTickTokens {
        sys_tick: $thrd.sys_tick.into(),
        stk_ctrl: $regs.stk_ctrl,
        stk_load: $regs.stk_load,
        stk_val: $regs.stk_val,
      }
    )
  }
}

#[allow(missing_docs)]
impl<T: IrqSysTick<Ltt>> SysTick<T> {
  #[inline(always)]
  pub fn irq(&self) -> T {
    self.tokens.sys_tick
  }

  #[inline(always)]
  pub fn ctrl(&self) -> &stk::Ctrl<Frt> {
    &self.tokens.stk_ctrl
  }

  #[inline(always)]
  pub fn load(&self) -> &stk::Load<Srt> {
    &self.tokens.stk_load
  }

  #[inline(always)]
  pub fn val(&self) -> &stk::Val<Srt> {
    &self.tokens.stk_val
  }

  fn interval_stream<F, S>(
    &mut self,
    duration: u32,
    mut ctrl_val: stk::ctrl::Val,
    f: F,
  ) -> S
  where
    F: FnOnce(T) -> S,
    S: Stream,
  {
    ctrl_val = disable(&mut self.tokens.stk_ctrl.hold(ctrl_val)).val();
    self.tokens.stk_ctrl.store_val(ctrl_val);
    schedule(&self.tokens.stk_load, &self.tokens.stk_val, duration);
    let stream = f(self.tokens.sys_tick);
    ctrl_val = enable(&mut self.tokens.stk_ctrl.hold(ctrl_val)).val();
    self.tokens.stk_ctrl.store_val(ctrl_val);
    stream
  }
}

impl<T: IrqSysTick<Ltt>> From<SysTick<T>> for SysTickTokens<T, Frt> {
  #[inline(always)]
  fn from(sys_tick: SysTick<T>) -> Self {
    sys_tick.tokens
  }
}

impl<T: IrqSysTick<Ltt>> Timer for SysTick<T> {
  type InputTokens = SysTickTokens<T, Srt>;
  type Tokens = SysTickTokens<T, Frt>;
  type Duration = u32;
  type Ctrl = stk::Ctrl<Frt>;

  #[inline(always)]
  fn new(tokens: SysTickTokens<T, Srt>) -> Self {
    Self {
      tokens: SysTickTokens {
        sys_tick: tokens.sys_tick,
        stk_ctrl: tokens.stk_ctrl.into(),
        stk_load: tokens.stk_load,
        stk_val: tokens.stk_val,
      },
    }
  }

  fn sleep(
    &mut self,
    duration: Self::Duration,
    mut ctrl_val: <Self::Ctrl as Reg<Frt>>::Val,
  ) -> RoutineFuture<(), !> {
    ctrl_val = disable(&mut self.tokens.stk_ctrl.hold(ctrl_val)).val();
    self.tokens.stk_ctrl.store_val(ctrl_val);
    schedule(&self.tokens.stk_load, &self.tokens.stk_val, duration);
    let ctrl = self.tokens.stk_ctrl.fork();
    let future = self.tokens.sys_tick.future_fn(move || {
      ctrl.store_val(ctrl_val);
      Ok(())
    });
    ctrl_val = enable(&mut self.tokens.stk_ctrl.hold(ctrl_val)).val();
    self.tokens.stk_ctrl.store_val(ctrl_val);
    future
  }

  fn interval(
    &mut self,
    duration: Self::Duration,
    ctrl_val: <Self::Ctrl as Reg<Frt>>::Val,
  ) -> RoutineStreamUnit<TimerOverflow> {
    self.interval_stream(duration, ctrl_val, |irq| {
      irq.stream(
        || Err(TimerOverflow),
        || loop {
          yield Some(());
        },
      )
    })
  }

  fn interval_skip(
    &mut self,
    duration: Self::Duration,
    ctrl_val: <Self::Ctrl as Reg<Frt>>::Val,
  ) -> RoutineStreamUnit<!> {
    self.interval_stream(duration, ctrl_val, |irq| {
      irq.stream_skip(|| loop {
        yield Some(());
      })
    })
  }

  #[inline(always)]
  fn stop(&mut self, mut ctrl_val: <Self::Ctrl as Reg<Frt>>::Val) {
    ctrl_val = disable(&mut self.tokens.stk_ctrl.hold(ctrl_val)).val();
    self.tokens.stk_ctrl.store_val(ctrl_val);
  }
}

#[inline(always)]
fn schedule(stk_load: &stk::Load<Srt>, stk_val: &stk::Val<Srt>, duration: u32) {
  stk_load.reset(|r| r.write_reload(duration));
  stk_val.reset(|r| r.write_current(0));
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
