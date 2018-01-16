//! SPI with DMA.

use drone_core::peripheral::{PeripheralDevice, PeripheralTokens};
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
use peripherals::spi::{Spi, SpiTokens};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use peripherals::spi::{Spi1Tokens, Spi2Tokens, Spi3Tokens};
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
#[allow(unused_imports)]
use thread::prelude::*;

/// Generic SPI with duplex DMA.
pub struct SpiDmaDx<T: SpiDmaDxTokens>(T);

/// Generic SPI with transmit-only DMA.
pub struct SpiDmaTx<T: SpiDmaTxTokens>(T);

/// Generic SPI with receive-only DMA.
pub struct SpiDmaRx<T: SpiDmaRxTokens>(T);

/// Generic SPI with duplex DMA tokens.
#[allow(missing_docs)]
pub trait SpiDmaDxTokens: PeripheralTokens<InputTokens = Self> {
  type SpiTokens: SpiTokens;
  type DmaTxTokens: DmaTokens;
  type DmaRxTokens: DmaTokens;

  fn from_plain(plain: SpiDmaDxPlainTokens<Self>) -> Self;
  fn into_plain(self) -> SpiDmaDxPlainTokens<Self>;

  fn spi(&self) -> &Spi<Self::SpiTokens>;
  fn dma_tx(&self) -> &Dma<Self::DmaTxTokens>;
  fn dma_rx(&self) -> &Dma<Self::DmaRxTokens>;

  fn ch_select(&self);
}

/// Generic SPI with transmit-only DMA tokens.
#[allow(missing_docs)]
pub trait SpiDmaTxTokens: PeripheralTokens<InputTokens = Self> {
  type SpiTokens: SpiTokens;
  type DmaTxTokens: DmaTokens;

  fn from_plain(plain: SpiDmaTxPlainTokens<Self>) -> Self;
  fn into_plain(self) -> SpiDmaTxPlainTokens<Self>;

  fn spi(&self) -> &Spi<Self::SpiTokens>;
  fn dma_tx(&self) -> &Dma<Self::DmaTxTokens>;

  fn ch_select(&self);
}

/// Generic SPI with receive-only DMA tokens.
#[allow(missing_docs)]
pub trait SpiDmaRxTokens: PeripheralTokens<InputTokens = Self> {
  type SpiTokens: SpiTokens;
  type DmaRxTokens: DmaTokens;

  fn from_plain(plain: SpiDmaRxPlainTokens<Self>) -> Self;
  fn into_plain(self) -> SpiDmaRxPlainTokens<Self>;

  fn spi(&self) -> &Spi<Self::SpiTokens>;
  fn dma_rx(&self) -> &Dma<Self::DmaRxTokens>;

  fn ch_select(&self);
}

type SpiDmaDxPlainTokens<T> = (
  Spi<<T as SpiDmaDxTokens>::SpiTokens>,
  Dma<<T as SpiDmaDxTokens>::DmaTxTokens>,
  Dma<<T as SpiDmaDxTokens>::DmaRxTokens>,
);

type SpiDmaTxPlainTokens<T> = (
  Spi<<T as SpiDmaTxTokens>::SpiTokens>,
  Dma<<T as SpiDmaTxTokens>::DmaTxTokens>,
);

type SpiDmaRxPlainTokens<T> = (
  Spi<<T as SpiDmaRxTokens>::SpiTokens>,
  Dma<<T as SpiDmaRxTokens>::DmaRxTokens>,
);

impl<T: SpiDmaDxTokens> PeripheralDevice<T> for SpiDmaDx<T> {
  #[inline(always)]
  fn from_tokens(tokens: T::InputTokens) -> Self {
    SpiDmaDx(tokens)
  }

  #[inline(always)]
  fn into_tokens(self) -> T {
    self.0
  }
}

impl<T: SpiDmaTxTokens> PeripheralDevice<T> for SpiDmaTx<T> {
  #[inline(always)]
  fn from_tokens(tokens: T::InputTokens) -> Self {
    SpiDmaTx(tokens)
  }

  #[inline(always)]
  fn into_tokens(self) -> T {
    self.0
  }
}

impl<T: SpiDmaRxTokens> PeripheralDevice<T> for SpiDmaRx<T> {
  #[inline(always)]
  fn from_tokens(tokens: T::InputTokens) -> Self {
    SpiDmaRx(tokens)
  }

  #[inline(always)]
  fn into_tokens(self) -> T {
    self.0
  }
}

impl<T: SpiDmaDxTokens> SpiDmaDx<T> {
  /// Returns SPI.
  #[inline(always)]
  pub fn spi(&self) -> &Spi<T::SpiTokens> {
    self.0.spi()
  }

  /// Returns DMA transmitting channel.
  #[inline(always)]
  pub fn dma_tx(&self) -> &Dma<T::DmaTxTokens> {
    self.0.dma_tx()
  }

  /// Returns DMA receiving channel.
  #[inline(always)]
  pub fn dma_rx(&self) -> &Dma<T::DmaRxTokens> {
    self.0.dma_rx()
  }

  /// Initializes DMA to use with SPI.
  #[inline(always)]
  pub fn dma_init(&self) {
    let dr_ptr = self.0.spi().dr().to_mut_ptr();
    self.0.dma_rx().set_peripheral_address(dr_ptr as usize);
    self.0.dma_tx().set_peripheral_address(dr_ptr as usize);
    self.0.ch_select();
  }

  /// Returns a future, which resolves on both DMA transmit and receive
  /// complete.
  pub fn transfer_complete(
    self,
    cr1: <<T::SpiTokens as SpiTokens>::Cr1 as Reg<Frt>>::Val,
    cr2: <<T::SpiTokens as SpiTokens>::Cr2 as Reg<Frt>>::Val,
  ) -> impl Future<Item = Self, Error = Self> {
    let (spi, dma_tx, dma_rx) = self.0.into_plain();
    spi.spe_after(cr1, move |spi| {
      spi.txdmaen_after(cr2, move |spi| {
        let dma_tx = dma_tx.transfer_complete();
        let dma_rx = dma_rx.transfer_complete();
        AsyncFuture::new(move || {
          let dma_rx = await!(dma_rx);
          let dma_tx = await!(dma_tx);
          match (dma_tx, dma_rx) {
            (Ok(dma_tx), Ok(dma_rx)) => {
              Ok(Self::from_tokens(T::from_plain((spi, dma_tx, dma_rx))))
            }
            (Ok(dma_tx), Err(dma_rx))
            | (Err(dma_tx), Ok(dma_rx))
            | (Err(dma_tx), Err(dma_rx)) => {
              Err(Self::from_tokens(T::from_plain((spi, dma_tx, dma_rx))))
            }
          }
        })
      })
    })
  }
}

impl<T: SpiDmaTxTokens> SpiDmaTx<T> {
  /// Returns SPI.
  #[inline(always)]
  pub fn spi(&self) -> &Spi<T::SpiTokens> {
    self.0.spi()
  }

  /// Returns DMA transmitting channel.
  #[inline(always)]
  pub fn dma_tx(&self) -> &Dma<T::DmaTxTokens> {
    self.0.dma_tx()
  }

  /// Initializes DMA to use with SPI.
  #[inline(always)]
  pub fn dma_init(&self) {
    let dr_ptr = self.0.spi().dr().to_mut_ptr();
    self.0.dma_tx().set_peripheral_address(dr_ptr as usize);
    self.0.ch_select();
  }

  /// Returns a future, which resolves on DMA transmit complete.
  pub fn transfer_complete(
    self,
    cr1: <<T::SpiTokens as SpiTokens>::Cr1 as Reg<Frt>>::Val,
    cr2: <<T::SpiTokens as SpiTokens>::Cr2 as Reg<Frt>>::Val,
  ) -> impl Future<Item = Self, Error = Self> {
    let (spi, dma_tx) = self.0.into_plain();
    spi.spe_after(cr1, move |spi| {
      spi.txdmaen_after(cr2, move |spi| {
        let dma_tx = dma_tx.transfer_complete();
        AsyncFuture::new(move || match await!(dma_tx) {
          Ok(dma_tx) => Ok(Self::from_tokens(T::from_plain((spi, dma_tx)))),
          Err(dma_tx) => Err(Self::from_tokens(T::from_plain((spi, dma_tx)))),
        })
      })
    })
  }
}

impl<T: SpiDmaRxTokens> SpiDmaRx<T> {
  /// Returns SPI.
  #[inline(always)]
  pub fn spi(&self) -> &Spi<T::SpiTokens> {
    self.0.spi()
  }

  /// Returns DMA receiving channel.
  #[inline(always)]
  pub fn dma_rx(&self) -> &Dma<T::DmaRxTokens> {
    self.0.dma_rx()
  }

  /// Initializes DMA to use with SPI.
  #[inline(always)]
  pub fn dma_init(&self) {
    let dr_ptr = self.0.spi().dr().to_ptr();
    self.0.dma_rx().set_peripheral_address(dr_ptr as usize);
    self.0.ch_select();
  }

  /// Returns a future, which resolves on DMA receive complete.
  pub fn transfer_complete(
    self,
    cr1: <<T::SpiTokens as SpiTokens>::Cr1 as Reg<Frt>>::Val,
  ) -> impl Future<Item = Self, Error = Self> {
    let (spi, dma_rx) = self.0.into_plain();
    spi.spe_after(cr1, move |spi| {
      let dma_rx = dma_rx.transfer_complete();
      AsyncFuture::new(move || match await!(dma_rx) {
        Ok(dma_rx) => Ok(Self::from_tokens(T::from_plain((spi, dma_rx)))),
        Err(dma_rx) => Err(Self::from_tokens(T::from_plain((spi, dma_rx)))),
      })
    })
  }
}

#[allow(unused_macros)]
macro_rules! spi_dma {
  (
    $doc_dx:expr,
    $name_dx:ident,
    $name_dx_macro:ident,
    $doc_tx:expr,
    $name_tx:ident,
    $name_tx_macro:ident,
    $doc_rx:expr,
    $name_rx:ident,
    $name_rx_macro:ident,
    $doc_dx_tokens:expr,
    $name_dx_tokens:ident,
    $doc_tx_tokens:expr,
    $name_tx_tokens:ident,
    $doc_rx_tokens:expr,
    $name_rx_tokens:ident,
    $irq_spi:ident,
    $irq_dma_tx:ident,
    $irq_dma_rx:ident,
    $spi_tokens:ident,
    $dma_tx_tokens:ident,
    $dma_rx_tokens:ident,
    $spi_macro:ident,
    $dma_tx_macro:ident,
    $dma_rx_macro:ident,
    $spi:ident,
    $dma_tx:ident,
    $dma_rx:ident,
    $dma_tx_cs:expr,
    $dma_rx_cs:expr,
  ) => {
    #[doc = $doc_dx]
    pub type $name_dx<Tx, Rx> = SpiDmaDx<$name_dx_tokens<Tx, Rx>>;

    #[doc = $doc_tx]
    pub type $name_tx<Tx> = SpiDmaTx<$name_tx_tokens<Tx>>;

    #[doc = $doc_rx]
    pub type $name_rx<Rx> = SpiDmaTx<$name_rx_tokens<Rx>>;

    #[doc = $doc_dx_tokens]
    #[allow(missing_docs)]
    pub struct $name_dx_tokens<Tx: $irq_dma_tx<Ltt>, Rx: $irq_dma_rx<Ltt>> {
      pub $spi: Spi<$spi_tokens<Frt>>,
      pub $dma_tx: Dma<$dma_tx_tokens<Tx>>,
      pub $dma_rx: Dma<$dma_rx_tokens<Rx>>,
    }

    #[doc = $doc_tx_tokens]
    #[allow(missing_docs)]
    pub struct $name_tx_tokens<Tx: $irq_dma_tx<Ltt>> {
      pub $spi: Spi<$spi_tokens<Frt>>,
      pub $dma_tx: Dma<$dma_tx_tokens<Tx>>,
    }

    #[doc = $doc_rx_tokens]
    #[allow(missing_docs)]
    pub struct $name_rx_tokens<Rx: $irq_dma_rx<Ltt>> {
      pub $spi: Spi<$spi_tokens<Frt>>,
      pub $dma_rx: Dma<$dma_rx_tokens<Rx>>,
    }

    /// Creates a new `SpiDmaDx`.
    #[macro_export]
    macro_rules! $name_dx_macro {
      ($regs:ident, $thrd:ident) => {
        $crate::peripherals::spi_dma::SpiDmaDx::from_tokens(
          $crate::peripherals::spi_dma::$name_dx_tokens {
            $spi: $spi_macro!($regs),
            $dma_tx: $dma_tx_macro!($regs, $thrd),
            $dma_rx: $dma_rx_macro!($regs, $thrd),
          }
        )
      }
    }

    /// Creates a new `SpiDmaTx`.
    #[macro_export]
    macro_rules! $name_tx_macro {
      ($regs:ident, $thrd:ident) => {
        $crate::peripherals::spi_dma::SpiDmaTx::from_tokens(
          $crate::peripherals::spi_dma::$name_tx_tokens {
            $spi: $spi_macro!($regs),
            $dma_tx: $dma_tx_macro!($regs, $thrd),
          }
        )
      }
    }

    /// Creates a new `SpiDmaRx`.
    #[macro_export]
    macro_rules! $name_rx_macro {
      ($regs:ident, $thrd:ident) => {
        $crate::peripherals::spi_dma::SpiDmaRx::from_tokens(
          $crate::peripherals::spi_dma::$name_rx_tokens {
            $spi: $spi_macro!($regs),
            $dma_rx: $dma_rx_macro!($regs, $thrd),
          }
        )
      }
    }

    impl<Tx, Rx> PeripheralTokens for $name_dx_tokens<Tx, Rx>
    where
      Tx: $irq_dma_tx<Ltt>,
      Rx: $irq_dma_rx<Ltt>,
    {
      // FIXME https://github.com/rust-lang/rust/issues/47385
      type InputTokens = Self;
    }

    impl<Tx, Rx> SpiDmaDxTokens for $name_dx_tokens<Tx, Rx>
    where
      Tx: $irq_dma_tx<Ltt>,
      Rx: $irq_dma_rx<Ltt>,
    {
      type SpiTokens = $spi_tokens<Frt>;
      type DmaTxTokens = $dma_tx_tokens<Tx>;
      type DmaRxTokens = $dma_rx_tokens<Rx>;

      #[inline(always)]
      fn from_plain(plain: SpiDmaDxPlainTokens<Self>) -> Self {
        Self {
          $spi: plain.0,
          $dma_tx: plain.1,
          $dma_rx: plain.2,
        }
      }

      #[inline(always)]
      fn into_plain(self) -> SpiDmaDxPlainTokens<Self> {
        (self.$spi, self.$dma_tx, self.$dma_rx)
      }

      #[inline(always)]
      fn spi(&self) -> &Spi<Self::SpiTokens> {
        &self.$spi
      }

      #[inline(always)]
      fn dma_tx(&self) -> &Dma<Self::DmaTxTokens> {
        &self.$dma_tx
      }

      #[inline(always)]
      fn dma_rx(&self) -> &Dma<Self::DmaRxTokens> {
        &self.$dma_rx
      }

      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      #[inline(always)]
      fn ch_select(&self) {
        self.$dma_tx.cselr_cs().modify(|r| {
          self.$dma_tx.cselr_cs().write(r, $dma_tx_cs);
          self.$dma_rx.cselr_cs().write(r, $dma_rx_cs);
        });
      }

      #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                    feature = "stm32l4x3", feature = "stm32l4x5",
                    feature = "stm32l4x6")))]
      #[inline(always)]
      fn ch_select(&self) {}
    }

    impl<Tx> PeripheralTokens for $name_tx_tokens<Tx>
    where
      Tx: $irq_dma_tx<Ltt>,
    {
      // FIXME https://github.com/rust-lang/rust/issues/47385
      type InputTokens = Self;
    }

    impl<Tx> SpiDmaTxTokens for $name_tx_tokens<Tx>
    where
      Tx: $irq_dma_tx<Ltt>,
    {
      type SpiTokens = $spi_tokens<Frt>;
      type DmaTxTokens = $dma_tx_tokens<Tx>;

      #[inline(always)]
      fn from_plain(plain: SpiDmaTxPlainTokens<Self>) -> Self {
        Self {
          $spi: plain.0,
          $dma_tx: plain.1,
        }
      }

      #[inline(always)]
      fn into_plain(self) -> SpiDmaTxPlainTokens<Self> {
        (self.$spi, self.$dma_tx)
      }

      #[inline(always)]
      fn spi(&self) -> &Spi<Self::SpiTokens> {
        &self.$spi
      }

      #[inline(always)]
      fn dma_tx(&self) -> &Dma<Self::DmaTxTokens> {
        &self.$dma_tx
      }

      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      #[inline(always)]
      fn ch_select(&self) {
        self.$dma_tx.cselr_cs().write_bits($dma_tx_cs);
      }

      #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                    feature = "stm32l4x3", feature = "stm32l4x5",
                    feature = "stm32l4x6")))]
      #[inline(always)]
      fn ch_select(&self) {}
    }

    impl<Rx> PeripheralTokens for $name_rx_tokens<Rx>
    where
      Rx: $irq_dma_rx<Ltt>,
    {
      // FIXME https://github.com/rust-lang/rust/issues/47385
      type InputTokens = Self;
    }

    impl<Rx> SpiDmaRxTokens for $name_rx_tokens<Rx>
    where
      Rx: $irq_dma_rx<Ltt>,
    {
      type SpiTokens = $spi_tokens<Frt>;
      type DmaRxTokens = $dma_rx_tokens<Rx>;

      #[inline(always)]
      fn from_plain(plain: SpiDmaRxPlainTokens<Self>) -> Self {
        Self {
          $spi: plain.0,
          $dma_rx: plain.1,
        }
      }

      #[inline(always)]
      fn into_plain(self) -> SpiDmaRxPlainTokens<Self> {
        (self.$spi, self.$dma_rx)
      }

      #[inline(always)]
      fn spi(&self) -> &Spi<Self::SpiTokens> {
        &self.$spi
      }

      #[inline(always)]
      fn dma_rx(&self) -> &Dma<Self::DmaRxTokens> {
        &self.$dma_rx
      }

      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      #[inline(always)]
      fn ch_select(&self) {
        self.$dma_rx.cselr_cs().write_bits($dma_rx_cs);
      }

      #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                    feature = "stm32l4x3", feature = "stm32l4x5",
                    feature = "stm32l4x6")))]
      #[inline(always)]
      fn ch_select(&self) {}
    }
  }
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
spi_dma! {
  "SPI1 with duplex DMA1",
  Spi1Dma1Dx,
  peripheral_spi1_dma1_dx,
  "SPI1 with transmit-only DMA1",
  Spi1Dma1Tx,
  peripheral_spi1_dma1_tx,
  "SPI1 with receive-only DMA1",
  Spi1Dma1Rx,
  peripheral_spi1_dma1_rx,
  "SPI1 with duplex DMA1 tokens",
  Spi1Dma1DxTokens,
  "SPI1 with transmit-only DMA1 tokens",
  Spi1Dma1TxTokens,
  "SPI1 with receive-only DMA1 tokens",
  Spi1Dma1RxTokens,
  IrqSpi1,
  IrqDma1Ch3,
  IrqDma1Ch2,
  Spi1Tokens,
  Dma1Ch3Tokens,
  Dma1Ch2Tokens,
  peripheral_spi1,
  peripheral_dma1_ch3,
  peripheral_dma1_ch2,
  spi1,
  dma1_ch3,
  dma1_ch2,
  0b0001,
  0b0001,
}

#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
spi_dma! {
  "SPI1 with duplex DMA2",
  Spi1Dma2Dx,
  peripheral_spi1_dma2_dx,
  "SPI1 with transmit-only DMA2",
  Spi1Dma2Tx,
  peripheral_spi1_dma2_tx,
  "SPI1 with receive-only DMA2",
  Spi1Dma2Rx,
  peripheral_spi1_dma2_rx,
  "SPI1 with duplex DMA2 tokens",
  Spi1Dma2DxTokens,
  "SPI1 with transmit-only DMA2 tokens",
  Spi1Dma2TxTokens,
  "SPI1 with receive-only DMA2 tokens",
  Spi1Dma2RxTokens,
  IrqSpi1,
  IrqDma2Ch4,
  IrqDma2Ch3,
  Spi1Tokens,
  Dma2Ch4Tokens,
  Dma2Ch3Tokens,
  peripheral_spi1,
  peripheral_dma2_ch4,
  peripheral_dma2_ch3,
  spi1,
  dma2_ch4,
  dma2_ch3,
  0b0100,
  0b0100,
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
spi_dma! {
  "SPI2 with duplex DMA1",
  Spi2Dma1Dx,
  peripheral_spi2_dma1_dx,
  "SPI2 with transmit-only DMA1",
  Spi2Dma1Tx,
  peripheral_spi2_dma1_tx,
  "SPI2 with receive-only DMA1",
  Spi2Dma1Rx,
  peripheral_spi2_dma1_rx,
  "SPI2 with duplex DMA1 tokens",
  Spi2Dma1DxTokens,
  "SPI2 with transmit-only DMA1 tokens",
  Spi2Dma1TxTokens,
  "SPI2 with receive-only DMA1 tokens",
  Spi2Dma1RxTokens,
  IrqSpi2,
  IrqDma1Ch5,
  IrqDma1Ch4,
  Spi2Tokens,
  Dma1Ch5Tokens,
  Dma1Ch4Tokens,
  peripheral_spi2,
  peripheral_dma1_ch5,
  peripheral_dma1_ch4,
  spi2,
  dma1_ch5,
  dma1_ch4,
  0b0001,
  0b0001,
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
spi_dma! {
  "SPI3 with duplex DMA2",
  Spi3Dma2Dx,
  peripheral_spi3_dma2_dx,
  "SPI3 with transmit-only DMA2",
  Spi3Dma2Tx,
  peripheral_spi3_dma2_tx,
  "SPI3 with receive-only DMA2",
  Spi3Dma2Rx,
  peripheral_spi3_dma2_rx,
  "SPI3 with duplex DMA2 tokens",
  Spi3Dma2DxTokens,
  "SPI3 with transmit-only DMA2 tokens",
  Spi3Dma2TxTokens,
  "SPI3 with receive-only DMA2 tokens",
  Spi3Dma2RxTokens,
  IrqSpi3,
  IrqDma2Ch2,
  IrqDma2Ch1,
  Spi3Tokens,
  Dma2Ch2Tokens,
  Dma2Ch1Tokens,
  peripheral_spi3,
  peripheral_dma2_ch2,
  peripheral_dma2_ch1,
  spi3,
  dma2_ch2,
  dma2_ch1,
  0b0011,
  0b0011,
}
