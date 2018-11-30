use super::{Timer, TimerOverflow, TimerRes};
use core::ptr::write_volatile;
use drone_core::bitfield::Bitfield;
use fib::{self, FiberFuture, FiberStreamUnit};
use futures::prelude::*;
use map::reg::{scb, stk};
use map::thr::IntSysTick;
use reg::prelude::*;
use thr::prelude::*;

/// SysTick driver.
pub type SysTick<I> = Timer<SysTickRes<I>>;

/// SysTick resource.
#[allow(missing_docs)]
pub struct SysTickRes<I: IntSysTick<Att>> {
  pub sys_tick: I,
  pub scb_icsr_pendstclr: scb::icsr::Pendstclr<Crt>,
  pub scb_icsr_pendstset: scb::icsr::Pendstset<Srt>,
  pub stk_ctrl: stk::Ctrl<Crt>,
  pub stk_load: stk::Load<Srt>,
  pub stk_val: stk::Val<Srt>,
}

/// Creates a new `SysTick`.
#[macro_export]
macro_rules! drv_sys_tick {
  ($reg:ident, $thr:ident) => {
    unsafe {
      $crate::drv::timer::Timer::new($crate::drv::timer::SysTickRes {
        sys_tick: $thr.sys_tick.to_attach(),
        scb_icsr_pendstclr: $reg.scb_icsr.pendstclr.acquire_copy(),
        scb_icsr_pendstset: $reg.scb_icsr.pendstset,
        stk_ctrl: $reg.stk_ctrl.acquire_copy(),
        stk_load: $reg.stk_load,
        stk_val: $reg.stk_val,
      })
    }
  };
}

impl<I: IntSysTick<Att>> TimerRes for SysTickRes<I> {
  type Duration = u32;
  type CtrlVal = stk::ctrl::Val;
  type SleepFuture = FiberFuture<(), !>;
  type IntervalStream = FiberStreamUnit<TimerOverflow>;
  type IntervalSkipStream = FiberStreamUnit<!>;

  #[inline]
  fn sleep(
    &mut self,
    dur: Self::Duration,
    mut ctrl_val: Self::CtrlVal,
  ) -> Self::SleepFuture {
    ctrl_val = disable(&mut self.stk_ctrl.hold(ctrl_val)).val();
    self.stk_ctrl.store_val(ctrl_val);
    schedule(&self.stk_load, &self.stk_val, dur);
    let ctrl = self.stk_ctrl;
    let pendstclr = self.scb_icsr_pendstclr;
    let fut = self.sys_tick.add_future(fib::new_fn(move || {
      ctrl.store_val(ctrl_val);
      unsafe { set_bit(&pendstclr) };
      Ok(())
    }));
    ctrl_val = enable(&mut self.stk_ctrl.hold(ctrl_val)).val();
    self.stk_ctrl.store_val(ctrl_val);
    fut
  }

  #[inline]
  fn interval(
    &mut self,
    dur: Self::Duration,
    ctrl_val: Self::CtrlVal,
  ) -> Self::IntervalStream {
    self.interval_stream(dur, ctrl_val, |int| {
      int.add_stream(
        || Err(TimerOverflow),
        fib::new(|| loop {
          yield Some(());
        }),
      )
    })
  }

  #[inline]
  fn interval_skip(
    &mut self,
    dur: Self::Duration,
    ctrl_val: Self::CtrlVal,
  ) -> Self::IntervalSkipStream {
    self.interval_stream(dur, ctrl_val, |int| {
      int.add_stream_skip(fib::new(|| loop {
        yield Some(());
      }))
    })
  }

  #[inline]
  fn stop(&mut self, mut ctrl_val: Self::CtrlVal) {
    ctrl_val = disable(&mut self.stk_ctrl.hold(ctrl_val)).val();
    self.stk_ctrl.store_val(ctrl_val);
  }
}

impl<I: IntSysTick<Att>> SysTickRes<I> {
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
impl<I: IntSysTick<Att>> Timer<SysTickRes<I>> {
  #[inline(always)]
  pub fn int(&self) -> I {
    self.0.sys_tick
  }

  #[inline(always)]
  pub fn ctrl(&self) -> &stk::Ctrl<Srt> {
    &self.0.stk_ctrl.as_sync()
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

impl<I: IntSysTick<Att>> Timer<SysTickRes<I>> {
  /// Change SysTick exception state to pending.
  #[inline]
  pub fn set_pending(&self) {
    unsafe { set_bit(&self.0.scb_icsr_pendstset) };
  }

  /// Returns `true` if SysTick exception is pending.
  #[inline]
  pub fn is_pending(&self) -> bool {
    self.0.scb_icsr_pendstset.read_bit()
  }

  /// Removes the pending state from the SysTick exception.
  #[inline]
  pub fn clear_pending(&self) {
    unsafe { set_bit(&self.0.scb_icsr_pendstclr) };
  }
}

#[inline]
fn schedule(stk_load: &stk::Load<Srt>, stk_val: &stk::Val<Srt>, dur: u32) {
  stk_load.store(|r| r.write_reload(dur));
  stk_val.store(|r| r.write_current(0));
}

#[inline]
fn enable<'a, 'b>(
  ctrl: &'a mut stk::ctrl::Hold<'b, Crt>,
) -> &'a mut stk::ctrl::Hold<'b, Crt> {
  ctrl.set_enable().set_tickint()
}

#[inline]
fn disable<'a, 'b>(
  ctrl: &'a mut stk::ctrl::Hold<'b, Crt>,
) -> &'a mut stk::ctrl::Hold<'b, Crt> {
  ctrl.clear_enable().clear_tickint()
}

#[inline(always)]
unsafe fn set_bit<F, T>(field: &F)
where
  F: WWRegFieldBit<T>,
  F::Reg: WReg<T>,
  T: RegTag,
{
  let mut val = <F::Reg as Reg<T>>::Val::default();
  field.set(&mut val);
  write_volatile(
    F::Reg::ADDRESS as *mut <<F::Reg as Reg<T>>::Val as Bitfield>::Bits,
    val.bits(),
  );
}
