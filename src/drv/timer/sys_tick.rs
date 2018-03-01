use super::{Timer, TimerOverflow, TimerRes};
use drv::prelude::*;
use fib::{self, FiberFuture, FiberStreamUnit};
use reg::{scb, stk};
use reg::prelude::*;
use thr::int::IntSysTick;
use thr::prelude::*;

/// SysTick driver.
pub type SysTick<I> = Timer<SysTickRes<I, Frt>>;

/// SysTick resource.
#[allow(missing_docs)]
pub struct SysTickRes<I: IntSysTick<Ltt>, Rt: RegTag> {
  pub sys_tick: I,
  pub scb_icsr_pendstclr: scb::icsr::Pendstclr<Rt>,
  pub scb_icsr_pendstset: scb::icsr::Pendstset<Srt>,
  pub stk_ctrl: stk::Ctrl<Rt>,
  pub stk_load: stk::Load<Srt>,
  pub stk_val: stk::Val<Srt>,
}

/// Creates a new `SysTick`.
#[macro_export]
macro_rules! drv_sys_tick {
  ($reg:ident, $thr:ident) => {
    $crate::drv::timer::Timer::from_res(
      $crate::drv::timer::SysTickRes {
        sys_tick: $thr.sys_tick.into(),
        scb_icsr_pendstclr: $reg.scb_icsr.pendstclr,
        scb_icsr_pendstset: $reg.scb_icsr.pendstset,
        stk_ctrl: $reg.stk_ctrl,
        stk_load: $reg.stk_load,
        stk_val: $reg.stk_val,
      }
    )
  }
}

impl<I: IntSysTick<Ltt>> From<SysTickRes<I, Srt>> for SysTickRes<I, Frt> {
  #[inline(always)]
  fn from(res: SysTickRes<I, Srt>) -> Self {
    Self {
      sys_tick: res.sys_tick,
      scb_icsr_pendstclr: res.scb_icsr_pendstclr.into(),
      scb_icsr_pendstset: res.scb_icsr_pendstset,
      stk_ctrl: res.stk_ctrl.into(),
      stk_load: res.stk_load,
      stk_val: res.stk_val,
    }
  }
}

impl<I: IntSysTick<Ltt>> Resource for SysTickRes<I, Frt> {
  type Source = SysTickRes<I, Srt>;
}

impl<I: IntSysTick<Ltt>> TimerRes for SysTickRes<I, Frt> {
  type Duration = u32;
  type CtrlVal = stk::ctrl::Val;
  type SleepFuture = FiberFuture<(), !>;
  type IntervalStream = FiberStreamUnit<TimerOverflow>;
  type IntervalSkipStream = FiberStreamUnit<!>;

  #[inline(always)]
  fn sleep(
    &mut self,
    dur: Self::Duration,
    mut ctrl_val: Self::CtrlVal,
  ) -> Self::SleepFuture {
    ctrl_val = disable(&mut self.stk_ctrl.hold(ctrl_val)).val();
    self.stk_ctrl.store_val(ctrl_val);
    schedule(&self.stk_load, &self.stk_val, dur);
    let ctrl = self.stk_ctrl.fork();
    let pendstclr = self.scb_icsr_pendstclr.fork();
    let fut = fib::spawn_future(
      self.sys_tick,
      fib::new_fn(move || {
        ctrl.store_val(ctrl_val);
        pendstclr.store_set();
        Ok(())
      }),
    );
    ctrl_val = enable(&mut self.stk_ctrl.hold(ctrl_val)).val();
    self.stk_ctrl.store_val(ctrl_val);
    fut
  }

  #[inline(always)]
  fn interval(
    &mut self,
    dur: Self::Duration,
    ctrl_val: Self::CtrlVal,
  ) -> Self::IntervalStream {
    self.interval_stream(dur, ctrl_val, |int| {
      fib::spawn_stream(
        int,
        || Err(TimerOverflow),
        fib::new(|| loop {
          yield Some(());
        }),
      )
    })
  }

  #[inline(always)]
  fn interval_skip(
    &mut self,
    dur: Self::Duration,
    ctrl_val: Self::CtrlVal,
  ) -> Self::IntervalSkipStream {
    self.interval_stream(dur, ctrl_val, |int| {
      fib::spawn_stream_skip(
        int,
        fib::new(|| loop {
          yield Some(());
        }),
      )
    })
  }

  #[inline(always)]
  fn stop(&mut self, mut ctrl_val: Self::CtrlVal) {
    ctrl_val = disable(&mut self.stk_ctrl.hold(ctrl_val)).val();
    self.stk_ctrl.store_val(ctrl_val);
  }
}

impl<I: IntSysTick<Ltt>> SysTickRes<I, Frt> {
  fn interval_stream<F, S>(
    &mut self,
    dur: u32,
    mut ctrl_val: stk::ctrl::Val,
    f: F,
  ) -> S
  where
    F: FnOnce(I) -> S,
    S: Stream,
  {
    ctrl_val = disable(&mut self.stk_ctrl.hold(ctrl_val)).val();
    self.stk_ctrl.store_val(ctrl_val);
    schedule(&self.stk_load, &self.stk_val, dur);
    let stream = f(self.sys_tick);
    ctrl_val = enable(&mut self.stk_ctrl.hold(ctrl_val)).val();
    self.stk_ctrl.store_val(ctrl_val);
    stream
  }
}

#[allow(missing_docs)]
impl<I: IntSysTick<Ltt>> Timer<SysTickRes<I, Frt>> {
  #[inline(always)]
  pub fn int(&self) -> I {
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

  /// Change SysTick exception state to pending.
  #[inline(always)]
  pub fn set_pending(&self) {
    self.0.scb_icsr_pendstset.store_set();
  }

  /// Returns `true` if SysTick exception is pending.
  #[inline(always)]
  pub fn is_pending(&self) -> bool {
    self.0.scb_icsr_pendstset.read_bit()
  }

  /// Removes the pending state from the SysTick exception.
  #[inline(always)]
  pub fn clear_pending(&self) {
    self.0.scb_icsr_pendstclr.store_set();
  }
}

#[inline(always)]
fn schedule(stk_load: &stk::Load<Srt>, stk_val: &stk::Val<Srt>, dur: u32) {
  stk_load.store(|r| r.write_reload(dur));
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
