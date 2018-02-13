//! Serial peripheral interface.

#[allow(unused_imports)]
use core::ptr::{read_volatile, write_volatile};
use drivers::dma::{Dma, DmaRes};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use drivers::dma::{Dma1Ch2Res, Dma1Ch3Res, Dma1Ch4Res, Dma1Ch5Res, Dma2Ch1Res,
                   Dma2Ch2Res};
#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
use drivers::dma::{Dma2Ch3Res, Dma2Ch4Res};
use drivers::prelude::*;
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use reg::{spi1, spi2, spi3};
use reg::marker::*;
use reg::prelude::*;
#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x6"))]
use thread::irq::{IrqDma1Ch2, IrqDma1Ch3, IrqDma1Ch4, IrqDma1Ch5, IrqDma2Ch1,
                  IrqDma2Ch2, IrqDma2Ch3, IrqDma2Ch4};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x3",
          feature = "stm32l4x5"))]
use thread::irq::{IrqDma1Channel2 as IrqDma1Ch2,
                  IrqDma1Channel3 as IrqDma1Ch3,
                  IrqDma1Channel4 as IrqDma1Ch4,
                  IrqDma1Channel5 as IrqDma1Ch5,
                  IrqDma2Channel1 as IrqDma2Ch1, IrqDma2Channel2 as IrqDma2Ch2};
#[cfg(any(feature = "stm32l4x3", feature = "stm32l4x5"))]
use thread::irq::{IrqDma2Channel3 as IrqDma2Ch3, IrqDma2Channel4 as IrqDma2Ch4};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use thread::irq::{IrqSpi1, IrqSpi2, IrqSpi3};
use thread::prelude::*;

/// SPI driver.
pub struct Spi<T: SpiRes>(T);

/// DMA-driven SPI driver.
pub trait SpiDmaTx<T, Tx>
where
  T: SpiDmaTxRes<Tx>,
  Tx: DmaRes,
{
  /// Initializes DMA for the SPI as peripheral.
  fn dma_tx_init(&self, dma_tx: &Dma<Tx>);

  /// Initializes DMA for the SPI as peripheral.
  fn dma_tx_paddr_init(&self, dma_tx: &Dma<Tx>);
}

/// DMA-driven SPI driver.
pub trait SpiDmaRx<T, Rx>
where
  T: SpiDmaRxRes<Rx>,
  Rx: DmaRes,
{
  /// Initializes DMA for the SPI as peripheral.
  fn dma_rx_init(&self, dma_rx: &Dma<Rx>);

  /// Initializes DMA for the SPI as peripheral.
  fn dma_rx_paddr_init(&self, dma_rx: &Dma<Rx>);
}

/// DMA-driven SPI driver.
pub trait SpiDmaDx<T, Tx, Rx>
where
  T: SpiDmaTxRes<Tx> + SpiDmaRxRes<Rx>,
  Tx: DmaRes,
  Rx: DmaRes,
{
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  /// Initializes DMA for the SPI as peripheral.
  fn dma_dx_init(&self, dma_tx: &Dma<Tx>, dma_rx: &Dma<Rx>)
  where
    Tx: DmaRes<Cselr = Rx::Cselr>;

  #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6")))]
  /// Initializes DMA for the SPI as peripheral.
  fn dma_dx_init(&self, dma_tx: &Dma<Tx>, dma_rx: &Dma<Rx>);

  /// Initializes DMA for the SPI as peripheral.
  fn dma_dx_paddr_init(&self, dma_tx: &Dma<Tx>, dma_rx: &Dma<Rx>);
}

/// SPI resource.
#[allow(missing_docs)]
pub trait SpiRes: Resource<Input = Self> {
  type Cr1: SRwRegBitBand;
  type Cr2: SRwRegBitBand;
  type Crcpr: SRwRegBitBand;
  type Dr: SRwRegBitBand;
  type Rxcrcr: SRoRegBitBand;
  type Sr: SRwRegBitBand;
  type SrBsy: SRoRwRegFieldBitBand<Reg = Self::Sr>;
  type SrRxne: SRoRwRegFieldBitBand<Reg = Self::Sr>;
  type Txcrcr: SRoRegBitBand;

  res_reg_decl!(Cr1, cr1, cr1_mut);
  res_reg_decl!(Cr2, cr2, cr2_mut);
  res_reg_decl!(Crcpr, crcpr, crcpr_mut);
  res_reg_decl!(Dr, dr, dr_mut);
  res_reg_decl!(Rxcrcr, rxcrcr, rxcrcr_mut);
  res_reg_decl!(Sr, sr, sr_mut);
  res_reg_decl!(SrBsy, sr_bsy, sr_bsy_mut);
  res_reg_decl!(SrRxne, sr_rxne, sr_rxne_mut);
  res_reg_decl!(Txcrcr, txcrcr, txcrcr_mut);
}

/// Interrupt-driven SPI resource.
#[allow(missing_docs)]
pub trait SpiIrqRes: SpiRes {
  type WithoutIrq: SpiRes;
  type Irq: IrqToken<Ltt>;

  fn join_irq(res: Self::WithoutIrq, irq: Self::Irq) -> Self;
  fn split_irq(self) -> (Self::WithoutIrq, Self::Irq);

  fn irq(&self) -> Self::Irq;
}

/// DMA-driven SPI resource.
#[allow(missing_docs)]
pub trait SpiDmaTxRes<T: DmaRes>: SpiRes {
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  fn dma_tx_ch_init(&self, cs_val: &mut CselrVal<T>, dma: &Dma<T>);
}

/// DMA-driven SPI resource.
#[allow(missing_docs)]
pub trait SpiDmaRxRes<T: DmaRes>: SpiRes {
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  fn dma_rx_ch_init(&self, cs_val: &mut CselrVal<T>, dma: &Dma<T>);
}

#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
type CselrVal<T> = <<T as DmaRes>::Cselr as Reg<Srt>>::Val;

impl<T: SpiRes> Driver for Spi<T> {
  type Resource = T;

  #[inline(always)]
  fn from_res(res: T::Input) -> Self {
    Spi(res)
  }

  #[inline(always)]
  fn into_res(self) -> T {
    self.0
  }
}

#[allow(missing_docs)]
impl<T: SpiRes> Spi<T> {
  #[inline(always)]
  pub fn cr1(&self) -> &T::Cr1 {
    self.0.cr1()
  }

  #[inline(always)]
  pub fn cr2(&self) -> &T::Cr2 {
    self.0.cr2()
  }

  #[inline(always)]
  pub fn crcpr(&self) -> &T::Crcpr {
    self.0.crcpr()
  }

  #[inline(always)]
  pub fn dr(&self) -> &T::Dr {
    self.0.dr()
  }

  #[inline(always)]
  pub fn rxcrcr(&self) -> &T::Rxcrcr {
    self.0.rxcrcr()
  }

  #[inline(always)]
  pub fn sr(&self) -> &T::Sr {
    self.0.sr()
  }

  #[inline(always)]
  pub fn sr_bsy(&self) -> &T::SrBsy {
    self.0.sr_bsy()
  }

  #[inline(always)]
  pub fn sr_rxne(&self) -> &T::SrRxne {
    self.0.sr_rxne()
  }

  #[inline(always)]
  pub fn txcrcr(&self) -> &T::Txcrcr {
    self.0.txcrcr()
  }

  /// Writes `u8` value to the data register.
  #[inline(always)]
  pub fn send_byte(&self, value: u8) {
    unsafe {
      write_volatile(self.0.dr().to_mut_ptr() as *mut _, value);
    }
  }

  /// Writes `u16` value to the data register.
  #[inline(always)]
  pub fn send_hword(&self, value: u16) {
    unsafe {
      write_volatile(self.0.dr().to_mut_ptr() as *mut _, value);
    }
  }

  /// Reads `u8` value from the data register.
  #[inline(always)]
  pub fn recv_byte(&self) -> u8 {
    unsafe { read_volatile(self.0.dr().to_ptr() as *const _) }
  }

  /// Reads `u16` value from the data register.
  #[inline(always)]
  pub fn recv_hword(&self) -> u16 {
    unsafe { read_volatile(self.0.dr().to_ptr() as *const _) }
  }

  /// Waits while SPI is busy in communication or Tx buffer is not empty.
  #[inline(always)]
  pub fn busy_wait(&self) {
    while self.0.sr_bsy().read_bit_band() {}
  }
}

#[allow(missing_docs)]
impl<T: SpiIrqRes> Spi<T> {
  #[inline(always)]
  pub fn join_irq(res: Spi<T::WithoutIrq>, irq: T::Irq) -> Spi<T> {
    Spi(T::join_irq(res.0, irq))
  }

  #[inline(always)]
  pub fn split_irq(self) -> (Spi<T::WithoutIrq>, T::Irq) {
    let (res, irq) = self.0.split_irq();
    (Spi(res), irq)
  }

  #[inline(always)]
  pub fn irq(&self) -> T::Irq {
    self.0.irq()
  }
}

#[allow(missing_docs)]
impl<T, Tx> SpiDmaTx<T, Tx> for Spi<T>
where
  T: SpiDmaTxRes<Tx>,
  Tx: DmaRes,
{
  #[inline(always)]
  fn dma_tx_init(&self, dma_tx: &Dma<Tx>) {
    self.dma_tx_paddr_init(dma_tx);
    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    dma_tx.cselr_cs().modify(|r| {
      self.0.dma_tx_ch_init(r, dma_tx);
    });
  }

  #[inline(always)]
  fn dma_tx_paddr_init(&self, dma_tx: &Dma<Tx>) {
    unsafe { dma_tx.set_paddr(self.0.dr().to_mut_ptr() as usize) };
  }
}

#[allow(missing_docs)]
impl<T, Rx> SpiDmaRx<T, Rx> for Spi<T>
where
  T: SpiDmaRxRes<Rx>,
  Rx: DmaRes,
{
  #[inline(always)]
  fn dma_rx_init(&self, dma_rx: &Dma<Rx>) {
    self.dma_rx_paddr_init(dma_rx);
    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    dma_rx.cselr_cs().modify(|r| {
      self.0.dma_rx_ch_init(r, dma_rx);
    });
  }

  #[inline(always)]
  fn dma_rx_paddr_init(&self, dma_rx: &Dma<Rx>) {
    unsafe { dma_rx.set_paddr(self.0.dr().to_ptr() as usize) };
  }
}

#[allow(missing_docs)]
impl<T, Tx, Rx> SpiDmaDx<T, Tx, Rx> for Spi<T>
where
  T: SpiDmaTxRes<Tx> + SpiDmaRxRes<Rx>,
  Tx: DmaRes,
  Rx: DmaRes,
{
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  #[inline(always)]
  fn dma_dx_init(&self, dma_tx: &Dma<Tx>, dma_rx: &Dma<Rx>)
  where
    Tx: DmaRes<Cselr = Rx::Cselr>,
  {
    self.dma_dx_paddr_init(dma_tx, dma_rx);
    dma_tx.cselr_cs().modify(|r| {
      self.0.dma_tx_ch_init(r, dma_tx);
      self.0.dma_rx_ch_init(r, dma_rx);
    });
  }

  #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6")))]
  #[inline(always)]
  fn dma_dx_init(&self, dma_tx: &Dma<Tx>, dma_rx: &Dma<Rx>) {
    self.dma_dx_paddr_init(dma_tx, dma_rx);
  }

  #[inline(always)]
  fn dma_dx_paddr_init(&self, dma_tx: &Dma<Tx>, dma_rx: &Dma<Rx>) {
    self.dma_tx_paddr_init(dma_tx);
    self.dma_rx_paddr_init(dma_rx);
  }
}

#[allow(unused_macros)]
macro_rules! spi_shared {
  (
    $spi:ident,
    $spi_cr1:ident,
    $spi_cr2:ident,
    $spi_crcpr:ident,
    $spi_dr:ident,
    $spi_rxcrcr:ident,
    $spi_sr:ident,
    $spi_txcrcr:ident,
    $name_res:ident,
    ($($tp:ident: $bound:path),*),
    ($((
      [$($dma_tx_attr:meta,)*],
      $dma_tx_res:ident,
      $irq_dma_tx:ident,
      $dma_tx_cs:expr,
      ($($dma_tx_tp:ident: $dma_tx_bound:path),*)
    ),)*),
    ($((
      [$($dma_rx_attr:meta,)*],
      $dma_rx_res:ident,
      $irq_dma_rx:ident,
      $dma_rx_cs:expr,
      ($($dma_rx_tp:ident: $dma_rx_bound:path),*)
    ),)*),
  ) => {
    impl<$($tp: $bound,)*> SpiRes for $name_res<$($tp),*> {
      type Cr1 = $spi::Cr1<Srt>;
      type Cr2 = $spi::Cr2<Srt>;
      type Crcpr = $spi::Crcpr<Srt>;
      type Dr = $spi::Dr<Srt>;
      type Rxcrcr = $spi::Rxcrcr<Srt>;
      type Sr = $spi::Sr<Srt>;
      type SrBsy = $spi::sr::Bsy<Srt>;
      type SrRxne = $spi::sr::Rxne<Srt>;
      type Txcrcr = $spi::Txcrcr<Srt>;

      res_reg_impl!(Cr1, cr1, cr1_mut, $spi_cr1);
      res_reg_impl!(Cr2, cr2, cr2_mut, $spi_cr2);
      res_reg_impl!(Crcpr, crcpr, crcpr_mut, $spi_crcpr);
      res_reg_impl!(Dr, dr, dr_mut, $spi_dr);
      res_reg_impl!(Rxcrcr, rxcrcr, rxcrcr_mut, $spi_rxcrcr);
      res_reg_impl!(Sr, sr, sr_mut, $spi_sr);
      res_reg_field_impl!(SrBsy, sr_bsy, sr_bsy_mut, $spi_sr, bsy);
      res_reg_field_impl!(SrRxne, sr_rxne, sr_rxne_mut, $spi_sr, rxne);
      res_reg_impl!(Txcrcr, txcrcr, txcrcr_mut, $spi_txcrcr);
    }

    $(
      $(#[$dma_tx_attr])*
      impl<$($dma_tx_tp,)* Tx> SpiDmaTxRes<$dma_tx_res<Tx, Frt>>
        for $name_res<$($dma_tx_tp),*>
      where
        Tx: $irq_dma_tx<Ltt>,
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

    $(
      $(#[$dma_rx_attr])*
      impl<$($dma_rx_tp,)* Rx> SpiDmaRxRes<$dma_rx_res<Rx, Frt>>
        for $name_res<$($dma_rx_tp),*>
      where
        Rx: $irq_dma_rx<Ltt>,
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
  }
}

#[allow(unused_macros)]
macro_rules! spi {
  (
    $doc:expr,
    $name:ident,
    $name_macro:ident,
    $doc_irq:expr,
    $name_irq:ident,
    $name_irq_macro:ident,
    $doc_res:expr,
    $name_res:ident,
    $doc_irq_res:expr,
    $name_irq_res:ident,
    $irq_ty:ident,
    $spi:ident,
    $spi_cr1:ident,
    $spi_cr2:ident,
    $spi_crcpr:ident,
    $spi_dr:ident,
    $spi_rxcrcr:ident,
    $spi_sr:ident,
    $spi_txcrcr:ident,
    ($((
      $(#[$dma_tx_attr:meta])*
      $dma_tx_res:ident,
      $irq_dma_tx:ident,
      $dma_tx_cs:expr
    )),*),
    ($((
      $(#[$dma_rx_attr:meta])*
      $dma_rx_res:ident,
      $irq_dma_rx:ident,
      $dma_rx_cs:expr
    )),*),
  ) => {
    #[doc = $doc]
    pub type $name = Spi<$name_res>;

    #[doc = $doc_irq]
    pub type $name_irq<I> = Spi<$name_irq_res<I>>;

    #[doc = $doc_res]
    #[allow(missing_docs)]
    pub struct $name_res {
      pub $spi_cr1: $spi::Cr1<Srt>,
      pub $spi_cr2: $spi::Cr2<Srt>,
      pub $spi_crcpr: $spi::Crcpr<Srt>,
      pub $spi_dr: $spi::Dr<Srt>,
      pub $spi_rxcrcr: $spi::Rxcrcr<Srt>,
      pub $spi_sr: $spi::Sr<Srt>,
      pub $spi_txcrcr: $spi::Txcrcr<Srt>,
    }

    #[doc = $doc_irq_res]
    #[allow(missing_docs)]
    pub struct $name_irq_res<I: $irq_ty<Ltt>> {
      pub $spi: I,
      pub $spi_cr1: $spi::Cr1<Srt>,
      pub $spi_cr2: $spi::Cr2<Srt>,
      pub $spi_crcpr: $spi::Crcpr<Srt>,
      pub $spi_dr: $spi::Dr<Srt>,
      pub $spi_rxcrcr: $spi::Rxcrcr<Srt>,
      pub $spi_sr: $spi::Sr<Srt>,
      pub $spi_txcrcr: $spi::Txcrcr<Srt>,
    }

    /// Creates a new `Spi`.
    #[macro_export]
    macro_rules! $name_macro {
      ($regs:ident) => {
        $crate::drivers::spi::Spi::from_res(
          $crate::drivers::spi::$name_res {
            $spi_cr1: $regs.$spi_cr1,
            $spi_cr2: $regs.$spi_cr2,
            $spi_crcpr: $regs.$spi_crcpr,
            $spi_dr: $regs.$spi_dr,
            $spi_rxcrcr: $regs.$spi_rxcrcr,
            $spi_sr: $regs.$spi_sr,
            $spi_txcrcr: $regs.$spi_txcrcr,
          }
        )
      }
    }

    /// Creates a new `SpiIrq`.
    #[macro_export]
    macro_rules! $name_irq_macro {
      ($regs:ident, $thrd:ident) => {
        $crate::drivers::spi::Spi::from_res(
          $crate::drivers::spi::$name_irq_res {
            $spi: $thrd.$spi.into(),
            $spi_cr1: $regs.$spi_cr1,
            $spi_cr2: $regs.$spi_cr2,
            $spi_crcpr: $regs.$spi_crcpr,
            $spi_dr: $regs.$spi_dr,
            $spi_rxcrcr: $regs.$spi_rxcrcr,
            $spi_sr: $regs.$spi_sr,
            $spi_txcrcr: $regs.$spi_txcrcr,
          }
        )
      }
    }

    impl Resource for $name_res {
      // FIXME https://github.com/rust-lang/rust/issues/47385
      type Input = Self;
    }

    spi_shared! {
      $spi,
      $spi_cr1,
      $spi_cr2,
      $spi_crcpr,
      $spi_dr,
      $spi_rxcrcr,
      $spi_sr,
      $spi_txcrcr,
      $name_res,
      (),
      ($(([$($dma_tx_attr,)*], $dma_tx_res, $irq_dma_tx, $dma_tx_cs, ()),)*),
      ($(([$($dma_rx_attr,)*], $dma_rx_res, $irq_dma_rx, $dma_rx_cs, ()),)*),
    }

    impl<I: $irq_ty<Ltt>> Resource for $name_irq_res<I> {
      // FIXME https://github.com/rust-lang/rust/issues/47385
      type Input = Self;
    }

    spi_shared! {
      $spi,
      $spi_cr1,
      $spi_cr2,
      $spi_crcpr,
      $spi_dr,
      $spi_rxcrcr,
      $spi_sr,
      $spi_txcrcr,
      $name_irq_res,
      (I: $irq_ty<Ltt>),
      ($((
        [$($dma_tx_attr,)*], $dma_tx_res, $irq_dma_tx, $dma_tx_cs,
        (I: $irq_ty<Ltt>)
      ),)*),
      ($((
        [$($dma_rx_attr,)*], $dma_rx_res, $irq_dma_rx, $dma_rx_cs,
        (I: $irq_ty<Ltt>)
      ),)*),
    }

    impl<I: $irq_ty<Ltt>> SpiIrqRes for $name_irq_res<I> {
      type WithoutIrq = $name_res;
      type Irq = I;

      #[inline(always)]
      fn join_irq(res: Self::WithoutIrq, irq: Self::Irq) -> Self {
        $name_irq_res {
          $spi: irq,
          $spi_cr1: res.$spi_cr1,
          $spi_cr2: res.$spi_cr2,
          $spi_crcpr: res.$spi_crcpr,
          $spi_dr: res.$spi_dr,
          $spi_rxcrcr: res.$spi_rxcrcr,
          $spi_sr: res.$spi_sr,
          $spi_txcrcr: res.$spi_txcrcr,
        }
      }

      #[inline(always)]
      fn split_irq(self) -> (Self::WithoutIrq, Self::Irq) {
        (
          $name_res {
            $spi_cr1: self.$spi_cr1,
            $spi_cr2: self.$spi_cr2,
            $spi_crcpr: self.$spi_crcpr,
            $spi_dr: self.$spi_dr,
            $spi_rxcrcr: self.$spi_rxcrcr,
            $spi_sr: self.$spi_sr,
            $spi_txcrcr: self.$spi_txcrcr,
          },
          self.$spi,
        )
      }

      #[inline(always)]
      fn irq(&self) -> Self::Irq {
        self.$spi
      }
    }
  }
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
spi! {
  "SPI1 driver.",
  Spi1,
  drv_spi1,
  "SPI1 driver with interrupt.",
  Spi1Irq,
  drv_spi1_irq,
  "SPI1 resource.",
  Spi1Res,
  "SPI1 resource with interrupt.",
  Spi1IrqRes,
  IrqSpi1,
  spi1,
  spi1_cr1,
  spi1_cr2,
  spi1_crcpr,
  spi1_dr,
  spi1_rxcrcr,
  spi1_sr,
  spi1_txcrcr,
  (
    (Dma1Ch3Res, IrqDma1Ch3, 0b0001),
    (
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      Dma2Ch4Res, IrqDma2Ch4, 0b0100
    )
  ),
  (
    (Dma1Ch2Res, IrqDma1Ch2, 0b0001),
    (
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      Dma2Ch3Res, IrqDma2Ch3, 0b0100
    )
  ),
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
spi! {
  "SPI2 driver.",
  Spi2,
  drv_spi2,
  "SPI2 driver with interrupt.",
  Spi2Irq,
  drv_spi2_irq,
  "SPI2 resource.",
  Spi2Res,
  "SPI2 resource with interrupt.",
  Spi2IrqRes,
  IrqSpi2,
  spi2,
  spi2_cr1,
  spi2_cr2,
  spi2_crcpr,
  spi2_dr,
  spi2_rxcrcr,
  spi2_sr,
  spi2_txcrcr,
  ((Dma1Ch5Res, IrqDma1Ch5, 0b0001)),
  ((Dma1Ch4Res, IrqDma1Ch4, 0b0001)),
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
spi! {
  "SPI3 driver.",
  Spi3,
  drv_spi3,
  "SPI3 driver with interrupt.",
  Spi3Irq,
  drv_spi3_irq,
  "SPI3 resource.",
  Spi3Res,
  "SPI3 resource with interrupt.",
  Spi3IrqRes,
  IrqSpi3,
  spi3,
  spi3_cr1,
  spi3_cr2,
  spi3_crcpr,
  spi3_dr,
  spi3_rxcrcr,
  spi3_sr,
  spi3_txcrcr,
  ((Dma2Ch2Res, IrqDma2Ch2, 0b0011)),
  ((Dma2Ch1Res, IrqDma2Ch1, 0b0011)),
}
