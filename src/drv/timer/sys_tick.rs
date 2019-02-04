use super::{Timer, TimerOverflow};
use crate::{
  fib::{self, FiberFuture, FiberStreamUnit, TryFiberStreamUnit},
  map::{
    periph::sys_tick::SysTickPeriph,
    reg::{scb, stk},
    thr::IntSysTick,
  },
  reg::prelude::*,
  thr::prelude::*,
};
use core::ptr::write_volatile;
use drone_core::bitfield::Bitfield;
use futures::stream::Stream;

/// SysTick driver.
#[allow(missing_docs)]
pub struct SysTick<I: IntSysTick<Att>> {
  periph: SysTickDiverged,
  int: I,
}

/// SysTick diverged peripheral.
#[allow(missing_docs)]
pub struct SysTickDiverged {
  pub scb_icsr_pendstclr: scb::icsr::Pendstclr<Crt>,
  pub scb_icsr_pendstset: scb::icsr::Pendstset<Srt>,
  pub stk_ctrl: stk::Ctrl<Crt>,
  pub stk_load: stk::Load<Srt>,
  pub stk_val: stk::Val<Srt>,
}

/// Acquires [`SysTick`].
#[macro_export]
macro_rules! drv_sys_tick {
  ($reg:ident, $int:expr) => {
    $crate::drv::timer::SysTick::new($crate::periph_sys_tick!($reg), $int)
  };
}

impl<I: IntSysTick<Att>> Timer for SysTick<I> {
  type Duration = u32;
  type CtrlVal = stk::ctrl::Val;
  type SleepFuture = FiberFuture<()>;
  type IntervalStream = TryFiberStreamUnit<TimerOverflow>;
  type IntervalSkipStream = FiberStreamUnit;

  #[inline]
  fn sleep(
    &mut self,
    dur: Self::Duration,
    mut ctrl_val: Self::CtrlVal,
  ) -> Self::SleepFuture {
    ctrl_val = disable(&mut self.periph.stk_ctrl.hold(ctrl_val)).val();
    self.periph.stk_ctrl.store_val(ctrl_val);
    schedule(&self.periph.stk_load, &self.periph.stk_val, dur);
    let ctrl = self.periph.stk_ctrl;
    let pendstclr = self.periph.scb_icsr_pendstclr;
    let fut = self.int.add_future(fib::new_fn(move || {
      ctrl.store_val(ctrl_val);
      unsafe { set_bit(&pendstclr) };
    }));
    ctrl_val = enable(&mut self.periph.stk_ctrl.hold(ctrl_val)).val();
    self.periph.stk_ctrl.store_val(ctrl_val);
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
    ctrl_val = disable(&mut self.periph.stk_ctrl.hold(ctrl_val)).val();
    self.periph.stk_ctrl.store_val(ctrl_val);
  }
}

impl<I: IntSysTick<Att>> SysTick<I> {
  /// Creates a new [`SysTick`].
  #[inline(always)]
  pub fn new(periph: SysTickPeriph, int: I) -> Self {
    let periph = SysTickDiverged {
      scb_icsr_pendstclr: periph.scb_icsr_pendstclr.to_copy(),
      scb_icsr_pendstset: periph.scb_icsr_pendstset,
      stk_ctrl: periph.stk_ctrl.to_copy(),
      stk_load: periph.stk_load,
      stk_val: periph.stk_val,
    };
    Self { periph, int }
  }

  /// Creates a new [`SysTick`].
  ///
  /// # Safety
  ///
  /// Some of the `Crt` register tokens can be still in use.
  #[inline(always)]
  pub unsafe fn from_diverged(periph: SysTickDiverged, int: I) -> Self {
    Self { periph, int }
  }

  /// Releases the peripheral.
  #[inline(always)]
  pub fn free(self) -> SysTickDiverged {
    self.periph
  }

  /// Change SysTick exception state to pending.
  #[inline]
  pub fn set_pending(&self) {
    unsafe { set_bit(&self.periph.scb_icsr_pendstset) };
  }

  /// Returns `true` if SysTick exception is pending.
  #[inline]
  pub fn is_pending(&self) -> bool {
    self.periph.scb_icsr_pendstset.read_bit()
  }

  /// Removes the pending state from the SysTick exception.
  #[inline]
  pub fn clear_pending(&self) {
    unsafe { set_bit(&self.periph.scb_icsr_pendstclr) };
  }

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
    ctrl_val = disable(&mut self.periph.stk_ctrl.hold(ctrl_val)).val();
    self.periph.stk_ctrl.store_val(ctrl_val);
    schedule(&self.periph.stk_load, &self.periph.stk_val, dur);
    let stream = f(self.int);
    ctrl_val = enable(&mut self.periph.stk_ctrl.hold(ctrl_val)).val();
    self.periph.stk_ctrl.store_val(ctrl_val);
    stream
  }
}

#[allow(missing_docs)]
impl<I: IntSysTick<Att>> SysTick<I> {
  #[inline(always)]
  pub fn int(&self) -> I {
    self.int
  }

  #[inline(always)]
  pub fn ctrl(&self) -> &stk::Ctrl<Srt> {
    &self.periph.stk_ctrl.as_sync()
  }

  #[inline(always)]
  pub fn load(&self) -> &stk::Load<Srt> {
    &self.periph.stk_load
  }

  #[inline(always)]
  pub fn val(&self) -> &stk::Val<Srt> {
    &self.periph.stk_val
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
