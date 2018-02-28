//! Instrumentation Trace Macrocell.

use drivers::prelude::*;
use reg::{itm, scb, tpiu};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107"))]
use reg::dbg as dbgmcu;
#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
use reg::dbgmcu;
use reg::prelude::*;

/// ITM driver.
pub struct Itm(ItmRes);

/// ITM resource.
#[allow(missing_docs)]
pub struct ItmRes {
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub dbgmcu_cr: dbgmcu::Cr<Srt>,
  pub itm_lar: itm::Lar<Srt>,
  pub itm_tcr: itm::Tcr<Srt>,
  pub itm_tpr: itm::Tpr<Srt>,
  pub scb_demcr_trcena: scb::demcr::Trcena<Srt>,
  pub tpiu_ffcr: tpiu::Ffcr<Srt>,
  pub tpiu_sppr: tpiu::Sppr<Srt>,
}

/// Creates a new `Itm`.
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
#[macro_export]
macro_rules! drv_itm {
  ($reg:ident) => {
    $crate::drivers::itm::Itm::from_res(
      $crate::drivers::itm::ItmRes {
        dbgmcu_cr: $reg.dbgmcu_cr,
        itm_lar: $reg.itm_lar,
        itm_tcr: $reg.itm_tcr,
        itm_tpr: $reg.itm_tpr,
        scb_demcr_trcena: $reg.scb_demcr.trcena,
        tpiu_ffcr: $reg.tpiu_ffcr,
        tpiu_sppr: $reg.tpiu_sppr,
      }
    )
  }
}

impl Driver for Itm {
  type Resource = ItmRes;

  #[inline(always)]
  fn from_res(res: ItmRes) -> Self {
    Itm(res)
  }

  #[inline(always)]
  fn into_res(self) -> ItmRes {
    self.0
  }
}

impl Resource for ItmRes {
  // FIXME https://github.com/rust-lang/rust/issues/47385
  type Input = Self;
}

#[allow(missing_docs)]
impl Itm {
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub fn dbgmcu_cr(&self) -> &dbgmcu::Cr<Srt> {
    &self.0.dbgmcu_cr
  }

  #[inline(always)]
  pub fn itm_lar(&self) -> &itm::Lar<Srt> {
    &self.0.itm_lar
  }

  #[inline(always)]
  pub fn itm_tcr(&self) -> &itm::Tcr<Srt> {
    &self.0.itm_tcr
  }

  #[inline(always)]
  pub fn itm_tpr(&self) -> &itm::Tpr<Srt> {
    &self.0.itm_tpr
  }

  #[inline(always)]
  pub fn scb_demcr_trcena(&self) -> &scb::demcr::Trcena<Srt> {
    &self.0.scb_demcr_trcena
  }

  #[inline(always)]
  pub fn tpiu_ffcr(&self) -> &tpiu::Ffcr<Srt> {
    &self.0.tpiu_ffcr
  }

  #[inline(always)]
  pub fn tpiu_sppr(&self) -> &tpiu::Sppr<Srt> {
    &self.0.tpiu_sppr
  }

  /// Initializes ITM.
  pub fn init(&self) {
    // FIXME better to set this via debugger.
    #[cfg(any(feature = "stm32f100", feature = "stm32f101",
              feature = "stm32f102", feature = "stm32f103",
              feature = "stm32f107", feature = "stm32l4x1",
              feature = "stm32l4x2", feature = "stm32l4x3",
              feature = "stm32l4x5", feature = "stm32l4x6"))]
    self.0.dbgmcu_cr.store(|r| {
      r.write_trace_mode(0b00)
        .set_trace_ioen()
        .set_dbg_standby()
        .set_dbg_stop()
        .set_dbg_sleep()
    });
    self.0.scb_demcr_trcena.set_bit();
    self.0.tpiu_sppr.store(|r| r.write_txmode(0b10));
    self.0.tpiu_ffcr.store(|r| r.clear_en_f_cont());
    self.itm_unlock();
    self
      .0
      .itm_tcr
      .modify(|r| r.write_trace_bus_id(1).set_itmena());
    self.0.itm_tpr.store(|r| r.write_privmask(0x0000_0001));
  }

  /// Unlock Write Access to ITM registers.
  #[inline(always)]
  pub fn itm_unlock(&self) {
    self.0.itm_lar.store(|r| r.write_unlock(0xC5AC_CE55))
  }
}
