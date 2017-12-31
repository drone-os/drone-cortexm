//! Serial peripheral interface.

#[allow(unused_imports)]
use core::ptr::{read_volatile, write_volatile};
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
use thread::interrupts::{IrqSpi1, IrqSpi2, IrqSpi3};
use thread::prelude::*;

/// Generic SPI.
#[allow(missing_docs)]
pub trait Spi: Sized
where
  Self: Sized,
  Self::Tokens: From<Self>,
{
  /// Generic SPI input tokens.
  type InputTokens;

  /// Generic SPI tokens.
  type Tokens;

  type Cr1: for<'a> WRegShared<'a, Ftt>;
  type Cr2: for<'a> WRegShared<'a, Ftt>;
  type Crcpr: Reg<Stt>;
  type Dr: Reg<Stt>;
  type Rxcrcr: Reg<Stt>;
  type Sr: Reg<Stt>;
  type Txcrcr: Reg<Stt>;

  /// Creates a new `Spi` driver from provided `tokens`.
  fn new(tokens: Self::InputTokens) -> Self;

  fn cr1(&self) -> &Self::Cr1;
  fn cr2(&self) -> &Self::Cr2;
  fn crcpr(&self) -> &Self::Crcpr;
  fn dr(&self) -> &Self::Dr;
  fn rxcrcr(&self) -> &Self::Rxcrcr;
  fn sr(&self) -> &Self::Sr;
  fn txcrcr(&self) -> &Self::Txcrcr;

  /// Writes `u8` value to the data register.
  fn send_byte(&self, value: u8);

  /// Writes `u16` value to the data register.
  fn send_hword(&self, value: u16);

  /// Reads `u8` value from the data register.
  fn recv_byte(&self) -> u8;

  /// Reads `u16` value from the data register.
  fn recv_hword(&self) -> u16;

  /// Waits while SPI is busy in communication or Tx buffer is not empty.
  fn busy_wait(&self);

  /// Moves `self` into `f` while `SPE` is cleared, and then sets `SPE`.
  fn spe_after<F, R>(self, cr1_val: <Self::Cr1 as Reg<Ftt>>::Val, f: F) -> R
  where
    F: FnOnce(Self) -> R;

  /// Moves `self` into `f` while `TXDMAEN` is cleared, and then sets `TXDMAEN`.
  fn txdmaen_after<F, R>(
    self,
    cr2_val: <Self::Cr2 as Reg<Ftt>>::Val,
    f: F,
  ) -> R
  where
    F: FnOnce(Self) -> R;
}

/// Generic interrupt-driven SPI.
#[allow(missing_docs)]
pub trait SpiIrq<T: Thread, I: ThreadNumber>: Spi {
  fn irq(&self) -> ThreadToken<T, I>;
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
    type Cr1 = $spi::Cr1<Ftt>;
    type Cr2 = $spi::Cr2<Ftt>;
    type Crcpr = $spi::Crcpr<Stt>;
    type Dr = $spi::Dr<Stt>;
    type Rxcrcr = $spi::Rxcrcr<Stt>;
    type Sr = $spi::Sr<Stt>;
    type Txcrcr = $spi::Txcrcr<Stt>;

    #[inline(always)]
    fn cr1(&self) -> &Self::Cr1 {
      &self.tokens.$spi_cr1
    }

    #[inline(always)]
    fn cr2(&self) -> &Self::Cr2 {
      &self.tokens.$spi_cr2
    }

    #[inline(always)]
    fn crcpr(&self) -> &Self::Crcpr {
      &self.tokens.$spi_crcpr
    }

    #[inline(always)]
    fn dr(&self) -> &Self::Dr {
      &self.tokens.$spi_dr
    }

    #[inline(always)]
    fn rxcrcr(&self) -> &Self::Rxcrcr {
      &self.tokens.$spi_rxcrcr
    }

    #[inline(always)]
    fn sr(&self) -> &Self::Sr {
      &self.tokens.$spi_sr
    }

    #[inline(always)]
    fn txcrcr(&self) -> &Self::Txcrcr {
      &self.tokens.$spi_txcrcr
    }

    #[inline(always)]
    fn send_byte(&self, value: u8) {
      unsafe {
        write_volatile(self.tokens.$spi_dr.to_mut_ptr() as *mut _, value);
      }
    }

    #[inline(always)]
    fn send_hword(&self, value: u16) {
      unsafe {
        write_volatile(self.tokens.$spi_dr.to_mut_ptr() as *mut _, value);
      }
    }

    #[inline(always)]
    fn recv_byte(&self) -> u8 {
      unsafe { read_volatile(self.tokens.$spi_dr.to_ptr() as *const _) }
    }

    #[inline(always)]
    fn recv_hword(&self) -> u16 {
      unsafe { read_volatile(self.tokens.$spi_dr.to_ptr() as *const _) }
    }

    #[inline(always)]
    fn busy_wait(&self) {
      while self.sr().bsy.read_bit_band() {}
    }

    #[inline]
    fn spe_after<F, R>(
      mut self,
      mut cr1_val: <Self::Cr1 as Reg<Ftt>>::Val,
      f: F,
    ) -> R
    where
      F: FnOnce(Self) -> R,
    {
      let cr1 = self.tokens.$spi_cr1.fork();
      let cr1_spe = self.tokens.$spi_cr1.spe.fork();
      cr1_spe.clear(&mut cr1_val);
      cr1.store_val(cr1_val);
      let result = f(self);
      cr1_spe.set(&mut cr1_val);
      cr1.store_val(cr1_val);
      result
    }

    #[inline]
    fn txdmaen_after<F, R>(
      mut self,
      mut cr2_val: <Self::Cr2 as Reg<Ftt>>::Val,
      f: F,
    ) -> R
    where
      F: FnOnce(Self) -> R,
    {
      let cr2 = self.tokens.$spi_cr2.fork();
      let cr2_txdmaen = self.tokens.$spi_cr2.txdmaen.fork();
      cr2_txdmaen.clear(&mut cr2_val);
      cr2.store_val(cr2_val);
      let result = f(self);
      cr2_txdmaen.set(&mut cr2_val);
      cr2.store_val(cr2_val);
      result
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
    pub struct $name {
      tokens: $name_tokens<Ftt>,
    }

    #[doc = $doc_irq]
    pub struct $name_irq<T: Thread, I: $irq_ty> {
      tokens: $name_irq_tokens<T, I, Ftt>,
    }

    #[doc = $doc_tokens]
    #[allow(missing_docs)]
    pub struct $name_tokens<R: RegTag> {
      pub $spi_cr1: $spi::Cr1<R>,
      pub $spi_cr2: $spi::Cr2<R>,
      pub $spi_crcpr: $spi::Crcpr<Stt>,
      pub $spi_dr: $spi::Dr<Stt>,
      pub $spi_rxcrcr: $spi::Rxcrcr<Stt>,
      pub $spi_sr: $spi::Sr<Stt>,
      pub $spi_txcrcr: $spi::Txcrcr<Stt>,
    }

    #[doc = $doc_irq_tokens]
    #[allow(missing_docs)]
    pub struct $name_irq_tokens<T: Thread, I: $irq_ty, R: RegTag> {
      pub $spi: ThreadToken<T, I>,
      pub $spi_cr1: $spi::Cr1<R>,
      pub $spi_cr2: $spi::Cr2<R>,
      pub $spi_crcpr: $spi::Crcpr<Stt>,
      pub $spi_dr: $spi::Dr<Stt>,
      pub $spi_rxcrcr: $spi::Rxcrcr<Stt>,
      pub $spi_sr: $spi::Sr<Stt>,
      pub $spi_txcrcr: $spi::Txcrcr<Stt>,
    }

    /// Creates a new `Spi` driver from tokens.
    #[macro_export]
    macro_rules! $name_macro {
      ($regs:ident) => {
        $crate::peripherals::spi::Spi::new(
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

    /// Creates a new `SpiIrq` driver from tokens.
    #[macro_export]
    macro_rules! $name_irq_macro {
      ($thrd:ident, $regs:ident) => {
        $crate::peripherals::spi::SpiIrq::new(
          $crate::peripherals::spi::$name_irq_tokens {
            $spi: $thrd.$spi,
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

    impl From<$name> for $name_tokens<Ftt> {
      #[inline(always)]
      fn from(spi: $name) -> Self {
        spi.tokens
      }
    }

    impl Spi for $name {
      type InputTokens = $name_tokens<Stt>;
      type Tokens = $name_tokens<Ftt>;

      #[inline(always)]
      fn new(tokens: Self::InputTokens) -> Self {
        Self {
          tokens: Self::Tokens {
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

    impl<T, I> From<$name_irq<T, I>> for $name_irq_tokens<T, I, Ftt>
    where
      T: Thread,
      I: $irq_ty,
    {
      #[inline(always)]
      fn from(spi_irq: $name_irq<T, I>) -> Self {
        spi_irq.tokens
      }
    }

    impl<T: Thread, I: $irq_ty> Spi for $name_irq<T, I> {
      type InputTokens = $name_irq_tokens<T, I, Stt>;
      type Tokens = $name_irq_tokens<T, I, Ftt>;

      #[inline(always)]
      fn new(tokens: Self::InputTokens) -> Self {
        Self {
          tokens: Self::Tokens {
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

    impl<T: Thread, I: $irq_ty> SpiIrq<T, I> for $name_irq<T, I> {
      #[inline(always)]
      fn irq(&self) -> ThreadToken<T, I> {
        self.tokens.$spi
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
