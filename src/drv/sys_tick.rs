//! SysTick timer.

use crate::{
    drv::timer::{Timer, TimerInterval, TimerOverflow, TimerSleep, TimerStop},
    fib::{self, Fiber},
    map::{
        periph::sys_tick::SysTickPeriph,
        reg::{scb, stk},
        thr::IntSysTick,
    },
    reg::{prelude::*, WWRegFieldBit},
    thr::prelude::*,
};
use core::{pin::Pin, ptr::write_volatile};
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
        $crate::drv::sys_tick::SysTick::new($crate::periph_sys_tick!($reg), $int)
    };
}

impl<I: IntSysTick<Att>> Timer for SysTick<I> {
    type Stop = Self;

    fn sleep(&mut self, duration: usize) -> TimerSleep<'_, Self> {
        let ctrl = self.periph.stk_ctrl;
        let pendstclr = self.periph.scb_icsr_pendstclr;
        let fut = Box::pin(self.int.add_future(fib::new(move || {
            loop {
                let mut ctrl_val = ctrl.load();
                if ctrl_val.countflag() {
                    ctrl.store_val(disable(&mut ctrl_val).val());
                    unsafe { set_bit(&pendstclr) };
                    break;
                }
                yield;
            }
        })));
        schedule(&self.periph.stk_load, &self.periph.stk_val, duration);
        let mut ctrl_val = self.periph.stk_ctrl.load();
        self.periph.stk_ctrl.store_val(enable(&mut ctrl_val).val());
        TimerSleep::new(self, fut)
    }

    fn interval(&mut self, duration: usize) -> TimerInterval<'_, Self, Result<(), TimerOverflow>> {
        self.interval_stream(duration, |int, ctrl| {
            Box::pin(int.add_stream(|| Err(TimerOverflow), Self::interval_fib(ctrl)))
        })
    }

    fn interval_skip(&mut self, duration: usize) -> TimerInterval<'_, Self, ()> {
        self.interval_stream(duration, |int, ctrl| {
            Box::pin(int.add_stream_skip(Self::interval_fib(ctrl)))
        })
    }
}

impl<I: IntSysTick<Att>> TimerStop for SysTick<I> {
    fn stop(&mut self) {
        let mut ctrl_val = self.periph.stk_ctrl.load();
        self.periph.stk_ctrl.store_val(disable(&mut ctrl_val).val());
    }
}

impl<I: IntSysTick<Att>> SysTick<I> {
    /// Creates a new [`SysTick`].
    #[inline]
    pub fn new(periph: SysTickPeriph, int: I) -> Self {
        let periph = SysTickDiverged {
            scb_icsr_pendstclr: periph.scb_icsr_pendstclr.into_copy(),
            scb_icsr_pendstset: periph.scb_icsr_pendstset,
            stk_ctrl: periph.stk_ctrl.into_copy(),
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
    #[inline]
    pub unsafe fn from_diverged(periph: SysTickDiverged, int: I) -> Self {
        Self { periph, int }
    }

    /// Releases the peripheral.
    #[inline]
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

    #[inline]
    fn interval_stream<'a, T: 'a>(
        &'a mut self,
        duration: usize,
        f: impl FnOnce(I, stk::Ctrl<Crt>) -> Pin<Box<dyn Stream<Item = T> + Send + 'a>>,
    ) -> TimerInterval<'a, Self, T> {
        let stream = f(self.int, self.periph.stk_ctrl);
        schedule(&self.periph.stk_load, &self.periph.stk_val, duration);
        let mut ctrl_val = self.periph.stk_ctrl.load();
        self.periph.stk_ctrl.store_val(enable(&mut ctrl_val).val());
        TimerInterval::new(self, stream)
    }

    #[inline]
    fn interval_fib<T>(
        ctrl: stk::Ctrl<Crt>,
    ) -> impl Fiber<Input = (), Yield = Option<()>, Return = T> {
        fib::new(move || {
            loop {
                yield if ctrl.load().countflag() {
                    Some(())
                } else {
                    None
                };
            }
        })
    }
}

#[allow(missing_docs)]
impl<I: IntSysTick<Att>> SysTick<I> {
    #[inline]
    pub fn int(&self) -> I {
        self.int
    }

    #[inline]
    pub fn ctrl(&self) -> &stk::Ctrl<Srt> {
        &self.periph.stk_ctrl.as_sync()
    }

    #[inline]
    pub fn load(&self) -> &stk::Load<Srt> {
        &self.periph.stk_load
    }

    #[inline]
    pub fn val(&self) -> &stk::Val<Srt> {
        &self.periph.stk_val
    }
}

#[inline]
fn schedule(stk_load: &stk::Load<Srt>, stk_val: &stk::Val<Srt>, duration: usize) {
    stk_load.store(|r| r.write_reload(duration as u32));
    stk_val.store(|r| r.write_current(0));
}

#[inline]
fn enable<'a, 'b>(ctrl: &'a mut stk::ctrl::Hold<'b, Crt>) -> &'a mut stk::ctrl::Hold<'b, Crt> {
    ctrl.set_enable().set_tickint()
}

#[inline]
fn disable<'a, 'b>(ctrl: &'a mut stk::ctrl::Hold<'b, Crt>) -> &'a mut stk::ctrl::Hold<'b, Crt> {
    ctrl.clear_enable().clear_tickint()
}

#[inline]
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
