//! Serial peripheral interface.

#[allow(unused_imports)]
use core::ptr::{read_volatile, write_volatile};
use drone_core::peripherals::{PeripheralDevice, PeripheralTokens};
use peripherals::dma::{Dma, DmaTokens};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use peripherals::dma::{Dma1Ch2Tokens, Dma1Ch3Tokens, Dma1Ch4Tokens,
                       Dma1Ch5Tokens, Dma2Ch1Tokens, Dma2Ch2Tokens};
#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
use peripherals::dma::{Dma2Ch3Tokens, Dma2Ch4Tokens};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use reg::{spi1, spi2, spi3};
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

/// Generic SPI.
pub struct Spi<T: SpiTokens>(T);

/// Generic DMA-driven SPI tokens.
pub trait SpiDmaTx<T, Tx>
where
  T: SpiTokensDmaTx<Tx>,
  Tx: DmaTokens,
{
  /// Initializes DMA for the SPI as peripheral.
  fn dma_tx_init(&self, dma_tx: &Dma<Tx>);

  /// Initializes DMA for the SPI as peripheral.
  fn dma_tx_peripheral_addr_init(&self, dma_tx: &Dma<Tx>);
}

/// Generic DMA-driven SPI tokens.
pub trait SpiDmaRx<T, Rx>
where
  T: SpiTokensDmaRx<Rx>,
  Rx: DmaTokens,
{
  /// Initializes DMA for the SPI as peripheral.
  fn dma_rx_init(&self, dma_rx: &Dma<Rx>);

  /// Initializes DMA for the SPI as peripheral.
  fn dma_rx_peripheral_addr_init(&self, dma_rx: &Dma<Rx>);
}

/// Generic DMA-driven SPI tokens.
pub trait SpiDmaDx<T, Tx, Rx>
where
  T: SpiTokensDmaTx<Tx> + SpiTokensDmaRx<Rx>,
  Tx: DmaTokens,
  Rx: DmaTokens,
{
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  /// Initializes DMA for the SPI as peripheral.
  fn dma_dx_init(&self, dma_tx: &Dma<Tx>, dma_rx: &Dma<Rx>)
  where
    Tx: DmaTokens<Cselr = Rx::Cselr>;

  #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6")))]
  /// Initializes DMA for the SPI as peripheral.
  fn dma_dx_init(&self, dma_tx: &Dma<Tx>, dma_rx: &Dma<Rx>);

  /// Initializes DMA for the SPI as peripheral.
  fn dma_dx_peripheral_addr_init(&self, dma_tx: &Dma<Tx>, dma_rx: &Dma<Rx>);
}

/// Generic SPI tokens.
#[allow(missing_docs)]
pub trait SpiTokens: PeripheralTokens {
  type Cr1: for<'a> RwRegAtomicRef<'a, Frt> + RegBitBand<Frt> + RegFork;
  type Cr2: for<'a> RwRegAtomicRef<'a, Frt> + RegBitBand<Frt> + RegFork;
  type Crcpr: for<'a> RwRegAtomicRef<'a, Srt>;
  type Dr: for<'a> RwRegAtomicRef<'a, Srt>;
  type Rxcrcr: RoReg<Srt>;
  type Sr: for<'a> RwRegAtomicRef<'a, Srt> + RegBitBand<Srt>;
  type SrBsy: RegField<Srt, Reg = Self::Sr> + RRegFieldBitBand<Srt>;
  type Txcrcr: RoReg<Srt>;

  fn cr1(&self) -> &Self::Cr1;
  fn cr1_mut(&mut self) -> &mut Self::Cr1;
  fn cr2(&self) -> &Self::Cr2;
  fn cr2_mut(&mut self) -> &mut Self::Cr2;
  fn crcpr(&self) -> &Self::Crcpr;
  fn dr(&self) -> &Self::Dr;
  fn rxcrcr(&self) -> &Self::Rxcrcr;
  fn sr(&self) -> &Self::Sr;
  fn sr_bsy(&self) -> &Self::SrBsy;
  fn txcrcr(&self) -> &Self::Txcrcr;
}

/// Generic interrupt-driven SPI tokens.
#[allow(missing_docs)]
pub trait SpiTokensIrq: SpiTokens {
  type WithoutIrq: SpiTokens;
  type Irq: IrqToken<Ltt>;

  fn join_irq(tokens: Self::WithoutIrq, irq: Self::Irq) -> Self;
  fn split_irq(self) -> (Self::WithoutIrq, Self::Irq);

  fn irq(&self) -> Self::Irq;
}

/// Generic DMA-driven SPI tokens.
#[allow(missing_docs)]
pub trait SpiTokensDmaTx<T: DmaTokens>: SpiTokens {
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  fn dma_tx_ch_select(&self, cs_val: &mut CselrVal<T>, dma: &Dma<T>);
}

/// Generic DMA-driven SPI tokens.
#[allow(missing_docs)]
pub trait SpiTokensDmaRx<T: DmaTokens>: SpiTokens {
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  fn dma_rx_ch_select(&self, cs_val: &mut CselrVal<T>, dma: &Dma<T>);
}

#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
type CselrVal<T> = <<T as DmaTokens>::Cselr as Reg<Srt>>::Val;

impl<T: SpiTokens> PeripheralDevice for Spi<T> {
  type Tokens = T;

  #[inline(always)]
  fn from_tokens(tokens: T::InputTokens) -> Self {
    Spi(tokens.into())
  }

  #[inline(always)]
  fn into_tokens(self) -> T {
    self.0
  }
}

#[allow(missing_docs)]
impl<T: SpiTokens> Spi<T> {
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

  /// Moves `self` into `f`, and then sets `SPE`.
  #[inline(always)]
  pub fn store_cr1_after<F, R>(
    mut self,
    cr1_val: <T::Cr1 as Reg<Frt>>::Val,
    f: F,
  ) -> R
  where
    F: FnOnce(Self) -> R,
  {
    let cr1 = self.0.cr1_mut().fork();
    let result = f(self);
    cr1.store_val(cr1_val);
    result
  }

  /// Moves `self` into `f`, and then sets `TXDMAEN`.
  #[inline(always)]
  pub fn store_cr2_after<F, R>(
    mut self,
    cr2_val: <T::Cr2 as Reg<Frt>>::Val,
    f: F,
  ) -> R
  where
    F: FnOnce(Self) -> R,
  {
    let cr2 = self.0.cr2_mut().fork();
    let result = f(self);
    cr2.store_val(cr2_val);
    result
  }
}

#[allow(missing_docs)]
impl<T: SpiTokensIrq> Spi<T> {
  #[inline(always)]
  pub fn join_irq(tokens: Spi<T::WithoutIrq>, irq: T::Irq) -> Spi<T> {
    Spi(T::join_irq(tokens.0, irq))
  }

  #[inline(always)]
  pub fn split_irq(self) -> (Spi<T::WithoutIrq>, T::Irq) {
    let (tokens, irq) = self.0.split_irq();
    (Spi(tokens), irq)
  }

  #[inline(always)]
  pub fn irq(&self) -> T::Irq {
    self.0.irq()
  }
}

#[allow(missing_docs)]
impl<T, Tx> SpiDmaTx<T, Tx> for Spi<T>
where
  T: SpiTokensDmaTx<Tx>,
  Tx: DmaTokens,
{
  #[inline(always)]
  fn dma_tx_init(&self, dma_tx: &Dma<Tx>) {
    self.dma_tx_peripheral_addr_init(dma_tx);
    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    dma_tx.cselr_cs().modify(|r| {
      self.0.dma_tx_ch_select(r, dma_tx);
    });
  }

  #[inline(always)]
  fn dma_tx_peripheral_addr_init(&self, dma_tx: &Dma<Tx>) {
    unsafe { dma_tx.set_peripheral_addr(self.0.dr().to_mut_ptr() as usize) };
  }
}

#[allow(missing_docs)]
impl<T, Rx> SpiDmaRx<T, Rx> for Spi<T>
where
  T: SpiTokensDmaRx<Rx>,
  Rx: DmaTokens,
{
  #[inline(always)]
  fn dma_rx_init(&self, dma_rx: &Dma<Rx>) {
    self.dma_rx_peripheral_addr_init(dma_rx);
    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    dma_rx.cselr_cs().modify(|r| {
      self.0.dma_rx_ch_select(r, dma_rx);
    });
  }

  #[inline(always)]
  fn dma_rx_peripheral_addr_init(&self, dma_rx: &Dma<Rx>) {
    unsafe { dma_rx.set_peripheral_addr(self.0.dr().to_ptr() as usize) };
  }
}

#[allow(missing_docs)]
impl<T, Tx, Rx> SpiDmaDx<T, Tx, Rx> for Spi<T>
where
  T: SpiTokensDmaTx<Tx> + SpiTokensDmaRx<Rx>,
  Tx: DmaTokens,
  Rx: DmaTokens,
{
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  #[inline(always)]
  fn dma_dx_init(&self, dma_tx: &Dma<Tx>, dma_rx: &Dma<Rx>)
  where
    Tx: DmaTokens<Cselr = Rx::Cselr>,
  {
    self.dma_dx_peripheral_addr_init(dma_tx, dma_rx);
    dma_tx.cselr_cs().modify(|r| {
      self.0.dma_tx_ch_select(r, dma_tx);
      self.0.dma_rx_ch_select(r, dma_rx);
    });
  }

  #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6")))]
  #[inline(always)]
  fn dma_dx_init(&self, dma_tx: &Dma<Tx>, dma_rx: &Dma<Rx>) {
    self.dma_dx_peripheral_addr_init(dma_tx, dma_rx);
  }

  #[inline(always)]
  fn dma_dx_peripheral_addr_init(&self, dma_tx: &Dma<Tx>, dma_rx: &Dma<Rx>) {
    self.dma_tx_peripheral_addr_init(dma_tx);
    self.dma_rx_peripheral_addr_init(dma_rx);
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
    $name_tokens:ident,
    ($($tp:ident: $bound:path),*),
    ($((
      [$($dma_tx_attr:meta,)*],
      $dma_tx_tokens:ident,
      $irq_dma_tx:ident,
      $dma_tx_cs:expr,
      ($($dma_tx_tp:ident: $dma_tx_bound:path),*)
    ),)*),
    ($((
      [$($dma_rx_attr:meta,)*],
      $dma_rx_tokens:ident,
      $irq_dma_rx:ident,
      $dma_rx_cs:expr,
      ($($dma_rx_tp:ident: $dma_rx_bound:path),*)
    ),)*),
  ) => {
    impl<$($tp: $bound,)*> SpiTokens for $name_tokens<$($tp,)* Frt> {
      type Cr1 = $spi::Cr1<Frt>;
      type Cr2 = $spi::Cr2<Frt>;
      type Crcpr = $spi::Crcpr<Srt>;
      type Dr = $spi::Dr<Srt>;
      type Rxcrcr = $spi::Rxcrcr<Srt>;
      type Sr = $spi::Sr<Srt>;
      type SrBsy = $spi::sr::Bsy<Srt>;
      type Txcrcr = $spi::Txcrcr<Srt>;

      #[inline(always)]
      fn cr1(&self) -> &Self::Cr1 {
        &self.$spi_cr1
      }

      #[inline(always)]
      fn cr1_mut(&mut self) -> &mut Self::Cr1 {
        &mut self.$spi_cr1
      }

      #[inline(always)]
      fn cr2(&self) -> &Self::Cr2 {
        &self.$spi_cr2
      }

      #[inline(always)]
      fn cr2_mut(&mut self) -> &mut Self::Cr2 {
        &mut self.$spi_cr2
      }

      #[inline(always)]
      fn crcpr(&self) -> &Self::Crcpr {
        &self.$spi_crcpr
      }

      #[inline(always)]
      fn dr(&self) -> &Self::Dr {
        &self.$spi_dr
      }

      #[inline(always)]
      fn rxcrcr(&self) -> &Self::Rxcrcr {
        &self.$spi_rxcrcr
      }

      #[inline(always)]
      fn sr(&self) -> &Self::Sr {
        &self.$spi_sr
      }

      #[inline(always)]
      fn sr_bsy(&self) -> &Self::SrBsy {
        &self.$spi_sr.bsy
      }

      #[inline(always)]
      fn txcrcr(&self) -> &Self::Txcrcr {
        &self.$spi_txcrcr
      }
    }

    $(
      $(#[$dma_tx_attr])*
      impl<$($dma_tx_tp,)* Tx> SpiTokensDmaTx<$dma_tx_tokens<Tx, Frt>>
        for $name_tokens<$($dma_tx_tp,)* Frt>
      where
        Tx: $irq_dma_tx<Ltt>,
        $($dma_tx_tp: $dma_tx_bound,)*
      {
        #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                  feature = "stm32l4x3", feature = "stm32l4x5",
                  feature = "stm32l4x6"))]
        #[inline(always)]
        fn dma_tx_ch_select(
          &self,
          cs_val: &mut CselrVal<$dma_tx_tokens<Tx, Frt>>,
          dma: &Dma<$dma_tx_tokens<Tx, Frt>>,
        ) {
          dma.cselr_cs().write(cs_val, $dma_tx_cs);
        }
      }
    )*

    $(
      $(#[$dma_rx_attr])*
      impl<$($dma_rx_tp,)* Rx> SpiTokensDmaRx<$dma_rx_tokens<Rx, Frt>>
        for $name_tokens<$($dma_rx_tp,)* Frt>
      where
        Rx: $irq_dma_rx<Ltt>,
        $($dma_rx_tp: $dma_rx_bound,)*
      {
        #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                  feature = "stm32l4x3", feature = "stm32l4x5",
                  feature = "stm32l4x6"))]
        #[inline(always)]
        fn dma_rx_ch_select(
          &self,
          cs_val: &mut CselrVal<$dma_rx_tokens<Rx, Frt>>,
          dma: &Dma<$dma_rx_tokens<Rx, Frt>>,
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
    $doc_tokens:expr,
    $name_tokens:ident,
    $doc_irq_tokens:expr,
    $name_irq_tokens:ident,
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
      $dma_tx_tokens:ident,
      $irq_dma_tx:ident,
      $dma_tx_cs:expr
    )),*),
    ($((
      $(#[$dma_rx_attr:meta])*
      $dma_rx_tokens:ident,
      $irq_dma_rx:ident,
      $dma_rx_cs:expr
    )),*),
  ) => {
    #[doc = $doc]
    pub type $name = Spi<$name_tokens<Frt>>;

    #[doc = $doc_irq]
    pub type $name_irq<I> = Spi<$name_irq_tokens<I, Frt>>;

    #[doc = $doc_tokens]
    #[allow(missing_docs)]
    pub struct $name_tokens<Rt: RegTag> {
      pub $spi_cr1: $spi::Cr1<Rt>,
      pub $spi_cr2: $spi::Cr2<Rt>,
      pub $spi_crcpr: $spi::Crcpr<Srt>,
      pub $spi_dr: $spi::Dr<Srt>,
      pub $spi_rxcrcr: $spi::Rxcrcr<Srt>,
      pub $spi_sr: $spi::Sr<Srt>,
      pub $spi_txcrcr: $spi::Txcrcr<Srt>,
    }

    #[doc = $doc_irq_tokens]
    #[allow(missing_docs)]
    pub struct $name_irq_tokens<I: $irq_ty<Ltt>, Rt: RegTag> {
      pub $spi: I,
      pub $spi_cr1: $spi::Cr1<Rt>,
      pub $spi_cr2: $spi::Cr2<Rt>,
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
        $crate::peripherals::spi::Spi::from_tokens(
          $crate::peripherals::spi::$name_tokens {
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
        $crate::peripherals::spi::Spi::from_tokens(
          $crate::peripherals::spi::$name_irq_tokens {
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

    impl From<$name_tokens<Srt>> for $name_tokens<Frt> {
      #[inline(always)]
      fn from(tokens: $name_tokens<Srt>) -> Self {
        Self {
          $spi_cr1: tokens.$spi_cr1.into(),
          $spi_cr2: tokens.$spi_cr2.into(),
          $spi_crcpr: tokens.$spi_crcpr,
          $spi_dr: tokens.$spi_dr,
          $spi_rxcrcr: tokens.$spi_rxcrcr,
          $spi_sr: tokens.$spi_sr,
          $spi_txcrcr: tokens.$spi_txcrcr,
        }
      }
    }

    impl PeripheralTokens for $name_tokens<Frt> {
      type InputTokens = $name_tokens<Srt>;
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
      $name_tokens,
      (),
      ($(([$($dma_tx_attr,)*], $dma_tx_tokens, $irq_dma_tx, $dma_tx_cs, ()),)*),
      ($(([$($dma_rx_attr,)*], $dma_rx_tokens, $irq_dma_rx, $dma_rx_cs, ()),)*),
    }

    impl<I> From<$name_irq_tokens<I, Srt>> for $name_irq_tokens<I, Frt>
    where
      I: $irq_ty<Ltt>,
    {
      #[inline(always)]
      fn from(tokens: $name_irq_tokens<I, Srt>) -> Self {
        Self {
          $spi: tokens.$spi,
          $spi_cr1: tokens.$spi_cr1.into(),
          $spi_cr2: tokens.$spi_cr2.into(),
          $spi_crcpr: tokens.$spi_crcpr,
          $spi_dr: tokens.$spi_dr,
          $spi_rxcrcr: tokens.$spi_rxcrcr,
          $spi_sr: tokens.$spi_sr,
          $spi_txcrcr: tokens.$spi_txcrcr,
        }
      }
    }

    impl<I: $irq_ty<Ltt>> PeripheralTokens for $name_irq_tokens<I, Frt> {
      type InputTokens = $name_irq_tokens<I, Srt>;
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
      $name_irq_tokens,
      (I: $irq_ty<Ltt>),
      ($((
        [$($dma_tx_attr,)*], $dma_tx_tokens, $irq_dma_tx, $dma_tx_cs,
        (I: $irq_ty<Ltt>)
      ),)*),
      ($((
        [$($dma_rx_attr,)*], $dma_rx_tokens, $irq_dma_rx, $dma_rx_cs,
        (I: $irq_ty<Ltt>)
      ),)*),
    }

    impl<I: $irq_ty<Ltt>> SpiTokensIrq for $name_irq_tokens<I, Frt> {
      type WithoutIrq = $name_tokens<Frt>;
      type Irq = I;

      #[inline(always)]
      fn join_irq(tokens: Self::WithoutIrq, irq: Self::Irq) -> Self {
        $name_irq_tokens {
          $spi: irq,
          $spi_cr1: tokens.$spi_cr1,
          $spi_cr2: tokens.$spi_cr2,
          $spi_crcpr: tokens.$spi_crcpr,
          $spi_dr: tokens.$spi_dr,
          $spi_rxcrcr: tokens.$spi_rxcrcr,
          $spi_sr: tokens.$spi_sr,
          $spi_txcrcr: tokens.$spi_txcrcr,
        }
      }

      #[inline(always)]
      fn split_irq(self) -> (Self::WithoutIrq, Self::Irq) {
        (
          $name_tokens {
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
  "SPI1.",
  Spi1,
  peripheral_spi1,
  "SPI1 with interrupt.",
  Spi1Irq,
  peripheral_spi1_irq,
  "SPI1 tokens.",
  Spi1Tokens,
  "SPI1 with interrupt tokens.",
  Spi1IrqTokens,
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
    (Dma1Ch3Tokens, IrqDma1Ch3, 0b0001),
    (
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      Dma2Ch4Tokens, IrqDma2Ch4, 0b0100
    )
  ),
  (
    (Dma1Ch2Tokens, IrqDma1Ch2, 0b0001),
    (
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      Dma2Ch3Tokens, IrqDma2Ch3, 0b0100
    )
  ),
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
spi! {
  "SPI2.",
  Spi2,
  peripheral_spi2,
  "SPI2 with interrupt.",
  Spi2Irq,
  peripheral_spi2_irq,
  "SPI2 tokens.",
  Spi2Tokens,
  "SPI2 with interrupt tokens.",
  Spi2IrqTokens,
  IrqSpi2,
  spi2,
  spi2_cr1,
  spi2_cr2,
  spi2_crcpr,
  spi2_dr,
  spi2_rxcrcr,
  spi2_sr,
  spi2_txcrcr,
  ((Dma1Ch5Tokens, IrqDma1Ch5, 0b0001)),
  ((Dma1Ch4Tokens, IrqDma1Ch4, 0b0001)),
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
spi! {
  "SPI3.",
  Spi3,
  peripheral_spi3,
  "SPI3 with interrupt.",
  Spi3Irq,
  peripheral_spi3_irq,
  "SPI3 tokens.",
  Spi3Tokens,
  "SPI3 with interrupt tokens.",
  Spi3IrqTokens,
  IrqSpi3,
  spi3,
  spi3_cr1,
  spi3_cr2,
  spi3_crcpr,
  spi3_dr,
  spi3_rxcrcr,
  spi3_sr,
  spi3_txcrcr,
  ((Dma2Ch2Tokens, IrqDma2Ch2, 0b0011)),
  ((Dma2Ch1Tokens, IrqDma2Ch1, 0b0011)),
}
