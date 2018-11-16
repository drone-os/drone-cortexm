//! Inter-Integrated Circuit.

use core::marker::PhantomData;
use drone_core::bitfield::Bitfield;
use drone_core::drv::Resource;
use drone_stm32_core::fib;
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drone_stm32_device::reg::i2c4;
use drone_stm32_device::reg::marker::*;
use drone_stm32_device::reg::prelude::*;
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drone_stm32_device::reg::{i2c1, i2c2, i2c3, rcc};
use drone_stm32_device::reg::{RegGuard, RegGuardCnt, RegGuardRes};
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6"
))]
use drone_stm32_device::thr::int::{
  IntDma1Ch2, IntDma1Ch3, IntDma1Ch4, IntDma1Ch5, IntDma1Ch6, IntDma1Ch7,
  IntDma2Ch1, IntDma2Ch2, IntDma2Ch6, IntDma2Ch7,
};
#[cfg(any(feature = "stm32l4x3", feature = "stm32l4x5"))]
use drone_stm32_device::thr::int::{
  IntDma1Channel2 as IntDma1Ch2, IntDma1Channel3 as IntDma1Ch3,
  IntDma1Channel4 as IntDma1Ch4, IntDma1Channel5 as IntDma1Ch5,
  IntDma1Channel6 as IntDma1Ch6, IntDma1Channel7 as IntDma1Ch7,
  IntDma2Channel6 as IntDma2Ch6, IntDma2Channel7 as IntDma2Ch7,
};
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drone_stm32_device::thr::int::{
  IntI2C1Er, IntI2C1Ev, IntI2C2Er, IntI2C2Ev, IntI2C3Er, IntI2C3Ev,
};
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drone_stm32_device::thr::int::{IntI2C4Er, IntI2C4Ev};
use drone_stm32_device::thr::prelude::*;
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
use drone_stm32_drv_dma::dma::{
  Dma, Dma1Ch2Bond, Dma1Ch2Res, Dma1Ch3Bond, Dma1Ch3Res, Dma1Ch4Bond,
  Dma1Ch4Res, Dma1Ch5Bond, Dma1Ch5Res, Dma1Ch6Bond, Dma1Ch6Res, Dma1Ch7Bond,
  Dma1Ch7Res, Dma2Ch6Bond, Dma2Ch6Res, Dma2Ch7Bond, Dma2Ch7Res, DmaRes,
};
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6",
))]
use drone_stm32_drv_dma::dma::{
  Dma2Ch1Bond, Dma2Ch1Res, Dma2Ch2Bond, Dma2Ch2Res,
};
use drone_stm32_drv_dma::dma::{DmaBond, DmaBondOnRgc, DmaTxRes};
#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drone_stm32_drv_dmamux::dmamux::{DmamuxCh, DmamuxChRes};
use futures::prelude::*;

/// I2C error.
#[derive(Debug, Fail)]
pub enum I2CError {
  /// Bus error.
  #[fail(display = "I2C bus error.")]
  Berr,
  /// Overrun/Underrun.
  #[fail(display = "I2C overrun.")]
  Ovr,
  /// Arbitration lost.
  #[fail(display = "I2C arbitration lost.")]
  Arlo,
  /// Timeout or t_low detection flag.
  #[fail(display = "I2C timeout.")]
  Timeout,
  /// SMBus alert.
  #[fail(display = "I2C SMBus alert.")]
  Alert,
  /// PEC error in reception.
  #[fail(display = "I2C PEC error.")]
  Pecerr,
}

/// I2C transfer failure event.
#[derive(Debug, Fail)]
pub enum I2CBreak {
  /// NACK reception.
  #[fail(display = "I2C NACK received.")]
  Nack,
  /// Stop reception.
  #[fail(display = "I2C STOP received.")]
  Stop,
}

/// I2C driver.
#[derive(Driver)]
pub struct I2C<T, C>(T, PhantomData<C>)
where
  T: I2CRes,
  C: RegGuardCnt<I2COn<T>>;

/// I2C resource.
#[allow(missing_docs)]
pub trait I2CRes:
  Resource + I2CResCr1 + I2CResCr2 + I2CResIsr + I2CResIcr
{
  type IntEv: IntToken<Ttt>;
  type IntEr: IntToken<Ttt>;
  type Oar1: SRwRegBitBand;
  type Oar2: SRwRegBitBand;
  type Timingr: SRwRegBitBand;
  type Timeoutr: SRwRegBitBand;
  type Pecr: SRoRegBitBand;
  type RxdrVal: Bitfield<Bits = u32>;
  type Rxdr: SRoRegBitBand<Val = Self::RxdrVal>;
  type RxdrRxdata: SRoRoRegFieldBits<Reg = Self::Rxdr>;
  type TxdrVal: Bitfield<Bits = u32>;
  type Txdr: SRwRegBitBand<Val = Self::TxdrVal>;
  type TxdrTxdata: SRwRwRegFieldBits<Reg = Self::Txdr>;
  type RccApbEnrVal: Bitfield<Bits = u32>;
  type RccApbEnr: CRwRegBitBand<Val = Self::RccApbEnrVal>;
  type RccApbEnrI2CEn: CRwRwRegFieldBitBand<Reg = Self::RccApbEnr>;

  fn int_ev(&self) -> Self::IntEv;
  fn int_er(&self) -> Self::IntEr;

  res_decl!(Oar1, oar1);
  res_decl!(Oar2, oar2);
  res_decl!(Timingr, timingr);
  res_decl!(Timeoutr, timeoutr);
  res_decl!(Pecr, pecr);
  res_decl!(Rxdr, rxdr);
  res_decl!(RxdrRxdata, rxdr_rxdata);
  res_decl!(Txdr, txdr);
  res_decl!(TxdrTxdata, txdr_txdata);
  res_decl!(RccApbEnrI2CEn, rcc_en);
}

#[allow(missing_docs)]
pub trait I2CResCr1 {
  type Cr1Val: Bitfield<Bits = u32>;
  type Cr1: SRwRegBitBand<Val = Self::Cr1Val>;
  type Cr1Pe: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Txie: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Rxie: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Addrie: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Nackie: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Stopie: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Tcie: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Errie: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Dnf: SRwRwRegFieldBits<Reg = Self::Cr1>;
  type Cr1Anfoff: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Txdmaen: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Rxdmaen: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Sbc: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Nostretch: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Wupen: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Gcen: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Smbhen: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Smbden: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Alerten: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Pecen: SRwRwRegFieldBitBand<Reg = Self::Cr1>;

  res_decl!(Cr1, cr1);
  res_decl!(Cr1Pe, cr1_pe);
  res_decl!(Cr1Txie, cr1_txie);
  res_decl!(Cr1Rxie, cr1_rxie);
  res_decl!(Cr1Addrie, cr1_addrie);
  res_decl!(Cr1Nackie, cr1_nackie);
  res_decl!(Cr1Stopie, cr1_stopie);
  res_decl!(Cr1Tcie, cr1_tcie);
  res_decl!(Cr1Errie, cr1_errie);
  res_decl!(Cr1Dnf, cr1_dnf);
  res_decl!(Cr1Anfoff, cr1_anfoff);
  res_decl!(Cr1Txdmaen, cr1_txdmaen);
  res_decl!(Cr1Rxdmaen, cr1_rxdmaen);
  res_decl!(Cr1Sbc, cr1_sbc);
  res_decl!(Cr1Nostretch, cr1_nostretch);
  res_decl!(Cr1Wupen, cr1_wupen);
  res_decl!(Cr1Gcen, cr1_gcen);
  res_decl!(Cr1Smbhen, cr1_smbhen);
  res_decl!(Cr1Smbden, cr1_smbden);
  res_decl!(Cr1Alerten, cr1_alerten);
  res_decl!(Cr1Pecen, cr1_pecen);
}

#[allow(missing_docs)]
pub trait I2CResCr2 {
  type Cr2Val: Bitfield<Bits = u32>;
  type Cr2: SRwRegBitBand<Val = Self::Cr2Val>;
  type Cr2Pecbyte: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2Autoend: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2Reload: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2Nbytes: SRwRwRegFieldBits<Reg = Self::Cr2>;
  type Cr2Nack: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2Stop: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2Start: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2Head10R: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2Add10: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2RdWrn: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2Sadd: SRwRwRegFieldBits<Reg = Self::Cr2>;

  res_decl!(Cr2, cr2);
  res_decl!(Cr2Pecbyte, cr2_pecbyte);
  res_decl!(Cr2Autoend, cr2_autoend);
  res_decl!(Cr2Reload, cr2_reload);
  res_decl!(Cr2Nbytes, cr2_nbytes);
  res_decl!(Cr2Nack, cr2_nack);
  res_decl!(Cr2Stop, cr2_stop);
  res_decl!(Cr2Start, cr2_start);
  res_decl!(Cr2Head10R, cr2_head10r);
  res_decl!(Cr2Add10, cr2_add10);
  res_decl!(Cr2RdWrn, cr2_rd_wrn);
  res_decl!(Cr2Sadd, cr2_sadd);
}

#[allow(missing_docs)]
pub trait I2CResIsr {
  type Isr: CRwRegBitBand;
  type IsrNackf: CRoRwRegFieldBitBand<Reg = Self::Isr>;
  type IsrStopf: CRoRwRegFieldBitBand<Reg = Self::Isr>;
  type IsrTc: CRoRwRegFieldBitBand<Reg = Self::Isr>;
  type IsrTcr: CRoRwRegFieldBitBand<Reg = Self::Isr>;
  type IsrBerr: CRoRwRegFieldBitBand<Reg = Self::Isr>;
  type IsrArlo: CRoRwRegFieldBitBand<Reg = Self::Isr>;
  type IsrOvr: CRoRwRegFieldBitBand<Reg = Self::Isr>;
  type IsrPecerr: CRoRwRegFieldBitBand<Reg = Self::Isr>;
  type IsrTimeout: CRoRwRegFieldBitBand<Reg = Self::Isr>;
  type IsrAlert: CRoRwRegFieldBitBand<Reg = Self::Isr>;

  res_decl!(Isr, isr);
  res_decl!(IsrNackf, isr_nackf);
  res_decl!(IsrStopf, isr_stopf);
  res_decl!(IsrTc, isr_tc);
  res_decl!(IsrTcr, isr_tcr);
  res_decl!(IsrBerr, isr_berr);
  res_decl!(IsrArlo, isr_arlo);
  res_decl!(IsrOvr, isr_ovr);
  res_decl!(IsrPecerr, isr_pecerr);
  res_decl!(IsrTimeout, isr_timeout);
  res_decl!(IsrAlert, isr_alert);
}

#[allow(missing_docs)]
pub trait I2CResIcr {
  type Icr: CWoRegBitBand;
  type IcrNackcf: CWoWoRegFieldBitBand<Reg = Self::Icr>;
  type IcrStopcf: CWoWoRegFieldBitBand<Reg = Self::Icr>;
  type IcrBerrcf: CWoWoRegFieldBitBand<Reg = Self::Icr>;
  type IcrArlocf: CWoWoRegFieldBitBand<Reg = Self::Icr>;
  type IcrOvrcf: CWoWoRegFieldBitBand<Reg = Self::Icr>;
  type IcrPeccf: CWoWoRegFieldBitBand<Reg = Self::Icr>;
  type IcrTimoutcf: CWoWoRegFieldBitBand<Reg = Self::Icr>;
  type IcrAlertcf: CWoWoRegFieldBitBand<Reg = Self::Icr>;

  res_decl!(Icr, icr);
  res_decl!(IcrNackcf, icr_nackcf);
  res_decl!(IcrStopcf, icr_stopcf);
  res_decl!(IcrBerrcf, icr_berrcf);
  res_decl!(IcrArlocf, icr_arlocf);
  res_decl!(IcrOvrcf, icr_ovrcf);
  res_decl!(IcrPeccf, icr_peccf);
  res_decl!(IcrTimoutcf, icr_timoutcf);
  res_decl!(IcrAlertcf, icr_alertcf);
}

/// DMA-driven I2C resource.
#[allow(missing_docs)]
pub trait I2CDmaRxRes<T: DmaBond>: I2CRes {
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  fn dmamux_rx_init(
    &self,
    cr_val: &mut <<T::DmamuxChRes as DmamuxChRes>::Cr as Reg<Srt>>::Val,
    dmamux: &DmamuxCh<T::DmamuxChRes>,
  );

  #[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  ))]
  fn dma_rx_ch_init(
    &self,
    cs_val: &mut <<T::DmaRes as DmaRes>::Cselr as Reg<Srt>>::Val,
    dma: &Dma<T::DmaRes>,
  );
}

/// DMA-driven I2C resource.
#[allow(missing_docs)]
pub trait I2CDmaTxRes<T: DmaBond>: I2CRes {
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  fn dmamux_tx_init(
    &self,
    cr_val: &mut <<T::DmamuxChRes as DmamuxChRes>::Cr as Reg<Srt>>::Val,
    dmamux: &DmamuxCh<T::DmamuxChRes>,
  );

  #[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  ))]
  fn dma_tx_ch_init(
    &self,
    cs_val: &mut <<T::DmaRes as DmaRes>::Cselr as Reg<Srt>>::Val,
    dma: &Dma<T::DmaRes>,
  );
}

/// I2C clock on guard resource.
pub struct I2COn<T: I2CRes>(T::RccApbEnrI2CEn);

#[allow(missing_docs)]
impl<T, C> I2C<T, C>
where
  T: I2CRes,
  C: RegGuardCnt<I2COn<T>>,
{
  #[inline(always)]
  pub fn int_ev(&self) -> T::IntEv {
    self.0.int_ev()
  }

  #[inline(always)]
  pub fn int_er(&self) -> T::IntEr {
    self.0.int_er()
  }

  #[inline(always)]
  pub fn cr1(&self) -> &T::Cr1 {
    self.0.cr1()
  }

  #[inline(always)]
  pub fn cr1_pe(&self) -> &T::Cr1Pe {
    self.0.cr1_pe()
  }

  #[inline(always)]
  pub fn cr1_txie(&self) -> &T::Cr1Txie {
    self.0.cr1_txie()
  }

  #[inline(always)]
  pub fn cr1_rxie(&self) -> &T::Cr1Rxie {
    self.0.cr1_rxie()
  }

  #[inline(always)]
  pub fn cr1_addrie(&self) -> &T::Cr1Addrie {
    self.0.cr1_addrie()
  }

  #[inline(always)]
  pub fn cr1_nackie(&self) -> &T::Cr1Nackie {
    self.0.cr1_nackie()
  }

  #[inline(always)]
  pub fn cr1_stopie(&self) -> &T::Cr1Stopie {
    self.0.cr1_stopie()
  }

  #[inline(always)]
  pub fn cr1_tcie(&self) -> &T::Cr1Tcie {
    self.0.cr1_tcie()
  }

  #[inline(always)]
  pub fn cr1_errie(&self) -> &T::Cr1Errie {
    self.0.cr1_errie()
  }

  #[inline(always)]
  pub fn cr1_dnf(&self) -> &T::Cr1Dnf {
    self.0.cr1_dnf()
  }

  #[inline(always)]
  pub fn cr1_anfoff(&self) -> &T::Cr1Anfoff {
    self.0.cr1_anfoff()
  }

  #[inline(always)]
  pub fn cr1_txdmaen(&self) -> &T::Cr1Txdmaen {
    self.0.cr1_txdmaen()
  }

  #[inline(always)]
  pub fn cr1_rxdmaen(&self) -> &T::Cr1Rxdmaen {
    self.0.cr1_rxdmaen()
  }

  #[inline(always)]
  pub fn cr1_sbc(&self) -> &T::Cr1Sbc {
    self.0.cr1_sbc()
  }

  #[inline(always)]
  pub fn cr1_nostretch(&self) -> &T::Cr1Nostretch {
    self.0.cr1_nostretch()
  }

  #[inline(always)]
  pub fn cr1_wupen(&self) -> &T::Cr1Wupen {
    self.0.cr1_wupen()
  }

  #[inline(always)]
  pub fn cr1_gcen(&self) -> &T::Cr1Gcen {
    self.0.cr1_gcen()
  }

  #[inline(always)]
  pub fn cr1_smbhen(&self) -> &T::Cr1Smbhen {
    self.0.cr1_smbhen()
  }

  #[inline(always)]
  pub fn cr1_smbden(&self) -> &T::Cr1Smbden {
    self.0.cr1_smbden()
  }

  #[inline(always)]
  pub fn cr1_alerten(&self) -> &T::Cr1Alerten {
    self.0.cr1_alerten()
  }

  #[inline(always)]
  pub fn cr1_pecen(&self) -> &T::Cr1Pecen {
    self.0.cr1_pecen()
  }

  #[inline(always)]
  pub fn cr2(&self) -> &T::Cr2 {
    self.0.cr2()
  }

  #[inline(always)]
  pub fn cr2_pecbyte(&self) -> &T::Cr2Pecbyte {
    self.0.cr2_pecbyte()
  }

  #[inline(always)]
  pub fn cr2_autoend(&self) -> &T::Cr2Autoend {
    self.0.cr2_autoend()
  }

  #[inline(always)]
  pub fn cr2_reload(&self) -> &T::Cr2Reload {
    self.0.cr2_reload()
  }

  #[inline(always)]
  pub fn cr2_nbytes(&self) -> &T::Cr2Nbytes {
    self.0.cr2_nbytes()
  }

  #[inline(always)]
  pub fn cr2_nack(&self) -> &T::Cr2Nack {
    self.0.cr2_nack()
  }

  #[inline(always)]
  pub fn cr2_stop(&self) -> &T::Cr2Stop {
    self.0.cr2_stop()
  }

  #[inline(always)]
  pub fn cr2_start(&self) -> &T::Cr2Start {
    self.0.cr2_start()
  }

  #[inline(always)]
  pub fn cr2_head10r(&self) -> &T::Cr2Head10R {
    self.0.cr2_head10r()
  }

  #[inline(always)]
  pub fn cr2_add10(&self) -> &T::Cr2Add10 {
    self.0.cr2_add10()
  }

  #[inline(always)]
  pub fn cr2_rd_wrn(&self) -> &T::Cr2RdWrn {
    self.0.cr2_rd_wrn()
  }

  #[inline(always)]
  pub fn cr2_sadd(&self) -> &T::Cr2Sadd {
    self.0.cr2_sadd()
  }

  #[inline(always)]
  pub fn oar1(&self) -> &T::Oar1 {
    self.0.oar1()
  }

  #[inline(always)]
  pub fn oar2(&self) -> &T::Oar2 {
    self.0.oar2()
  }

  #[inline(always)]
  pub fn timingr(&self) -> &T::Timingr {
    self.0.timingr()
  }

  #[inline(always)]
  pub fn timeoutr(&self) -> &T::Timeoutr {
    self.0.timeoutr()
  }

  #[inline(always)]
  pub fn isr(&self) -> &<T::Isr as Reg<Crt>>::SReg {
    self.0.isr().as_sync()
  }

  #[inline(always)]
  pub fn icr(&self) -> &<T::Icr as Reg<Crt>>::SReg {
    self.0.icr().as_sync()
  }

  #[inline(always)]
  pub fn pecr(&self) -> &T::Pecr {
    self.0.pecr()
  }

  #[inline(always)]
  pub fn rxdr(&self) -> &T::Rxdr {
    self.0.rxdr()
  }

  #[inline(always)]
  pub fn rxdr_rxdata(&self) -> &T::RxdrRxdata {
    self.0.rxdr_rxdata()
  }

  #[inline(always)]
  pub fn txdr(&self) -> &T::Txdr {
    self.0.txdr()
  }

  #[inline(always)]
  pub fn txdr_txdata(&self) -> &T::TxdrTxdata {
    self.0.txdr_txdata()
  }
}

impl<T, C> I2C<T, C>
where
  T: I2CRes,
  C: RegGuardCnt<I2COn<T>>,
{
  /// Enables the clock.
  pub fn on(&self) -> RegGuard<I2COn<T>, C> {
    RegGuard::new(I2COn(*self.0.rcc_en()))
  }

  /// Initializes DMA for the SPI as peripheral.
  pub fn dma_rx_init<Rx>(&self, rx: &Rx)
  where
    Rx: DmaBond,
    T: I2CDmaRxRes<Rx>,
    C: DmaBondOnRgc<Rx::DmaRes>,
  {
    self.set_dma_rx_paddr(rx);
    self.dmamux_rx_init(rx);
    #[cfg(any(
      feature = "stm32l4x1",
      feature = "stm32l4x2",
      feature = "stm32l4x3",
      feature = "stm32l4x5",
      feature = "stm32l4x6"
    ))]
    rx.dma_ch().cselr_cs().modify(|r| {
      self.0.dma_rx_ch_init(r, rx.dma_ch());
    });
  }

  /// Initializes DMA for the SPI as peripheral.
  pub fn dma_tx_init<Tx>(&self, tx: &Tx)
  where
    Tx: DmaBond,
    T: I2CDmaTxRes<Tx>,
    C: DmaBondOnRgc<Tx::DmaRes>,
  {
    self.set_dma_tx_paddr(tx);
    self.dmamux_tx_init(tx);
    #[cfg(any(
      feature = "stm32l4x1",
      feature = "stm32l4x2",
      feature = "stm32l4x3",
      feature = "stm32l4x5",
      feature = "stm32l4x6"
    ))]
    tx.dma_ch().cselr_cs().modify(|r| {
      self.0.dma_tx_ch_init(r, tx.dma_ch());
    });
  }

  /// Initializes DMA for the SPI as peripheral.
  pub fn dma_init<Rx, Tx>(&self, rx: &Rx, tx: &Tx)
  where
    Rx: DmaBond,
    Tx: DmaBond,
    Tx::DmaRes: DmaTxRes<Rx::DmaRes>,
    T: I2CDmaRxRes<Rx> + I2CDmaTxRes<Tx>,
    C: DmaBondOnRgc<Rx::DmaRes> + DmaBondOnRgc<Tx::DmaRes>,
  {
    self.set_dma_rx_paddr(rx);
    self.set_dma_tx_paddr(tx);
    self.dmamux_rx_init(rx);
    self.dmamux_tx_init(tx);
    #[cfg(any(
      feature = "stm32l4x1",
      feature = "stm32l4x2",
      feature = "stm32l4x3",
      feature = "stm32l4x5",
      feature = "stm32l4x6"
    ))]
    rx.dma_ch().cselr_cs().modify(|r| {
      self.0.dma_rx_ch_init(r, rx.dma_ch());
      self.0.dma_tx_ch_init(r, tx.dma_ch());
    });
  }

  /// Returns a future, which resolves on I2C error event.
  pub fn transfer_error(&self) -> impl Future<Item = !, Error = I2CError> {
    let berr = *self.0.isr_berr();
    let ovr = *self.0.isr_ovr();
    let arlo = *self.0.isr_arlo();
    let timeout = *self.0.isr_timeout();
    let alert = *self.0.isr_alert();
    let pecerr = *self.0.isr_pecerr();
    let berrcf = *self.0.icr_berrcf();
    let ovrcf = *self.0.icr_ovrcf();
    let arlocf = *self.0.icr_arlocf();
    let timoutcf = *self.0.icr_timoutcf();
    let alertcf = *self.0.icr_alertcf();
    let peccf = *self.0.icr_peccf();
    fib::add_future(
      self.0.int_er(),
      fib::new(move || loop {
        if berr.read_bit_band() {
          berrcf.set_bit_band();
          break Err(I2CError::Berr);
        }
        if ovr.read_bit_band() {
          ovrcf.set_bit_band();
          break Err(I2CError::Ovr);
        }
        if arlo.read_bit_band() {
          arlocf.set_bit_band();
          break Err(I2CError::Arlo);
        }
        if timeout.read_bit_band() {
          timoutcf.set_bit_band();
          break Err(I2CError::Timeout);
        }
        if alert.read_bit_band() {
          alertcf.set_bit_band();
          break Err(I2CError::Alert);
        }
        if pecerr.read_bit_band() {
          peccf.set_bit_band();
          break Err(I2CError::Pecerr);
        }
        yield;
      }),
    )
  }

  /// Returns a future, which resolves on I2C transfer failure event.
  pub fn transfer_break(&self) -> impl Future<Item = !, Error = I2CBreak> {
    let nackf = *self.0.isr_nackf();
    let stopf = *self.0.isr_stopf();
    let nackcf = *self.0.icr_nackcf();
    let stopcf = *self.0.icr_stopcf();
    fib::add_future(
      self.0.int_ev(),
      fib::new(move || loop {
        if nackf.read_bit_band() {
          nackcf.set_bit_band();
          break Err(I2CBreak::Nack);
        }
        if stopf.read_bit_band() {
          stopcf.set_bit_band();
          break Err(I2CBreak::Stop);
        }
        yield;
      }),
    )
  }

  #[inline]
  fn set_dma_rx_paddr<Rx: DmaBond>(&self, rx: &Rx) {
    unsafe { rx.dma_ch().set_paddr(self.0.rxdr().to_ptr() as usize) };
  }

  #[inline]
  fn set_dma_tx_paddr<Tx: DmaBond>(&self, tx: &Tx) {
    unsafe { tx.dma_ch().set_paddr(self.0.txdr().to_mut_ptr() as usize) };
  }

  #[allow(unused_variables)]
  #[inline]
  fn dmamux_rx_init<Rx>(&self, rx: &Rx)
  where
    Rx: DmaBond,
    T: I2CDmaRxRes<Rx>,
    C: DmaBondOnRgc<Rx::DmaRes>,
  {
    #[cfg(any(
      feature = "stm32l4r5",
      feature = "stm32l4r7",
      feature = "stm32l4r9",
      feature = "stm32l4s5",
      feature = "stm32l4s7",
      feature = "stm32l4s9"
    ))]
    rx.dmamux_ch().cr_dmareq_id().modify(|r| {
      self.0.dmamux_rx_init(r, rx.dmamux_ch());
    });
  }

  #[allow(unused_variables)]
  #[inline]
  fn dmamux_tx_init<Tx>(&self, tx: &Tx)
  where
    Tx: DmaBond,
    T: I2CDmaTxRes<Tx>,
    C: DmaBondOnRgc<Tx::DmaRes>,
  {
    #[cfg(any(
      feature = "stm32l4r5",
      feature = "stm32l4r7",
      feature = "stm32l4r9",
      feature = "stm32l4s5",
      feature = "stm32l4s7",
      feature = "stm32l4s9"
    ))]
    tx.dmamux_ch().cr_dmareq_id().modify(|r| {
      self.0.dmamux_tx_init(r, tx.dmamux_ch());
    });
  }
}

impl<T: I2CRes> Clone for I2COn<T> {
  #[inline(always)]
  fn clone(&self) -> Self {
    Self(self.0)
  }
}

impl<T: I2CRes> RegGuardRes for I2COn<T> {
  type Reg = T::RccApbEnr;
  type Field = T::RccApbEnrI2CEn;

  #[inline(always)]
  fn field(&self) -> &Self::Field {
    &self.0
  }

  #[inline(always)]
  fn up(&self, val: &mut <Self::Reg as Reg<Crt>>::Val) {
    self.0.set(val)
  }

  #[inline(always)]
  fn down(&self, val: &mut <Self::Reg as Reg<Crt>>::Val) {
    self.0.clear(val)
  }
}

#[allow(unused_macros)]
macro_rules! i2c {
  (
    $doc:expr,
    $name:ident,
    $name_macro:ident,
    $doc_res:expr,
    $name_res:ident,
    $doc_on:expr,
    $name_on:ident,
    $int_ev_ty:ident,
    $int_er_ty:ident,
    $i2cen_ty:ident,
    $i2c:ident,
    $i2c_ev:ident,
    $i2c_er:ident,
    $i2c_cr1:ident,
    $i2c_cr2:ident,
    $i2c_oar1:ident,
    $i2c_oar2:ident,
    $i2c_timingr:ident,
    $i2c_timeoutr:ident,
    $i2c_isr:ident,
    $i2c_icr:ident,
    $i2c_pecr:ident,
    $i2c_rxdr:ident,
    $i2c_txdr:ident,
    $apb_enr:ident,
    $rcc_apb_enr_i2cen:ident,
    $rcc_apb_enr:ident,
    $i2cen:ident,
    (
      $dma_rx_req_id:expr,
      $((
        $dma_rx_bond:ident,
        $dma_rx_res:ident,
        $int_dma_rx:ident,
        $dma_rx_cs:expr
      )),*
    ),
    (
      $dma_tx_req_id:expr,
      $((
        $dma_tx_bond:ident,
        $dma_tx_res:ident,
        $int_dma_tx:ident,
        $dma_tx_cs:expr
      )),*
    ),
  ) => {
    #[doc = $doc]
    pub type $name<Ev, Er, C> = I2C<$name_res<Ev, Er, Crt>, C>;

    #[doc = $doc_res]
    #[allow(missing_docs)]
    pub struct $name_res<Ev, Er, Rt>
    where
      Ev: $int_ev_ty<Ttt>,
      Er: $int_er_ty<Ttt>,
      Rt: RegTag,
    {
      pub $i2c_ev: Ev,
      pub $i2c_er: Er,
      pub $i2c_cr1: $i2c::Cr1<Srt>,
      pub $i2c_cr2: $i2c::Cr2<Srt>,
      pub $i2c_oar1: $i2c::Oar1<Srt>,
      pub $i2c_oar2: $i2c::Oar2<Srt>,
      pub $i2c_timingr: $i2c::Timingr<Srt>,
      pub $i2c_timeoutr: $i2c::Timeoutr<Srt>,
      pub $i2c_isr: $i2c::Isr<Rt>,
      pub $i2c_icr: $i2c::Icr<Rt>,
      pub $i2c_pecr: $i2c::Pecr<Srt>,
      pub $i2c_rxdr: $i2c::Rxdr<Srt>,
      pub $i2c_txdr: $i2c::Txdr<Srt>,
      pub $rcc_apb_enr_i2cen: rcc::$apb_enr::$i2cen_ty<Rt>,
    }

    #[doc = $doc_on]
    pub type $name_on<Ev, Er> = I2COn<$name_res<Ev, Er, Crt>>;

    /// Creates a new `I2C`.
    #[macro_export]
    macro_rules! $name_macro {
      ($reg: ident, $thr: ident, $rgc:path) => {
        <$crate::i2c::I2C<_, $rgc> as ::drone_core::drv::Driver>::new(
          $crate::i2c::$name_res {
            $i2c_ev: $thr.$i2c_ev.to_trigger(),
            $i2c_er: $thr.$i2c_er.to_trigger(),
            $i2c_cr1: $reg.$i2c_cr1,
            $i2c_cr2: $reg.$i2c_cr2,
            $i2c_oar1: $reg.$i2c_oar1,
            $i2c_oar2: $reg.$i2c_oar2,
            $i2c_timingr: $reg.$i2c_timingr,
            $i2c_timeoutr: $reg.$i2c_timeoutr,
            $i2c_isr: $reg.$i2c_isr,
            $i2c_icr: $reg.$i2c_icr,
            $i2c_pecr: $reg.$i2c_pecr,
            $i2c_rxdr: $reg.$i2c_rxdr,
            $i2c_txdr: $reg.$i2c_txdr,
            $rcc_apb_enr_i2cen: $reg.$rcc_apb_enr.$i2cen,
          },
        )
      };
    }

    impl<Ev, Er> Resource for $name_res<Ev, Er, Crt>
    where
      Ev: $int_ev_ty<Ttt>,
      Er: $int_er_ty<Ttt>,
    {
      type Source = $name_res<Ev, Er, Srt>;

      #[inline(always)]
      fn from_source(source: Self::Source) -> Self {
        Self {
          $i2c_ev: source.$i2c_ev,
          $i2c_er: source.$i2c_er,
          $i2c_cr1: source.$i2c_cr1,
          $i2c_cr2: source.$i2c_cr2,
          $i2c_oar1: source.$i2c_oar1,
          $i2c_oar2: source.$i2c_oar2,
          $i2c_timingr: source.$i2c_timingr,
          $i2c_timeoutr: source.$i2c_timeoutr,
          $i2c_isr: source.$i2c_isr.to_copy(),
          $i2c_icr: source.$i2c_icr.to_copy(),
          $i2c_pecr: source.$i2c_pecr,
          $i2c_rxdr: source.$i2c_rxdr,
          $i2c_txdr: source.$i2c_txdr,
          $rcc_apb_enr_i2cen: source.$rcc_apb_enr_i2cen.to_copy(),
        }
      }
    }

    impl<Ev, Er> I2CRes for $name_res<Ev, Er, Crt>
    where
      Ev: $int_ev_ty<Ttt>,
      Er: $int_er_ty<Ttt>,
    {
      type IntEv = Ev;
      type IntEr = Er;
      type Oar1 = $i2c::Oar1<Srt>;
      type Oar2 = $i2c::Oar2<Srt>;
      type Timingr = $i2c::Timingr<Srt>;
      type Timeoutr = $i2c::Timeoutr<Srt>;
      type Pecr = $i2c::Pecr<Srt>;
      type RxdrVal = $i2c::rxdr::Val;
      type Rxdr = $i2c::Rxdr<Srt>;
      type RxdrRxdata = $i2c::rxdr::Rxdata<Srt>;
      type TxdrVal = $i2c::txdr::Val;
      type Txdr = $i2c::Txdr<Srt>;
      type TxdrTxdata = $i2c::txdr::Txdata<Srt>;
      type RccApbEnrVal = rcc::$apb_enr::Val;
      type RccApbEnr = rcc::$apb_enr::Reg<Crt>;
      type RccApbEnrI2CEn = rcc::$apb_enr::$i2cen_ty<Crt>;

      #[inline(always)]
      fn int_ev(&self) -> Self::IntEv {
        self.$i2c_ev
      }

      #[inline(always)]
      fn int_er(&self) -> Self::IntEr {
        self.$i2c_er
      }

      res_impl!(Oar1, oar1, $i2c_oar1);
      res_impl!(Oar2, oar2, $i2c_oar2);
      res_impl!(Timingr, timingr, $i2c_timingr);
      res_impl!(Timeoutr, timeoutr, $i2c_timeoutr);
      res_impl!(Pecr, pecr, $i2c_pecr);
      res_impl!(Rxdr, rxdr, $i2c_rxdr);
      res_impl!(RxdrRxdata, rxdr_rxdata, $i2c_rxdr.rxdata);
      res_impl!(Txdr, txdr, $i2c_txdr);
      res_impl!(TxdrTxdata, txdr_txdata, $i2c_txdr.txdata);
      res_impl!(RccApbEnrI2CEn, rcc_en, $rcc_apb_enr_i2cen);
    }

    impl<Ev, Er> I2CResCr1 for $name_res<Ev, Er, Crt>
    where
      Ev: $int_ev_ty<Ttt>,
      Er: $int_er_ty<Ttt>,
    {
      type Cr1Val = $i2c::cr1::Val;
      type Cr1 = $i2c::Cr1<Srt>;
      type Cr1Pe = $i2c::cr1::Pe<Srt>;
      type Cr1Txie = $i2c::cr1::Txie<Srt>;
      type Cr1Rxie = $i2c::cr1::Rxie<Srt>;
      type Cr1Addrie = $i2c::cr1::Addrie<Srt>;
      type Cr1Nackie = $i2c::cr1::Nackie<Srt>;
      type Cr1Stopie = $i2c::cr1::Stopie<Srt>;
      type Cr1Tcie = $i2c::cr1::Tcie<Srt>;
      type Cr1Errie = $i2c::cr1::Errie<Srt>;
      type Cr1Dnf = $i2c::cr1::Dnf<Srt>;
      type Cr1Anfoff = $i2c::cr1::Anfoff<Srt>;
      type Cr1Txdmaen = $i2c::cr1::Txdmaen<Srt>;
      type Cr1Rxdmaen = $i2c::cr1::Rxdmaen<Srt>;
      type Cr1Sbc = $i2c::cr1::Sbc<Srt>;
      type Cr1Nostretch = $i2c::cr1::Nostretch<Srt>;
      type Cr1Wupen = $i2c::cr1::Wupen<Srt>;
      type Cr1Gcen = $i2c::cr1::Gcen<Srt>;
      type Cr1Smbhen = $i2c::cr1::Smbhen<Srt>;
      type Cr1Smbden = $i2c::cr1::Smbden<Srt>;
      type Cr1Alerten = $i2c::cr1::Alerten<Srt>;
      type Cr1Pecen = $i2c::cr1::Pecen<Srt>;

      res_impl!(Cr1, cr1, $i2c_cr1);
      res_impl!(Cr1Pe, cr1_pe, $i2c_cr1.pe);
      res_impl!(Cr1Txie, cr1_txie, $i2c_cr1.txie);
      res_impl!(Cr1Rxie, cr1_rxie, $i2c_cr1.rxie);
      res_impl!(Cr1Addrie, cr1_addrie, $i2c_cr1.addrie);
      res_impl!(Cr1Nackie, cr1_nackie, $i2c_cr1.nackie);
      res_impl!(Cr1Stopie, cr1_stopie, $i2c_cr1.stopie);
      res_impl!(Cr1Tcie, cr1_tcie, $i2c_cr1.tcie);
      res_impl!(Cr1Errie, cr1_errie, $i2c_cr1.errie);
      res_impl!(Cr1Dnf, cr1_dnf, $i2c_cr1.dnf);
      res_impl!(Cr1Anfoff, cr1_anfoff, $i2c_cr1.anfoff);
      res_impl!(Cr1Txdmaen, cr1_txdmaen, $i2c_cr1.txdmaen);
      res_impl!(Cr1Rxdmaen, cr1_rxdmaen, $i2c_cr1.rxdmaen);
      res_impl!(Cr1Sbc, cr1_sbc, $i2c_cr1.sbc);
      res_impl!(Cr1Nostretch, cr1_nostretch, $i2c_cr1.nostretch);
      res_impl!(Cr1Wupen, cr1_wupen, $i2c_cr1.wupen);
      res_impl!(Cr1Gcen, cr1_gcen, $i2c_cr1.gcen);
      res_impl!(Cr1Smbhen, cr1_smbhen, $i2c_cr1.smbhen);
      res_impl!(Cr1Smbden, cr1_smbden, $i2c_cr1.smbden);
      res_impl!(Cr1Alerten, cr1_alerten, $i2c_cr1.alerten);
      res_impl!(Cr1Pecen, cr1_pecen, $i2c_cr1.pecen);
    }

    impl<Ev, Er> I2CResCr2 for $name_res<Ev, Er, Crt>
    where
      Ev: $int_ev_ty<Ttt>,
      Er: $int_er_ty<Ttt>,
    {
      type Cr2Val = $i2c::cr2::Val;
      type Cr2 = $i2c::Cr2<Srt>;
      type Cr2Pecbyte = $i2c::cr2::Pecbyte<Srt>;
      type Cr2Autoend = $i2c::cr2::Autoend<Srt>;
      type Cr2Reload = $i2c::cr2::Reload<Srt>;
      type Cr2Nbytes = $i2c::cr2::Nbytes<Srt>;
      type Cr2Nack = $i2c::cr2::Nack<Srt>;
      type Cr2Stop = $i2c::cr2::Stop<Srt>;
      type Cr2Start = $i2c::cr2::Start<Srt>;
      type Cr2Head10R = $i2c::cr2::Head10R<Srt>;
      type Cr2Add10 = $i2c::cr2::Add10<Srt>;
      type Cr2RdWrn = $i2c::cr2::RdWrn<Srt>;
      type Cr2Sadd = $i2c::cr2::Sadd<Srt>;

      res_impl!(Cr2, cr2, $i2c_cr2);
      res_impl!(Cr2Pecbyte, cr2_pecbyte, $i2c_cr2.pecbyte);
      res_impl!(Cr2Autoend, cr2_autoend, $i2c_cr2.autoend);
      res_impl!(Cr2Reload, cr2_reload, $i2c_cr2.reload);
      res_impl!(Cr2Nbytes, cr2_nbytes, $i2c_cr2.nbytes);
      res_impl!(Cr2Nack, cr2_nack, $i2c_cr2.nack);
      res_impl!(Cr2Stop, cr2_stop, $i2c_cr2.stop);
      res_impl!(Cr2Start, cr2_start, $i2c_cr2.start);
      res_impl!(Cr2Head10R, cr2_head10r, $i2c_cr2.head10r);
      res_impl!(Cr2Add10, cr2_add10, $i2c_cr2.add10);
      res_impl!(Cr2RdWrn, cr2_rd_wrn, $i2c_cr2.rd_wrn);
      res_impl!(Cr2Sadd, cr2_sadd, $i2c_cr2.sadd);
    }

    impl<Ev, Er> I2CResIsr for $name_res<Ev, Er, Crt>
    where
      Ev: $int_ev_ty<Ttt>,
      Er: $int_er_ty<Ttt>,
    {
      type Isr = $i2c::Isr<Crt>;
      type IsrNackf = $i2c::isr::Nackf<Crt>;
      type IsrStopf = $i2c::isr::Stopf<Crt>;
      type IsrTc = $i2c::isr::Tc<Crt>;
      type IsrTcr = $i2c::isr::Tcr<Crt>;
      type IsrBerr = $i2c::isr::Berr<Crt>;
      type IsrArlo = $i2c::isr::Arlo<Crt>;
      type IsrOvr = $i2c::isr::Ovr<Crt>;
      type IsrPecerr = $i2c::isr::Pecerr<Crt>;
      type IsrTimeout = $i2c::isr::Timeout<Crt>;
      type IsrAlert = $i2c::isr::Alert<Crt>;

      res_impl!(Isr, isr, $i2c_isr);
      res_impl!(IsrNackf, isr_nackf, $i2c_isr.nackf);
      res_impl!(IsrStopf, isr_stopf, $i2c_isr.stopf);
      res_impl!(IsrTc, isr_tc, $i2c_isr.tc);
      res_impl!(IsrTcr, isr_tcr, $i2c_isr.tcr);
      res_impl!(IsrBerr, isr_berr, $i2c_isr.berr);
      res_impl!(IsrArlo, isr_arlo, $i2c_isr.arlo);
      res_impl!(IsrOvr, isr_ovr, $i2c_isr.ovr);
      res_impl!(IsrPecerr, isr_pecerr, $i2c_isr.pecerr);
      res_impl!(IsrTimeout, isr_timeout, $i2c_isr.timeout);
      res_impl!(IsrAlert, isr_alert, $i2c_isr.alert);
    }

    impl<Ev, Er> I2CResIcr for $name_res<Ev, Er, Crt>
    where
      Ev: $int_ev_ty<Ttt>,
      Er: $int_er_ty<Ttt>,
    {
      type Icr = $i2c::Icr<Crt>;
      type IcrNackcf = $i2c::icr::Nackcf<Crt>;
      type IcrStopcf = $i2c::icr::Stopcf<Crt>;
      type IcrBerrcf = $i2c::icr::Berrcf<Crt>;
      type IcrArlocf = $i2c::icr::Arlocf<Crt>;
      type IcrOvrcf = $i2c::icr::Ovrcf<Crt>;
      type IcrPeccf = $i2c::icr::Peccf<Crt>;
      type IcrTimoutcf = $i2c::icr::Timoutcf<Crt>;
      type IcrAlertcf = $i2c::icr::Alertcf<Crt>;

      res_impl!(Icr, icr, $i2c_icr);
      res_impl!(IcrNackcf, icr_nackcf, $i2c_icr.nackcf);
      res_impl!(IcrStopcf, icr_stopcf, $i2c_icr.stopcf);
      res_impl!(IcrBerrcf, icr_berrcf, $i2c_icr.berrcf);
      res_impl!(IcrArlocf, icr_arlocf, $i2c_icr.arlocf);
      res_impl!(IcrOvrcf, icr_ovrcf, $i2c_icr.ovrcf);
      res_impl!(IcrPeccf, icr_peccf, $i2c_icr.peccf);
      res_impl!(IcrTimoutcf, icr_timoutcf, $i2c_icr.timoutcf);
      res_impl!(IcrAlertcf, icr_alertcf, $i2c_icr.alertcf);
    }

    #[cfg(any(
      feature = "stm32l4r5",
      feature = "stm32l4r7",
      feature = "stm32l4r9",
      feature = "stm32l4s5",
      feature = "stm32l4s7",
      feature = "stm32l4s9"
    ))]
    impl<Ev, Er, T> I2CDmaRxRes<T> for $name_res<Ev, Er, Crt>
    where
      T: DmaBond,
      Ev: $int_ev_ty<Ttt>,
      Er: $int_er_ty<Ttt>,
    {
      #[inline(always)]
      fn dmamux_rx_init(
        &self,
        cr_val: &mut <<T::DmamuxChRes as DmamuxChRes>::Cr as Reg<Srt>>::Val,
        dmamux: &DmamuxCh<T::DmamuxChRes>,
      ) {
        dmamux.cr_dmareq_id().write(cr_val, $dma_rx_req_id);
      }
    }

    $(
      #[cfg(any(
        feature = "stm32l4x1",
        feature = "stm32l4x2",
        feature = "stm32l4x3",
        feature = "stm32l4x5",
        feature = "stm32l4x6"
      ))]
      impl<Ev, Er, Rx, C> I2CDmaRxRes<$dma_rx_bond<Rx, C>>
        for $name_res<Ev, Er, Crt>
      where
        Rx: $int_dma_rx<Ttt>,
        Ev: $int_ev_ty<Ttt>,
        Er: $int_er_ty<Ttt>,
        C: DmaBondOnRgc<$dma_rx_res<Rx, Crt>>,
      {
        #[inline(always)]
        fn dma_rx_ch_init(
          &self,
          cs_val: &mut <<<$dma_rx_bond<Rx, C> as DmaBond>::DmaRes
            as DmaRes>::Cselr as Reg<Srt>>::Val,
          dma: &Dma<<$dma_rx_bond<Rx, C> as DmaBond>::DmaRes>,
        ) {
          dma.cselr_cs().write(cs_val, $dma_rx_cs);
        }
      }
    )*

    #[cfg(any(
      feature = "stm32l4r5",
      feature = "stm32l4r7",
      feature = "stm32l4r9",
      feature = "stm32l4s5",
      feature = "stm32l4s7",
      feature = "stm32l4s9"
    ))]
    impl<Ev, Er, T> I2CDmaTxRes<T> for $name_res<Ev, Er, Crt>
    where
      T: DmaBond,
      Ev: $int_ev_ty<Ttt>,
      Er: $int_er_ty<Ttt>,
    {
      #[inline(always)]
      fn dmamux_tx_init(
        &self,
        cr_val: &mut <<T::DmamuxChRes as DmamuxChRes>::Cr as Reg<Srt>>::Val,
        dmamux: &DmamuxCh<T::DmamuxChRes>,
      ) {
        dmamux.cr_dmareq_id().write(cr_val, $dma_tx_req_id);
      }
    }

    $(
      #[cfg(any(
        feature = "stm32l4x1",
        feature = "stm32l4x2",
        feature = "stm32l4x3",
        feature = "stm32l4x5",
        feature = "stm32l4x6"
      ))]
      impl<Ev, Er, Tx, C> I2CDmaTxRes<$dma_tx_bond<Tx, C>>
        for $name_res<Ev, Er, Crt>
      where
        Tx: $int_dma_tx<Ttt>,
        Ev: $int_ev_ty<Ttt>,
        Er: $int_er_ty<Ttt>,
        C: DmaBondOnRgc<$dma_tx_res<Tx, Crt>>,
      {
        #[inline(always)]
        fn dma_tx_ch_init(
          &self,
          cs_val: &mut <<<$dma_tx_bond<Tx, C> as DmaBond>::DmaRes
            as DmaRes>::Cselr as Reg<Srt>>::Val,
          dma: &Dma<<$dma_tx_bond<Tx, C> as DmaBond>::DmaRes>,
        ) {
          dma.cselr_cs().write(cs_val, $dma_tx_cs);
        }
      }
    )*
  };
}

#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
i2c! {
  "I2C1 driver.",
  I2C1,
  drv_i2c1,
  "I2C1 resource.",
  I2C1Res,
  "I2C1 clock on guard resource.",
  I2C1On,
  IntI2C1Ev,
  IntI2C1Er,
  I2C1En,
  i2c1,
  i2c1_ev,
  i2c1_er,
  i2c1_cr1,
  i2c1_cr2,
  i2c1_oar1,
  i2c1_oar2,
  i2c1_timingr,
  i2c1_timeoutr,
  i2c1_isr,
  i2c1_icr,
  i2c1_pecr,
  i2c1_rxdr,
  i2c1_txdr,
  apb1enr1,
  rcc_apb1enr1_i2c1en,
  rcc_apb1enr1,
  i2c1en,
  (
    16,
    (Dma1Ch7Bond, Dma1Ch7Res, IntDma1Ch7, 3),
    (Dma2Ch6Bond, Dma2Ch6Res, IntDma2Ch6, 5)
  ),
  (
    17,
    (Dma1Ch6Bond, Dma1Ch6Res, IntDma1Ch6, 3),
    (Dma2Ch7Bond, Dma2Ch7Res, IntDma2Ch7, 5)
  ),
}

#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
i2c! {
  "I2C2 driver.",
  I2C2,
  drv_i2c2,
  "I2C2 resource.",
  I2C2Res,
  "I2C2 clock on guard resource.",
  I2C2On,
  IntI2C2Ev,
  IntI2C2Er,
  I2C2En,
  i2c2,
  i2c2_ev,
  i2c2_er,
  i2c2_cr1,
  i2c2_cr2,
  i2c2_oar1,
  i2c2_oar2,
  i2c2_timingr,
  i2c2_timeoutr,
  i2c2_isr,
  i2c2_icr,
  i2c2_pecr,
  i2c2_rxdr,
  i2c2_txdr,
  apb1enr1,
  rcc_apb1enr1_i2c2en,
  rcc_apb1enr1,
  i2c2en,
  (18, (Dma1Ch5Bond, Dma1Ch5Res, IntDma1Ch5, 3)),
  (19, (Dma1Ch4Bond, Dma1Ch4Res, IntDma1Ch4, 3)),
}

#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
i2c! {
  "I2C3 driver.",
  I2C3,
  drv_i2c3,
  "I2C3 resource.",
  I2C3Res,
  "I2C3 clock on guard resource.",
  I2C3On,
  IntI2C3Ev,
  IntI2C3Er,
  I2C3En,
  i2c3,
  i2c3_ev,
  i2c3_er,
  i2c3_cr1,
  i2c3_cr2,
  i2c3_oar1,
  i2c3_oar2,
  i2c3_timingr,
  i2c3_timeoutr,
  i2c3_isr,
  i2c3_icr,
  i2c3_pecr,
  i2c3_rxdr,
  i2c3_txdr,
  apb1enr1,
  rcc_apb1enr1_i2c3en,
  rcc_apb1enr1,
  i2c3en,
  (20, (Dma1Ch3Bond, Dma1Ch3Res, IntDma1Ch3, 3)),
  (21, (Dma1Ch2Bond, Dma1Ch2Res, IntDma1Ch2, 3)),
}

#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
i2c! {
  "I2C4 driver.",
  I2C4,
  drv_i2c4,
  "I2C4 resource.",
  I2C4Res,
  "I2C4 clock on guard resource.",
  I2C4On,
  IntI2C4Ev,
  IntI2C4Er,
  I2C4En,
  i2c4,
  i2c4_ev,
  i2c4_er,
  i2c4_cr1,
  i2c4_cr2,
  i2c4_oar1,
  i2c4_oar2,
  i2c4_timingr,
  i2c4_timeoutr,
  i2c4_isr,
  i2c4_icr,
  i2c4_pecr,
  i2c4_rxdr,
  i2c4_txdr,
  apb1enr2,
  rcc_apb1enr2_i2c4en,
  rcc_apb1enr2,
  i2c4en,
  (22, (Dma2Ch1Bond, Dma2Ch1Res, IntDma2Ch1, 0)),
  (23, (Dma2Ch2Bond, Dma2Ch2Res, IntDma2Ch2, 0)),
}
