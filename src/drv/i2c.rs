//! Inter-Integrated Circuit.

use drone_core::drv::Resource;
use drv::dma::{Dma, DmaRes};
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
use drv::dma::{Dma1Ch2Res, Dma1Ch3Res, Dma2Ch6Res, Dma2Ch7Res};
#[cfg(any(
  feature = "stm32f100",
  feature = "stm32f101",
  feature = "stm32f102",
  feature = "stm32f103",
  feature = "stm32f107",
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
use drv::dma::{Dma1Ch4Res, Dma1Ch5Res, Dma1Ch6Res, Dma1Ch7Res};
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6"
))]
use drv::dma::{Dma2Ch1Res, Dma2Ch2Res};
use fib;
use futures::prelude::*;
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
use reg::i2c3;
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6"
))]
use reg::i2c4;
use reg::marker::*;
use reg::prelude::*;
#[cfg(any(
  feature = "stm32f100",
  feature = "stm32f101",
  feature = "stm32f102",
  feature = "stm32f103",
  feature = "stm32f107",
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
use reg::{i2c1, i2c2};
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6"
))]
use thr::int::{
  IntDma1Ch2, IntDma1Ch3, IntDma1Ch4, IntDma1Ch5, IntDma1Ch6, IntDma1Ch7,
  IntDma2Ch1, IntDma2Ch2, IntDma2Ch6, IntDma2Ch7, IntI2C4Er, IntI2C4Ev,
};
#[cfg(any(feature = "stm32l4x3", feature = "stm32l4x5"))]
use thr::int::{
  IntDma1Channel2 as IntDma1Ch2, IntDma1Channel3 as IntDma1Ch3,
  IntDma2Channel6 as IntDma2Ch6, IntDma2Channel7 as IntDma2Ch7,
};
#[cfg(any(
  feature = "stm32f100",
  feature = "stm32f101",
  feature = "stm32f102",
  feature = "stm32f103",
  feature = "stm32f107",
  feature = "stm32l4x3",
  feature = "stm32l4x5"
))]
use thr::int::{
  IntDma1Channel4 as IntDma1Ch4, IntDma1Channel5 as IntDma1Ch5,
  IntDma1Channel6 as IntDma1Ch6, IntDma1Channel7 as IntDma1Ch7,
};
#[cfg(any(
  feature = "stm32f100",
  feature = "stm32f101",
  feature = "stm32f102",
  feature = "stm32f103",
  feature = "stm32f107",
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
use thr::int::{IntI2C1Er, IntI2C1Ev, IntI2C2Er, IntI2C2Ev};
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
use thr::int::{IntI2C3Er, IntI2C3Ev};
use thr::prelude::*;

/// Incomplete I2C transfer error.
#[derive(Debug, Fail)]
pub enum I2CTransferFailure {
  /// NACK reception.
  #[fail(display = "I2C NACK received.")]
  Nack,
  /// Stop reception.
  #[fail(display = "I2C STOP received.")]
  Stop,
}

/// I2C driver.
#[derive(Driver)]
pub struct I2C<T: I2CRes>(T);

/// DMA-driven I2C driver.
pub trait I2CDmaRx<T, Rx>
where
  T: I2CDmaRxRes<Rx>,
  Rx: DmaRes,
{
  /// Initializes DMA for the I2C as peripheral.
  fn dma_rx_init(&self, dma_rx: &Dma<Rx>);

  /// Initializes DMA for the I2C as peripheral.
  fn dma_rx_paddr_init(&self, dma_rx: &Dma<Rx>);
}

/// DMA-driven I2C driver.
pub trait I2CDmaTx<T, Tx>
where
  T: I2CDmaTxRes<Tx>,
  Tx: DmaRes,
{
  /// Initializes DMA for the I2C as peripheral.
  fn dma_tx_init(&self, dma_tx: &Dma<Tx>);

  /// Initializes DMA for the I2C as peripheral.
  fn dma_tx_paddr_init(&self, dma_tx: &Dma<Tx>);
}

/// DMA-driven I2C driver.
pub trait I2CDmaDx<T, Rx, Tx>
where
  T: I2CDmaRxRes<Rx> + I2CDmaTxRes<Tx>,
  Rx: DmaRes,
  Tx: DmaRes,
{
  #[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  ))]
  /// Initializes DMA for the I2C as peripheral.
  fn dma_dx_init(&self, dma_rx: &Dma<Rx>, dma_tx: &Dma<Tx>)
  where
    Tx: DmaRes<Cselr = Rx::Cselr>;

  #[cfg(not(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )))]
  /// Initializes DMA for the I2C as peripheral.
  fn dma_dx_init(&self, dma_rx: &Dma<Rx>, dma_tx: &Dma<Tx>);

  /// Initializes DMA for the I2C as peripheral.
  fn dma_dx_paddr_init(&self, dma_rx: &Dma<Rx>, dma_tx: &Dma<Tx>);
}

/// I2C resource.
#[allow(missing_docs)]
pub trait I2CRes: Resource {
  type Cr1: SRwRegBitBand;
  type Cr2: SRwRegBitBand;
  type Oar1: SRwRegBitBand;
  type Oar2: SRwRegBitBand;
  type Timingr: SRwRegBitBand;
  type Timeoutr: SRwRegBitBand;
  type Isr: FRwRegBitBand;
  type IsrNackf: FRoRwRegFieldBitBand<Reg = Self::Isr>;
  type IsrStopf: FRoRwRegFieldBitBand<Reg = Self::Isr>;
  type IsrTc: FRoRwRegFieldBitBand<Reg = Self::Isr>;
  type IsrTcr: FRoRwRegFieldBitBand<Reg = Self::Isr>;
  type Icr: FWoRegBitBand;
  type IcrNackcf: FWoWoRegFieldBitBand<Reg = Self::Icr>;
  type IcrStopcf: FWoWoRegFieldBitBand<Reg = Self::Icr>;
  type Pecr: SRoRegBitBand;
  type Rxdr: SRoRegBitBand;
  type Txdr: SRwRegBitBand;

  res_reg_decl!(Cr1, cr1, cr1_mut);
  res_reg_decl!(Cr2, cr2, cr2_mut);
  res_reg_decl!(Oar1, oar1, oar1_mut);
  res_reg_decl!(Oar2, oar2, oar2_mut);
  res_reg_decl!(Timingr, timingr, timingr_mut);
  res_reg_decl!(Timeoutr, timeoutr, timeoutr_mut);
  res_reg_decl!(Isr, isr, isr_mut);
  res_reg_decl!(IsrNackf, isr_nackf, isr_nackf_mut);
  res_reg_decl!(IsrStopf, isr_stopf, isr_stopf_mut);
  res_reg_decl!(IsrTc, isr_tc, isr_tc_mut);
  res_reg_decl!(IsrTcr, isr_tcr, isr_tcr_mut);
  res_reg_decl!(Icr, icr, icr_mut);
  res_reg_decl!(IcrNackcf, icr_nackcf, icr_nackcf_mut);
  res_reg_decl!(IcrStopcf, icr_stopcf, icr_stopcf_mut);
  res_reg_decl!(Pecr, pecr, pecr_mut);
  res_reg_decl!(Rxdr, rxdr, rxdr_mut);
  res_reg_decl!(Txdr, txdr, txdr_mut);
}

/// Interrupt-driven I2C resource.
#[allow(missing_docs)]
pub trait I2CIntRes: I2CRes {
  type WithoutInt: I2CRes;
  type IntEv: IntToken<Ltt>;
  type IntEr: IntToken<Ltt>;

  fn join_int(
    res: Self::WithoutInt,
    int_ev: Self::IntEv,
    int_er: Self::IntEr,
  ) -> Self;

  fn split_int(self) -> (Self::WithoutInt, Self::IntEv, Self::IntEr);

  fn int_ev(&self) -> Self::IntEv;

  fn int_er(&self) -> Self::IntEr;
}

/// DMA-driven I2C resource.
#[allow(missing_docs)]
pub trait I2CDmaRxRes<T: DmaRes>: I2CRes {
  #[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  ))]
  fn dma_rx_ch_init(&self, cs_val: &mut CselrVal<T>, dma: &Dma<T>);
}

/// DMA-driven I2C resource.
#[allow(missing_docs)]
pub trait I2CDmaTxRes<T: DmaRes>: I2CRes {
  #[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  ))]
  fn dma_tx_ch_init(&self, cs_val: &mut CselrVal<T>, dma: &Dma<T>);
}

#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
type CselrVal<T> = <<T as DmaRes>::Cselr as Reg<Srt>>::Val;

#[allow(missing_docs)]
impl<T: I2CRes> I2C<T> {
  #[inline(always)]
  pub fn cr1(&self) -> &T::Cr1 {
    self.0.cr1()
  }

  #[inline(always)]
  pub fn cr2(&self) -> &T::Cr2 {
    self.0.cr2()
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
  pub fn isr(&self) -> &T::Isr {
    self.0.isr()
  }

  #[inline(always)]
  pub fn icr(&self) -> &T::Icr {
    self.0.icr()
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
  pub fn txdr(&self) -> &T::Txdr {
    self.0.txdr()
  }
}

#[allow(missing_docs)]
impl<T: I2CIntRes> I2C<T> {
  #[inline(always)]
  pub fn join_int(
    res: I2C<T::WithoutInt>,
    int_ev: T::IntEv,
    int_er: T::IntEr,
  ) -> I2C<T> {
    I2C(T::join_int(res.0, int_ev, int_er))
  }

  #[inline(always)]
  pub fn split_int(self) -> (I2C<T::WithoutInt>, T::IntEv, T::IntEr) {
    let (res, int_ev, int_er) = self.0.split_int();
    (I2C(res), int_ev, int_er)
  }

  #[inline(always)]
  pub fn int_ev(&self) -> T::IntEv {
    self.0.int_ev()
  }

  #[inline(always)]
  pub fn int_er(&self) -> T::IntEr {
    self.0.int_er()
  }

  /// Returns a future, which resolves on I2C transfer complete event.
  pub fn transfer_complete(
    &mut self,
  ) -> impl Future<Item = (), Error = I2CTransferFailure> {
    let tc = self.0.isr_tc_mut().fork();
    let nackf = self.0.isr_nackf_mut().fork();
    let stopf = self.0.isr_stopf_mut().fork();
    let nackcf = self.0.icr_nackcf_mut().fork();
    let stopcf = self.0.icr_stopcf_mut().fork();
    fib::add_future(
      self.0.int_ev(),
      fib::new(move || loop {
        if nackf.read_bit_band() {
          nackcf.set_bit_band();
          break Err(I2CTransferFailure::Nack);
        }
        if stopf.read_bit_band() {
          stopcf.set_bit_band();
          break Err(I2CTransferFailure::Stop);
        }
        if tc.read_bit_band() {
          break Ok(());
        }
        yield;
      }),
    )
  }

  /// Returns a future, which resolves on I2C transfer complete reload event.
  pub fn transfer_reload(
    &mut self,
  ) -> impl Future<Item = (), Error = I2CTransferFailure> {
    let tcr = self.0.isr_tcr_mut().fork();
    let nackf = self.0.isr_nackf_mut().fork();
    let stopf = self.0.isr_stopf_mut().fork();
    let nackcf = self.0.icr_nackcf_mut().fork();
    let stopcf = self.0.icr_stopcf_mut().fork();
    fib::add_future(
      self.0.int_ev(),
      fib::new(move || loop {
        if nackf.read_bit_band() {
          nackcf.set_bit_band();
          break Err(I2CTransferFailure::Nack);
        }
        if stopf.read_bit_band() {
          stopcf.set_bit_band();
          break Err(I2CTransferFailure::Stop);
        }
        if tcr.read_bit_band() {
          break Ok(());
        }
        yield;
      }),
    )
  }
}

#[allow(missing_docs)]
impl<T, Rx> I2CDmaRx<T, Rx> for I2C<T>
where
  T: I2CDmaRxRes<Rx>,
  Rx: DmaRes,
{
  #[inline(always)]
  fn dma_rx_init(&self, dma_rx: &Dma<Rx>) {
    self.dma_rx_paddr_init(dma_rx);
    #[cfg(any(
      feature = "stm32l4x1",
      feature = "stm32l4x2",
      feature = "stm32l4x3",
      feature = "stm32l4x5",
      feature = "stm32l4x6"
    ))]
    dma_rx.cselr_cs().modify(|r| {
      self.0.dma_rx_ch_init(r, dma_rx);
    });
  }

  #[inline(always)]
  fn dma_rx_paddr_init(&self, dma_rx: &Dma<Rx>) {
    unsafe { dma_rx.set_paddr(self.0.rxdr().to_ptr() as usize) };
  }
}

#[allow(missing_docs)]
impl<T, Tx> I2CDmaTx<T, Tx> for I2C<T>
where
  T: I2CDmaTxRes<Tx>,
  Tx: DmaRes,
{
  #[inline(always)]
  fn dma_tx_init(&self, dma_tx: &Dma<Tx>) {
    self.dma_tx_paddr_init(dma_tx);
    #[cfg(any(
      feature = "stm32l4x1",
      feature = "stm32l4x2",
      feature = "stm32l4x3",
      feature = "stm32l4x5",
      feature = "stm32l4x6"
    ))]
    dma_tx.cselr_cs().modify(|r| {
      self.0.dma_tx_ch_init(r, dma_tx);
    });
  }

  #[inline(always)]
  fn dma_tx_paddr_init(&self, dma_tx: &Dma<Tx>) {
    unsafe { dma_tx.set_paddr(self.0.txdr().to_mut_ptr() as usize) };
  }
}

#[allow(missing_docs)]
impl<T, Rx, Tx> I2CDmaDx<T, Rx, Tx> for I2C<T>
where
  T: I2CDmaRxRes<Rx> + I2CDmaTxRes<Tx>,
  Rx: DmaRes,
  Tx: DmaRes,
{
  #[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  ))]
  #[inline(always)]
  fn dma_dx_init(&self, dma_rx: &Dma<Rx>, dma_tx: &Dma<Tx>)
  where
    Tx: DmaRes<Cselr = Rx::Cselr>,
  {
    self.dma_dx_paddr_init(dma_rx, dma_tx);
    dma_rx.cselr_cs().modify(|r| {
      self.0.dma_rx_ch_init(r, dma_rx);
      self.0.dma_tx_ch_init(r, dma_tx);
    });
  }

  #[cfg(not(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )))]
  #[inline(always)]
  fn dma_dx_init(&self, dma_rx: &Dma<Rx>, dma_tx: &Dma<Tx>) {
    self.dma_dx_paddr_init(dma_rx, dma_tx);
  }

  #[inline(always)]
  fn dma_dx_paddr_init(&self, dma_rx: &Dma<Rx>, dma_tx: &Dma<Tx>) {
    self.dma_rx_paddr_init(dma_rx);
    self.dma_tx_paddr_init(dma_tx);
  }
}

#[allow(unused_macros)]
macro_rules! i2c_shared {
  (
    $i2c:ident,
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
    $name_res:ident,
    ($($tp:ident: $bound:path),*),
    ($((
      [$($dma_rx_attr:meta,)*],
      $dma_rx_res:ident,
      $int_dma_rx:ident,
      $dma_rx_cs:expr,
      ($($dma_rx_tp:ident: $dma_rx_bound:path),*)
    ),)*),
    ($((
      [$($dma_tx_attr:meta,)*],
      $dma_tx_res:ident,
      $int_dma_tx:ident,
      $dma_tx_cs:expr,
      ($($dma_tx_tp:ident: $dma_tx_bound:path),*)
    ),)*),
  ) => {
    impl<$($tp: $bound),*> I2CRes for $name_res<$($tp,)* Frt> {
      type Cr1 = $i2c::Cr1<Srt>;
      type Cr2 = $i2c::Cr2<Srt>;
      type Oar1 = $i2c::Oar1<Srt>;
      type Oar2 = $i2c::Oar2<Srt>;
      type Timingr = $i2c::Timingr<Srt>;
      type Timeoutr = $i2c::Timeoutr<Srt>;
      type Isr = $i2c::Isr<Frt>;
      type IsrNackf = $i2c::isr::Nackf<Frt>;
      type IsrStopf = $i2c::isr::Stopf<Frt>;
      type IsrTc = $i2c::isr::Tc<Frt>;
      type IsrTcr = $i2c::isr::Tcr<Frt>;
      type Icr = $i2c::Icr<Frt>;
      type IcrNackcf = $i2c::icr::Nackcf<Frt>;
      type IcrStopcf = $i2c::icr::Stopcf<Frt>;
      type Pecr = $i2c::Pecr<Srt>;
      type Rxdr = $i2c::Rxdr<Srt>;
      type Txdr = $i2c::Txdr<Srt>;

      res_reg_impl!(Cr1, cr1, cr1_mut, $i2c_cr1);
      res_reg_impl!(Cr2, cr2, cr2_mut, $i2c_cr2);
      res_reg_impl!(Oar1, oar1, oar1_mut, $i2c_oar1);
      res_reg_impl!(Oar2, oar2, oar2_mut, $i2c_oar2);
      res_reg_impl!(Timingr, timingr, timingr_mut, $i2c_timingr);
      res_reg_impl!(Timeoutr, timeoutr, timeoutr_mut, $i2c_timeoutr);
      res_reg_impl!(Isr, isr, isr_mut, $i2c_isr);
      res_reg_field_impl!(IsrNackf, isr_nackf, isr_nackf_mut, $i2c_isr, nackf);
      res_reg_field_impl!(IsrStopf, isr_stopf, isr_stopf_mut, $i2c_isr, stopf);
      res_reg_field_impl!(IsrTc, isr_tc, isr_tc_mut, $i2c_isr, tc);
      res_reg_field_impl!(IsrTcr, isr_tcr, isr_tcr_mut, $i2c_isr, tcr);
      res_reg_impl!(Icr, icr, icr_mut, $i2c_icr);
      res_reg_field_impl!(IcrNackcf, icr_nackcf, icr_nackcf_mut, $i2c_icr,
                          nackcf);
      res_reg_field_impl!(IcrStopcf, icr_stopcf, icr_stopcf_mut, $i2c_icr,
                          stopcf);
      res_reg_impl!(Pecr, pecr, pecr_mut, $i2c_pecr);
      res_reg_impl!(Rxdr, rxdr, rxdr_mut, $i2c_rxdr);
      res_reg_impl!(Txdr, txdr, txdr_mut, $i2c_txdr);
    }

    $(
      $(#[$dma_rx_attr])*
      impl<$($dma_rx_tp,)* Rx> I2CDmaRxRes<$dma_rx_res<Rx, Frt>>
        for $name_res<$($dma_rx_tp,)* Frt>
      where
        Rx: $int_dma_rx<Ltt>,
        $($dma_rx_tp: $dma_rx_bound,)*
      {
        #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                  feature = "stm32l4x3", feature = "stm32l4x5",
                  feature = "stm32l4x6"))]
        #[inline(always)]
        fn dma_rx_ch_init(
          &self,
          cs_val: &mut CselrVal<$dma_rx_res<Rx, Frt>>,
          dma: &Dma<$dma_rx_res<Rx, Frt>>,
        ) {
          dma.cselr_cs().write(cs_val, $dma_rx_cs);
        }
      }
    )*

    $(
      $(#[$dma_tx_attr])*
      impl<$($dma_tx_tp,)* Tx> I2CDmaTxRes<$dma_tx_res<Tx, Frt>>
        for $name_res<$($dma_tx_tp,)* Frt>
      where
        Tx: $int_dma_tx<Ltt>,
        $($dma_tx_tp: $dma_tx_bound,)*
      {
        #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                  feature = "stm32l4x3", feature = "stm32l4x5",
                  feature = "stm32l4x6"))]
        #[inline(always)]
        fn dma_tx_ch_init(
          &self,
          cs_val: &mut CselrVal<$dma_tx_res<Tx, Frt>>,
          dma: &Dma<$dma_tx_res<Tx, Frt>>,
        ) {
          dma.cselr_cs().write(cs_val, $dma_tx_cs);
        }
      }
    )*
  };
}

#[allow(unused_macros)]
macro_rules! i2c {
  (
    $doc:expr,
    $name:ident,
    $name_macro:ident,
    $doc_int:expr,
    $name_int:ident,
    $name_int_macro:ident,
    $doc_res:expr,
    $name_res:ident,
    $doc_int_res:expr,
    $name_int_res:ident,
    $int_ev_ty:ident,
    $int_er_ty:ident,
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
    ($((
      $(#[$dma_rx_attr:meta])*
      $dma_rx_res:ident,
      $int_dma_rx:ident,
      $dma_rx_cs:expr
    )),*),
    ($((
      $(#[$dma_tx_attr:meta])*
      $dma_tx_res:ident,
      $int_dma_tx:ident,
      $dma_tx_cs:expr
    )),*),
  ) => {
    #[doc = $doc]
    pub type $name = I2C<$name_res<Frt>>;

    #[doc = $doc_int]
    pub type $name_int<Ev, Er> = I2C<$name_int_res<Ev, Er, Frt>>;

    #[doc = $doc_res]
    #[allow(missing_docs)]
    pub struct $name_res<Rt: RegTag> {
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
    }

    #[doc = $doc_int_res]
    #[allow(missing_docs)]
    pub struct $name_int_res<Ev, Er, Rt>
    where
      Ev: $int_ev_ty<Ltt>,
      Er: $int_er_ty<Ltt>,
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
    }

    /// Creates a new `I2C`.
    #[macro_export]
    macro_rules! $name_macro {
      ($reg: ident) => {
        $crate::drv::i2c::I2C::new($crate::drv::i2c::$name_res {
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
        })
      };
    }

    /// Creates a new `I2CInt`.
    #[macro_export]
    macro_rules! $name_int_macro {
      ($reg: ident,$thr: ident) => {
        $crate::drv::i2c::I2C::new($crate::drv::i2c::$name_int_res {
          $i2c_ev: $thr.$i2c_ev.into(),
          $i2c_er: $thr.$i2c_er.into(),
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
        })
      };
    }

    impl Resource for $name_res<Frt> {
      type Source = $name_res<Srt>;

      #[inline(always)]
      fn from_source(source: Self::Source) -> Self {
        Self {
          $i2c_cr1: source.$i2c_cr1,
          $i2c_cr2: source.$i2c_cr2,
          $i2c_oar1: source.$i2c_oar1,
          $i2c_oar2: source.$i2c_oar2,
          $i2c_timingr: source.$i2c_timingr,
          $i2c_timeoutr: source.$i2c_timeoutr,
          $i2c_isr: source.$i2c_isr.into(),
          $i2c_icr: source.$i2c_icr.into(),
          $i2c_pecr: source.$i2c_pecr,
          $i2c_rxdr: source.$i2c_rxdr,
          $i2c_txdr: source.$i2c_txdr,
        }
      }
    }

    i2c_shared! {
      $i2c,
      $i2c_cr1,
      $i2c_cr2,
      $i2c_oar1,
      $i2c_oar2,
      $i2c_timingr,
      $i2c_timeoutr,
      $i2c_isr,
      $i2c_icr,
      $i2c_pecr,
      $i2c_rxdr,
      $i2c_txdr,
      $name_res,
      (),
      ($(([$($dma_rx_attr,)*], $dma_rx_res, $int_dma_rx, $dma_rx_cs, ()),)*),
      ($(([$($dma_tx_attr,)*], $dma_tx_res, $int_dma_tx, $dma_tx_cs, ()),)*),
    }

    impl<Ev, Er> Resource for $name_int_res<Ev, Er, Frt>
    where
      Ev: $int_ev_ty<Ltt>,
      Er: $int_er_ty<Ltt>,
    {
      type Source = $name_int_res<Ev, Er, Srt>;

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
          $i2c_isr: source.$i2c_isr.into(),
          $i2c_icr: source.$i2c_icr.into(),
          $i2c_pecr: source.$i2c_pecr,
          $i2c_rxdr: source.$i2c_rxdr,
          $i2c_txdr: source.$i2c_txdr,
        }
      }
    }

    i2c_shared! {
      $i2c,
      $i2c_cr1,
      $i2c_cr2,
      $i2c_oar1,
      $i2c_oar2,
      $i2c_timingr,
      $i2c_timeoutr,
      $i2c_isr,
      $i2c_icr,
      $i2c_pecr,
      $i2c_rxdr,
      $i2c_txdr,
      $name_int_res,
      (Ev: $int_ev_ty<Ltt>, Er: $int_er_ty<Ltt>),
      ($((
        [$($dma_rx_attr,)*], $dma_rx_res, $int_dma_rx, $dma_rx_cs,
        (Ev: $int_ev_ty<Ltt>, Er: $int_er_ty<Ltt>)
      ),)*),
      ($((
        [$($dma_tx_attr,)*], $dma_tx_res, $int_dma_tx, $dma_tx_cs,
        (Ev: $int_ev_ty<Ltt>, Er: $int_er_ty<Ltt>)
      ),)*),
    }

    impl<Ev, Er> I2CIntRes for $name_int_res<Ev, Er, Frt>
    where
      Ev: $int_ev_ty<Ltt>,
      Er: $int_er_ty<Ltt>,
    {
      type WithoutInt = $name_res<Frt>;
      type IntEv = Ev;
      type IntEr = Er;

      #[inline(always)]
      fn join_int(
        res: Self::WithoutInt,
        int_ev: Self::IntEv,
        int_er: Self::IntEr,
      ) -> Self {
        $name_int_res {
          $i2c_ev: int_ev,
          $i2c_er: int_er,
          $i2c_cr1: res.$i2c_cr1,
          $i2c_cr2: res.$i2c_cr2,
          $i2c_oar1: res.$i2c_oar1,
          $i2c_oar2: res.$i2c_oar2,
          $i2c_timingr: res.$i2c_timingr,
          $i2c_timeoutr: res.$i2c_timeoutr,
          $i2c_isr: res.$i2c_isr,
          $i2c_icr: res.$i2c_icr,
          $i2c_pecr: res.$i2c_pecr,
          $i2c_rxdr: res.$i2c_rxdr,
          $i2c_txdr: res.$i2c_txdr,
        }
      }

      #[inline(always)]
      fn split_int(self) -> (Self::WithoutInt, Self::IntEv, Self::IntEr) {
        (
          $name_res {
            $i2c_cr1: self.$i2c_cr1,
            $i2c_cr2: self.$i2c_cr2,
            $i2c_oar1: self.$i2c_oar1,
            $i2c_oar2: self.$i2c_oar2,
            $i2c_timingr: self.$i2c_timingr,
            $i2c_timeoutr: self.$i2c_timeoutr,
            $i2c_isr: self.$i2c_isr,
            $i2c_icr: self.$i2c_icr,
            $i2c_pecr: self.$i2c_pecr,
            $i2c_rxdr: self.$i2c_rxdr,
            $i2c_txdr: self.$i2c_txdr,
          },
          self.$i2c_ev,
          self.$i2c_er,
        )
      }

      #[inline(always)]
      fn int_ev(&self) -> Self::IntEv {
        self.$i2c_ev
      }

      #[inline(always)]
      fn int_er(&self) -> Self::IntEr {
        self.$i2c_er
      }
    }
  };
}

#[cfg(any(
  feature = "stm32f100",
  feature = "stm32f101",
  feature = "stm32f102",
  feature = "stm32f103",
  feature = "stm32f107",
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
i2c! {
  "I2C1 driver.",
  I2C1,
  drv_i2c1,
  "I2C1 driver with interrupt.",
  I2C1Int,
  drv_i2c1_int,
  "I2C1 resource.",
  I2C1Res,
  "I2C1 resource with interrupt.",
  I2C1IntRes,
  IntI2C1Ev,
  IntI2C1Er,
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
  (
    (Dma1Ch7Res, IntDma1Ch7, 3),
    (
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      Dma2Ch6Res, IntDma2Ch6, 5
    )
  ),
  (
    (Dma1Ch6Res, IntDma1Ch6, 3),
    (
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      Dma2Ch7Res, IntDma2Ch7, 5
    )
  ),
}

#[cfg(any(
  feature = "stm32f100",
  feature = "stm32f101",
  feature = "stm32f102",
  feature = "stm32f103",
  feature = "stm32f107",
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
i2c! {
  "I2C2 driver.",
  I2C2,
  drv_i2c2,
  "I2C2 driver with interrupt.",
  I2C2Int,
  drv_i2c2_int,
  "I2C2 resource.",
  I2C2Res,
  "I2C2 resource with interrupt.",
  I2C2IntRes,
  IntI2C2Ev,
  IntI2C2Er,
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
  ((Dma1Ch5Res, IntDma1Ch5, 3)),
  ((Dma1Ch4Res, IntDma1Ch4, 3)),
}

#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
i2c! {
  "I2C3 driver.",
  I2C3,
  drv_i2c3,
  "I2C3 driver with interrupt.",
  I2C3Int,
  drv_i2c3_int,
  "I2C3 resource.",
  I2C3Res,
  "I2C3 resource with interrupt.",
  I2C3IntRes,
  IntI2C3Ev,
  IntI2C3Er,
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
  ((Dma1Ch3Res, IntDma1Ch3, 3)),
  ((Dma1Ch2Res, IntDma1Ch2, 3)),
}

#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6"
))]
i2c! {
  "I2C4 driver.",
  I2C4,
  drv_i2c4,
  "I2C4 driver with interrupt.",
  I2C4Int,
  drv_i2c4_int,
  "I2C4 resource.",
  I2C4Res,
  "I2C4 resource with interrupt.",
  I2C4IntRes,
  IntI2C4Ev,
  IntI2C4Er,
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
  ((Dma2Ch1Res, IntDma2Ch1, 0)),
  ((Dma2Ch2Res, IntDma2Ch2, 0)),
}
