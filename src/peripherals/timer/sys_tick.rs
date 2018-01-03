use super::Timer;
use drone_core::sync::spsc::unit;
use drone_core::thread::RoutineFuture;
use reg::prelude::*;
use reg::stk;
use thread::interrupts::IrqSysTick;
use thread::prelude::*;

/// SysTick timer.
pub struct SysTick<T: Thread, I: IrqSysTick> {
  tokens: SysTickTokens<T, I, Ftt>,
}

/// SysTick timer tokens.
#[allow(missing_docs)]
pub struct SysTickTokens<T: Thread, I: IrqSysTick, R: RegTag> {
  pub sys_tick: ThreadToken<T, I>,
  pub stk_ctrl: stk::Ctrl<R>,
  pub stk_load: stk::Load<Stt>,
  pub stk_val: stk::Val<Stt>,
}

/// Creates a new `SysTick` driver from tokens.
#[macro_export]
macro_rules! peripheral_sys_tick {
  ($thrd:ident, $regs:ident) => {
    $crate::peripherals::timer::SysTick::new(
      $crate::peripherals::timer::SysTickTokens {
        sys_tick: $thrd.sys_tick,
        stk_ctrl: $regs.stk_ctrl,
        stk_load: $regs.stk_load,
        stk_val: $regs.stk_val,
      }
    )
  }
}

#[allow(missing_docs)]
impl<T: Thread, I: IrqSysTick> SysTick<T, I> {
  #[inline(always)]
  pub fn irq(&self) -> ThreadToken<T, I> {
    self.tokens.sys_tick
  }

  #[inline(always)]
  pub fn ctrl(&self) -> &stk::Ctrl<Ftt> {
    &self.tokens.stk_ctrl
  }

  #[inline(always)]
  pub fn load(&self) -> &stk::Load<Stt> {
    &self.tokens.stk_load
  }

  #[inline(always)]
  pub fn val(&self) -> &stk::Val<Stt> {
    &self.tokens.stk_val
  }
}

impl<T: Thread, I: IrqSysTick> From<SysTick<T, I>>
  for SysTickTokens<T, I, Ftt> {
  #[inline(always)]
  fn from(sys_tick: SysTick<T, I>) -> Self {
    sys_tick.tokens
  }
}

impl<T: Thread, I: IrqSysTick> Timer for SysTick<T, I> {
  type InputTokens = SysTickTokens<T, I, Stt>;
  type Tokens = SysTickTokens<T, I, Ftt>;
  type Duration = u32;
  type Ctrl = stk::Ctrl<Ftt>;

  #[inline(always)]
  fn new(tokens: SysTickTokens<T, I, Stt>) -> Self {
    Self {
      tokens: SysTickTokens {
        sys_tick: tokens.sys_tick,
        stk_ctrl: tokens.stk_ctrl.into(),
        stk_load: tokens.stk_load,
        stk_val: tokens.stk_val,
      },
    }
  }

  #[inline]
  fn sleep(
    &mut self,
    duration: Self::Duration,
    mut ctrl_val: <Self::Ctrl as Reg<Ftt>>::Val,
  ) -> RoutineFuture<(), ()> {
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

  #[inline]
  fn interval(
    &mut self,
    duration: Self::Duration,
    mut ctrl_val: <Self::Ctrl as Reg<Ftt>>::Val,
  ) -> unit::Receiver<()> {
    ctrl_val = disable(&mut self.tokens.stk_ctrl.hold(ctrl_val)).val();
    self.tokens.stk_ctrl.store_val(ctrl_val);
    schedule(&self.tokens.stk_load, &self.tokens.stk_val, duration);
    let stream = self.tokens.sys_tick.stream_skip(|| loop {
      yield Some(());
    });
    ctrl_val = enable(&mut self.tokens.stk_ctrl.hold(ctrl_val)).val();
    self.tokens.stk_ctrl.store_val(ctrl_val);
    stream
  }

  #[inline(always)]
  fn stop(&mut self, mut ctrl_val: <Self::Ctrl as Reg<Ftt>>::Val) {
    ctrl_val = disable(&mut self.tokens.stk_ctrl.hold(ctrl_val)).val();
    self.tokens.stk_ctrl.store_val(ctrl_val);
  }
}

#[inline(always)]
fn schedule(stk_load: &stk::Load<Stt>, stk_val: &stk::Val<Stt>, duration: u32) {
  stk_load.reset(|r| r.write_reload(duration));
  stk_val.reset(|r| r.write_current(0));
}

#[inline(always)]
fn enable<'a, 'b>(
  ctrl: &'a mut stk::ctrl::Hold<'b, Ftt>,
) -> &'a mut stk::ctrl::Hold<'b, Ftt> {
  ctrl.set_enable().set_tickint()
}

#[inline(always)]
fn disable<'a, 'b>(
  ctrl: &'a mut stk::ctrl::Hold<'b, Ftt>,
) -> &'a mut stk::ctrl::Hold<'b, Ftt> {
  ctrl.clear_enable().clear_tickint()
}
