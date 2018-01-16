//! Serial peripheral interface.

#[allow(unused_imports)]
use core::ptr::{read_volatile, write_volatile};
use drone_core::peripheral::{PeripheralDevice, PeripheralTokens};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use reg::{spi1, spi2, spi3};
use reg::prelude::*;
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use thread::irq::{IrqSpi1, IrqSpi2, IrqSpi3};
use thread::prelude::*;

/// Generic SPI.
pub struct Spi<T: SpiTokens>(T);

/// Generic SPI tokens.
#[allow(missing_docs)]
pub trait SpiTokens: PeripheralTokens {
  type Cr1: for<'a> RwRegSharedRef<'a, Frt> + RegBitBand<Frt> + RegFork;
  type Cr1Spe: RegField<Frt, Reg = Self::Cr1>
    + WRwRegFieldBitShared<Frt>
    + RRegFieldBitBand<Frt>
    + WRegFieldBitBand<Frt>
    + RegFork;
  type Cr2: for<'a> RwRegSharedRef<'a, Frt> + RegBitBand<Frt> + RegFork;
  type Cr2Txdmaen: RegField<Frt, Reg = Self::Cr2>
    + WRwRegFieldBitShared<Frt>
    + RRegFieldBitBand<Frt>
    + WRegFieldBitBand<Frt>
    + RegFork;
  type Crcpr: for<'a> RwRegSharedRef<'a, Srt>;
  type Dr: for<'a> RwRegSharedRef<'a, Srt>;
  type Rxcrcr: RoReg<Srt>;
  type Sr: for<'a> RwRegSharedRef<'a, Srt> + RegBitBand<Srt>;
  type SrBsy: RegField<Srt, Reg = Self::Sr> + RRegFieldBitBand<Srt>;
  type Txcrcr: RoReg<Srt>;

  fn cr1(&self) -> &Self::Cr1;
  fn cr1_mut(&mut self) -> &mut Self::Cr1;
  fn cr1_spe_mut(&mut self) -> &mut Self::Cr1Spe;
  fn cr2(&self) -> &Self::Cr2;
  fn cr2_mut(&mut self) -> &mut Self::Cr2;
  fn cr2_txdmaen_mut(&mut self) -> &mut Self::Cr2Txdmaen;
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
  type Irq: IrqToken<Ltt>;

  fn irq(&self) -> Self::Irq;
}

impl<T: SpiTokens> PeripheralDevice<T> for Spi<T> {
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

  /// Moves `self` into `f` while `SPE` is cleared, and then sets `SPE`.
  pub fn spe_after<F, R>(
    mut self,
    mut cr1_val: <T::Cr1 as Reg<Frt>>::Val,
    f: F,
  ) -> R
  where
    F: FnOnce(Self) -> R,
  {
    let cr1 = self.0.cr1_mut().fork();
    let cr1_spe = self.0.cr1_spe_mut().fork();
    cr1_spe.clear(&mut cr1_val);
    cr1.store_val(cr1_val);
    let result = f(self);
    cr1_spe.set(&mut cr1_val);
    cr1.store_val(cr1_val);
    result
  }

  /// Moves `self` into `f` while `TXDMAEN` is cleared, and then sets `TXDMAEN`.
  pub fn txdmaen_after<F, R>(
    mut self,
    mut cr2_val: <T::Cr2 as Reg<Frt>>::Val,
    f: F,
  ) -> R
  where
    F: FnOnce(Self) -> R,
  {
    let cr2 = self.0.cr2_mut().fork();
    let cr2_txdmaen = self.0.cr2_txdmaen_mut().fork();
    cr2_txdmaen.clear(&mut cr2_val);
    cr2.store_val(cr2_val);
    let result = f(self);
    cr2_txdmaen.set(&mut cr2_val);
    cr2.store_val(cr2_val);
    result
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
  ) => {
    type Cr1 = $spi::Cr1<Frt>;
    type Cr1Spe = $spi::cr1::Spe<Frt>;
    type Cr2 = $spi::Cr2<Frt>;
    type Cr2Txdmaen = $spi::cr2::Txdmaen<Frt>;
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
    fn cr1_spe_mut(&mut self) -> &mut Self::Cr1Spe {
      &mut self.$spi_cr1.spe
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
    fn cr2_txdmaen_mut(&mut self) -> &mut Self::Cr2Txdmaen {
      &mut self.$spi_cr2.txdmaen
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
        $crate::peripherals::spi::SpiIrq::from_tokens(
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

    impl SpiTokens for $name_tokens<Frt> {
      spi_shared! {
        $spi,
        $spi_cr1,
        $spi_cr2,
        $spi_crcpr,
        $spi_dr,
        $spi_rxcrcr,
        $spi_sr,
        $spi_txcrcr,
      }
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

    impl<I: $irq_ty<Ltt>> SpiTokens for $name_irq_tokens<I, Frt> {
      spi_shared! {
        $spi,
        $spi_cr1,
        $spi_cr2,
        $spi_crcpr,
        $spi_dr,
        $spi_rxcrcr,
        $spi_sr,
        $spi_txcrcr,
      }
    }

    impl<I: $irq_ty<Ltt>> SpiTokensIrq for $name_irq_tokens<I, Frt> {
      type Irq = I;

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
}
