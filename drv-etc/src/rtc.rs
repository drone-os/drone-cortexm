//! Real-time clock.

use drone_stm32_device::reg::prelude::*;
use drone_stm32_device::reg::{rcc, rtc};

/// Real-time clock driver.
#[derive(Driver)]
pub struct Rtc(RtcRes);

/// Real-time clock resource.
#[allow(missing_docs)]
#[derive(Resource)]
pub struct RtcRes {
  pub rcc_bdcr_rtcen: rcc::bdcr::Rtcen<Srt>,
  pub rcc_bdcr_rtcsel: rcc::bdcr::Rtcsel<Srt>,
  pub rtc_tr: rtc::Tr<Srt>,
  pub rtc_dr: rtc::Dr<Srt>,
  pub rtc_cr: rtc::Cr<Srt>,
  pub rtc_isr: rtc::Isr<Srt>,
  pub rtc_prer: rtc::Prer<Srt>,
  pub rtc_wutr: rtc::Wutr<Srt>,
  pub rtc_alrmar: rtc::Alrmar<Srt>,
  pub rtc_alrmbr: rtc::Alrmbr<Srt>,
  pub rtc_wpr: rtc::Wpr<Srt>,
  pub rtc_ssr: rtc::Ssr<Srt>,
  pub rtc_shiftr: rtc::Shiftr<Srt>,
  pub rtc_tstr: rtc::Tstr<Srt>,
  pub rtc_tsdr: rtc::Tsdr<Srt>,
  pub rtc_tsssr: rtc::Tsssr<Srt>,
  pub rtc_calr: rtc::Calr<Srt>,
  pub rtc_tampcr: rtc::Tampcr<Srt>,
  pub rtc_alrmassr: rtc::Alrmassr<Srt>,
  pub rtc_alrmbssr: rtc::Alrmbssr<Srt>,
  pub rtc_or: rtc::Or<Srt>,
  pub rtc_bkp0r: rtc::Bkp0R<Srt>,
  pub rtc_bkp31r: rtc::Bkp31R<Srt>,
}

/// Creates a new `Rtc`.
#[macro_export]
macro_rules! drv_rtc {
  ($reg:ident) => {
    <$crate::rtc::Rtc as ::drone_core::drv::Driver>::new($crate::rtc::RtcRes {
      rcc_bdcr_rtcen: $reg.rcc_bdcr.rtcen,
      rcc_bdcr_rtcsel: $reg.rcc_bdcr.rtcsel,
      rtc_tr: $reg.rtc_tr,
      rtc_dr: $reg.rtc_dr,
      rtc_cr: $reg.rtc_cr,
      rtc_isr: $reg.rtc_isr,
      rtc_prer: $reg.rtc_prer,
      rtc_wutr: $reg.rtc_wutr,
      rtc_alrmar: $reg.rtc_alrmar,
      rtc_alrmbr: $reg.rtc_alrmbr,
      rtc_wpr: $reg.rtc_wpr,
      rtc_ssr: $reg.rtc_ssr,
      rtc_shiftr: $reg.rtc_shiftr,
      rtc_tstr: $reg.rtc_tstr,
      rtc_tsdr: $reg.rtc_tsdr,
      rtc_tsssr: $reg.rtc_tsssr,
      rtc_calr: $reg.rtc_calr,
      rtc_tampcr: $reg.rtc_tampcr,
      rtc_alrmassr: $reg.rtc_alrmassr,
      rtc_alrmbssr: $reg.rtc_alrmbssr,
      rtc_or: $reg.rtc_or,
      rtc_bkp0r: $reg.rtc_bkp0r,
      rtc_bkp31r: $reg.rtc_bkp31r,
    })
  };
}

#[allow(missing_docs)]
impl Rtc {
  #[inline(always)]
  pub fn rcc_bdcr_rtcen(&self) -> &rcc::bdcr::Rtcen<Srt> {
    &self.0.rcc_bdcr_rtcen
  }

  #[inline(always)]
  pub fn rcc_bdcr_rtcsel(&self) -> &rcc::bdcr::Rtcsel<Srt> {
    &self.0.rcc_bdcr_rtcsel
  }

  #[inline(always)]
  pub fn rtc_tr(&self) -> &rtc::Tr<Srt> {
    &self.0.rtc_tr
  }

  #[inline(always)]
  pub fn rtc_dr(&self) -> &rtc::Dr<Srt> {
    &self.0.rtc_dr
  }

  #[inline(always)]
  pub fn rtc_cr(&self) -> &rtc::Cr<Srt> {
    &self.0.rtc_cr
  }

  #[inline(always)]
  pub fn rtc_isr(&self) -> &rtc::Isr<Srt> {
    &self.0.rtc_isr
  }

  #[inline(always)]
  pub fn rtc_prer(&self) -> &rtc::Prer<Srt> {
    &self.0.rtc_prer
  }

  #[inline(always)]
  pub fn rtc_wutr(&self) -> &rtc::Wutr<Srt> {
    &self.0.rtc_wutr
  }

  #[inline(always)]
  pub fn rtc_alrmar(&self) -> &rtc::Alrmar<Srt> {
    &self.0.rtc_alrmar
  }

  #[inline(always)]
  pub fn rtc_alrmbr(&self) -> &rtc::Alrmbr<Srt> {
    &self.0.rtc_alrmbr
  }

  #[inline(always)]
  pub fn rtc_wpr(&self) -> &rtc::Wpr<Srt> {
    &self.0.rtc_wpr
  }

  #[inline(always)]
  pub fn rtc_ssr(&self) -> &rtc::Ssr<Srt> {
    &self.0.rtc_ssr
  }

  #[inline(always)]
  pub fn rtc_shiftr(&self) -> &rtc::Shiftr<Srt> {
    &self.0.rtc_shiftr
  }

  #[inline(always)]
  pub fn rtc_tstr(&self) -> &rtc::Tstr<Srt> {
    &self.0.rtc_tstr
  }

  #[inline(always)]
  pub fn rtc_tsdr(&self) -> &rtc::Tsdr<Srt> {
    &self.0.rtc_tsdr
  }

  #[inline(always)]
  pub fn rtc_tsssr(&self) -> &rtc::Tsssr<Srt> {
    &self.0.rtc_tsssr
  }

  #[inline(always)]
  pub fn rtc_calr(&self) -> &rtc::Calr<Srt> {
    &self.0.rtc_calr
  }

  #[inline(always)]
  pub fn rtc_tampcr(&self) -> &rtc::Tampcr<Srt> {
    &self.0.rtc_tampcr
  }

  #[inline(always)]
  pub fn rtc_alrmassr(&self) -> &rtc::Alrmassr<Srt> {
    &self.0.rtc_alrmassr
  }

  #[inline(always)]
  pub fn rtc_alrmbssr(&self) -> &rtc::Alrmbssr<Srt> {
    &self.0.rtc_alrmbssr
  }

  #[inline(always)]
  pub fn rtc_or(&self) -> &rtc::Or<Srt> {
    &self.0.rtc_or
  }

  #[inline(always)]
  pub fn rtc_bkp0r(&self) -> &rtc::Bkp0R<Srt> {
    &self.0.rtc_bkp0r
  }

  #[inline(always)]
  pub fn rtc_bkp31r(&self) -> &rtc::Bkp31R<Srt> {
    &self.0.rtc_bkp31r
  }
}

impl Rtc {
  /// Disables the write protection.
  pub fn unlock(&self) {
    self.0.rtc_wpr.store(|r| r.write_key(0xCA));
    self.0.rtc_wpr.store(|r| r.write_key(0x53));
  }

  /// Reactivates the write protection.
  pub fn lock(&self) {
    self.0.rtc_wpr.store(|r| r.write_key(0x00));
  }
}
