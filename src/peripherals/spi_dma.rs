//! SPI with DMA.

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use peripherals::dma::{Dma1Ch2, Dma1Ch3, Dma1Ch4, Dma1Ch5, Dma2Ch1, Dma2Ch2};
#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
use peripherals::dma::{Dma2Ch3, Dma2Ch4};
use peripherals::dma::Dma;
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use peripherals::spi::{Spi1, Spi2, Spi3};
use peripherals::spi::Spi;
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
use thread::prelude::*;

/// Generic SPI with duplex DMA.
pub trait SpiDmaDx<T, IrqDmaTx, IrqDmaRx>
where
  Self: Sized + Send + Sync + 'static,
  Self::Tokens: From<Self>,
  T: Thread,
  IrqDmaTx: ThreadNumber,
  IrqDmaRx: ThreadNumber,
{
  /// Generic SPI with duplex DMA tokens.
  type Tokens;

  /// SPI.
  type Spi: Spi;

  /// DMA transmitting channel.
  type DmaTx: Dma<T, IrqDmaTx>;

  /// DMA receiving channel.
  type DmaRx: Dma<T, IrqDmaRx>;

  /// Creates a new `SpiDmaDx` driver from provided `tokens`.
  fn new(tokens: Self::Tokens) -> Self;

  /// Initializes DMA to use with SPI.
  fn dma_init(&self);

  /// Returns SPI.
  fn spi(&self) -> &Self::Spi;

  /// Returns DMA transmitting channel.
  fn dma_tx(&self) -> &Self::DmaTx;

  /// Returns DMA receiving channel.
  fn dma_rx(&self) -> &Self::DmaRx;

  /// Returns a future, which resolves on both DMA transmit and receive
  /// complete.
  fn transfer_complete(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Ftt>>::Val,
    cr2: <<Self::Spi as Spi>::Cr2 as Reg<Ftt>>::Val,
  ) -> Box<Future<Item = Self, Error = Self> + Send>;
}

/// Generic SPI with transmit-only DMA.
pub trait SpiDmaTx<T, IrqDmaTx>
where
  Self: Sized + Send + Sync + 'static,
  Self::Tokens: From<Self>,
  T: Thread,
  IrqDmaTx: ThreadNumber,
{
  /// Generic SPI with transmit-only DMA tokens.
  type Tokens;

  /// SPI.
  type Spi: Spi;

  /// DMA transmitting channel.
  type DmaTx: Dma<T, IrqDmaTx>;

  /// Creates a new `SpiDmaTx` driver from provided `tokens`.
  fn new(tokens: Self::Tokens) -> Self;

  /// Initializes DMA to use with SPI.
  fn dma_init(&self);

  /// Returns SPI.
  fn spi(&self) -> &Self::Spi;

  /// Returns DMA transmitting channel.
  fn dma_tx(&self) -> &Self::DmaTx;

  /// Returns a future, which resolves on DMA transmit complete.
  fn transfer_complete(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Ftt>>::Val,
    cr2: <<Self::Spi as Spi>::Cr2 as Reg<Ftt>>::Val,
  ) -> Box<Future<Item = Self, Error = Self> + Send>;
}

/// Generic SPI with receive-only DMA.
pub trait SpiDmaRx<T, IrqDmaRx>
where
  Self: Sized + Send + Sync + 'static,
  Self::Tokens: From<Self>,
  T: Thread,
  IrqDmaRx: ThreadNumber,
{
  /// Generic SPI with receive-only DMA tokens.
  type Tokens;

  /// SPI.
  type Spi: Spi;

  /// DMA receiving channel.
  type DmaRx: Dma<T, IrqDmaRx>;

  /// Creates a new `SpiDmaRx` driver from provided `tokens`.
  fn new(tokens: Self::Tokens) -> Self;

  /// Initializes DMA to use with SPI.
  fn dma_init(&self);

  /// Returns SPI.
  fn spi(&self) -> &Self::Spi;

  /// Returns DMA receiving channel.
  fn dma_rx(&self) -> &Self::DmaRx;

  /// Returns a future, which resolves on DMA receive complete.
  fn transfer_complete(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Ftt>>::Val,
  ) -> Box<Future<Item = Self, Error = Self> + Send>;
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
    $spi_ty:ident,
    $dma_tx_ty:ident,
    $dma_rx_ty:ident,
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
    pub struct $name_dx<T, IrqDmaTx, IrqDmaRx>
    where
      T: Thread,
      IrqDmaTx: $irq_dma_tx,
      IrqDmaRx: $irq_dma_rx,
    {
      tokens: $name_dx_tokens<T, IrqDmaTx, IrqDmaRx>,
    }

    #[doc = $doc_tx]
    pub struct $name_tx<T, IrqDmaTx>
    where
      T: Thread,
      IrqDmaTx: $irq_dma_tx,
    {
      tokens: $name_tx_tokens<T, IrqDmaTx>,
    }

    #[doc = $doc_rx]
    pub struct $name_rx<T, IrqDmaRx>
    where
      T: Thread,
      IrqDmaRx: $irq_dma_rx,
    {
      tokens: $name_rx_tokens<T, IrqDmaRx>,
    }

    #[doc = $doc_dx_tokens]
    #[allow(missing_docs)]
    pub struct $name_dx_tokens<T, IrqDmaTx, IrqDmaRx>
    where
      T: Thread,
      IrqDmaTx: $irq_dma_tx,
      IrqDmaRx: $irq_dma_rx,
    {
      pub $spi: $spi_ty,
      pub $dma_tx: $dma_tx_ty<T, IrqDmaTx>,
      pub $dma_rx: $dma_rx_ty<T, IrqDmaRx>,
    }

    #[doc = $doc_tx_tokens]
    #[allow(missing_docs)]
    pub struct $name_tx_tokens<T, IrqDmaTx>
    where
      T: Thread,
      IrqDmaTx: $irq_dma_tx,
    {
      pub $spi: $spi_ty,
      pub $dma_tx: $dma_tx_ty<T, IrqDmaTx>,
    }

    #[doc = $doc_rx_tokens]
    #[allow(missing_docs)]
    pub struct $name_rx_tokens<T, IrqDmaRx>
    where
      T: Thread,
      IrqDmaRx: $irq_dma_rx,
    {
      pub $spi: $spi_ty,
      pub $dma_rx: $dma_rx_ty<T, IrqDmaRx>,
    }

    /// Creates a new `SpiDmaDx` driver from tokens.
    #[macro_export]
    macro_rules! $name_dx_macro {
      ($thrd:ident, $regs:ident) => {
        $crate::peripherals::spi_dma::SpiDmaDx::new(
          $crate::peripherals::spi_dma::$name_dx_tokens {
            $spi: $spi_macro!($regs),
            $dma_tx: $dma_tx_macro!($thrd, $regs),
            $dma_rx: $dma_rx_macro!($thrd, $regs),
          }
        )
      }
    }

    /// Creates a new `SpiDmaTx` driver from tokens.
    #[macro_export]
    macro_rules! $name_tx_macro {
      ($thrd:ident, $regs:ident) => {
        $crate::peripherals::spi_dma::SpiDmaTx::new(
          $crate::peripherals::spi_dma::$name_tx_tokens {
            $spi: $spi_macro!($regs),
            $dma_tx: $dma_tx_macro!($thrd, $regs),
          }
        )
      }
    }

    /// Creates a new `SpiDmaRx` driver from tokens.
    #[macro_export]
    macro_rules! $name_rx_macro {
      ($thrd:ident, $regs:ident) => {
        $crate::peripherals::spi_dma::SpiDmaRx::new(
          $crate::peripherals::spi_dma::$name_rx_tokens {
            $spi: $spi_macro!($regs),
            $dma_rx: $dma_rx_macro!($thrd, $regs),
          }
        )
      }
    }

    impl<T, IrqDmaTx, IrqDmaRx> $name_dx<T, IrqDmaTx, IrqDmaRx>
    where
      T: Thread,
      IrqDmaTx: $irq_dma_tx,
      IrqDmaRx: $irq_dma_rx,
    {
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      #[inline(always)]
      fn select_channels(&self) {
        self.tokens.$dma_tx.cselr_cs().modify(|r| {
          self.tokens.$dma_tx.cselr_cs().write(r, $dma_tx_cs);
          self.tokens.$dma_rx.cselr_cs().write(r, $dma_rx_cs);
        });
      }

      #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                    feature = "stm32l4x3", feature = "stm32l4x5",
                    feature = "stm32l4x6")))]
      #[inline(always)]
      fn select_channels(&self) {}
    }

    impl<T, IrqDmaTx> $name_tx<T, IrqDmaTx>
    where
      T: Thread,
      IrqDmaTx: $irq_dma_tx,
    {
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      #[inline(always)]
      fn select_channel(&self) {
        self.tokens.$dma_tx.cselr_cs().write_bits($dma_tx_cs);
      }

      #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                    feature = "stm32l4x3", feature = "stm32l4x5",
                    feature = "stm32l4x6")))]
      #[inline(always)]
      fn select_channel(&self) {}
    }

    impl<T, IrqDmaRx> $name_rx<T, IrqDmaRx>
    where
      T: Thread,
      IrqDmaRx: $irq_dma_rx,
    {
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      #[inline(always)]
      fn select_channel(&self) {
        self.tokens.$dma_rx.cselr_cs().write_bits($dma_rx_cs);
      }

      #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                    feature = "stm32l4x3", feature = "stm32l4x5",
                    feature = "stm32l4x6")))]
      #[inline(always)]
      fn select_channel(&self) {}
    }

    impl<T, IrqDmaTx, IrqDmaRx> From<$name_dx<T, IrqDmaTx, IrqDmaRx>>
      for $name_dx_tokens<T, IrqDmaTx, IrqDmaRx>
    where
      T: Thread,
      IrqDmaTx: $irq_dma_tx,
      IrqDmaRx: $irq_dma_rx,
    {
      #[inline(always)]
      fn from(spi_dma_dx: $name_dx<T, IrqDmaTx, IrqDmaRx>) -> Self {
        spi_dma_dx.tokens
      }
    }

    impl<T, IrqDmaTx, IrqDmaRx> SpiDmaDx<T, IrqDmaTx, IrqDmaRx>
      for $name_dx<T, IrqDmaTx, IrqDmaRx>
    where
      T: Thread,
      IrqDmaTx: $irq_dma_tx,
      IrqDmaRx: $irq_dma_rx,
    {
      type Tokens = $name_dx_tokens<T, IrqDmaTx, IrqDmaRx>;
      type Spi = $spi_ty;
      type DmaTx = $dma_tx_ty<T, IrqDmaTx>;
      type DmaRx = $dma_rx_ty<T, IrqDmaRx>;

      #[inline(always)]
      fn new(tokens: Self::Tokens) -> Self {
        Self { tokens }
      }

      #[inline(always)]
      fn dma_init(&self) {
        let dr_ptr = self.tokens.$spi.dr().to_mut_ptr();
        self.tokens.$dma_rx.set_peripheral_address(dr_ptr as usize);
        self.tokens.$dma_tx.set_peripheral_address(dr_ptr as usize);
        self.select_channels();
      }

      #[inline(always)]
      fn spi(&self) -> &Self::Spi {
        &self.tokens.$spi
      }

      #[inline(always)]
      fn dma_tx(&self) -> &Self::DmaTx {
        &self.tokens.$dma_tx
      }

      #[inline(always)]
      fn dma_rx(&self) -> &Self::DmaRx {
        &self.tokens.$dma_rx
      }

      #[inline]
      fn transfer_complete(
        self,
        cr1: <<Self::Spi as Spi>::Cr1 as Reg<Ftt>>::Val,
        cr2: <<Self::Spi as Spi>::Cr2 as Reg<Ftt>>::Val,
      ) -> Box<Future<Item = Self, Error = Self> + Send> {
        let Self::Tokens {
          $spi,
          $dma_tx,
          $dma_rx,
          ..
        } = self.into();
        $spi.spe_after(cr1, move |$spi| {
          $spi.txdmaen_after(cr2, move |$spi| {
            let $dma_tx = $dma_tx.transfer_complete();
            let $dma_rx = $dma_rx.transfer_complete();
            Box::new(AsyncFuture::new(move || {
              let $dma_rx = await!($dma_rx);
              let $dma_tx = await!($dma_tx);
              match ($dma_tx, $dma_rx) {
                (Ok($dma_tx), Ok($dma_rx)) => Ok(Self::new(Self::Tokens {
                  $spi,
                  $dma_tx,
                  $dma_rx,
                })),
                (Ok($dma_tx), Err($dma_rx))
                | (Err($dma_tx), Ok($dma_rx))
                | (Err($dma_tx), Err($dma_rx)) => Err(Self::new(Self::Tokens {
                  $spi,
                  $dma_tx,
                  $dma_rx,
                })),
              }
            }))
          })
        })
      }
    }

    impl<T, IrqDmaTx> From<$name_tx<T, IrqDmaTx>>
      for $name_tx_tokens<T, IrqDmaTx>
    where
      T: Thread,
      IrqDmaTx: $irq_dma_tx,
    {
      #[inline(always)]
      fn from(spi_dma_tx: $name_tx<T, IrqDmaTx>) -> Self {
        spi_dma_tx.tokens
      }
    }

    impl<T, IrqDmaTx> SpiDmaTx<T, IrqDmaTx> for $name_tx<T, IrqDmaTx>
    where
      T: Thread,
      IrqDmaTx: $irq_dma_tx,
    {
      type Tokens = $name_tx_tokens<T, IrqDmaTx>;
      type Spi = $spi_ty;
      type DmaTx = $dma_tx_ty<T, IrqDmaTx>;

      #[inline(always)]
      fn new(tokens: Self::Tokens) -> Self {
        Self { tokens }
      }

      #[inline(always)]
      fn dma_init(&self) {
        let dr_ptr = self.tokens.$spi.dr().to_mut_ptr();
        self.tokens.$dma_tx.set_peripheral_address(dr_ptr as usize);
        self.select_channel();
      }

      #[inline(always)]
      fn spi(&self) -> &Self::Spi {
        &self.tokens.$spi
      }

      #[inline(always)]
      fn dma_tx(&self) -> &Self::DmaTx {
        &self.tokens.$dma_tx
      }

      #[inline]
      fn transfer_complete(
        self,
        cr1: <<Self::Spi as Spi>::Cr1 as Reg<Ftt>>::Val,
        cr2: <<Self::Spi as Spi>::Cr2 as Reg<Ftt>>::Val,
      ) -> Box<Future<Item = Self, Error = Self> + Send> {
        let Self::Tokens {
          $spi,
          $dma_tx,
          ..
        } = self.into();
        $spi.spe_after(cr1, move |$spi| {
          $spi.txdmaen_after(cr2, move |$spi| {
            let $dma_tx = $dma_tx.transfer_complete();
            Box::new(AsyncFuture::new(move || match await!($dma_tx) {
              Ok($dma_tx) => Ok(Self::new(Self::Tokens {
                $spi,
                $dma_tx,
              })),
              Err($dma_tx) => Err(Self::new(Self::Tokens {
                $spi,
                $dma_tx,
              })),
            }))
          })
        })
      }
    }

    impl<T, IrqDmaRx> From<$name_rx<T, IrqDmaRx>>
      for $name_rx_tokens<T, IrqDmaRx>
    where
      T: Thread,
      IrqDmaRx: $irq_dma_rx,
    {
      #[inline(always)]
      fn from(spi_dma_rx: $name_rx<T, IrqDmaRx>) -> Self {
        spi_dma_rx.tokens
      }
    }

    impl<T, IrqDmaRx> SpiDmaRx<T, IrqDmaRx> for $name_rx<T, IrqDmaRx>
    where
      T: Thread,
      IrqDmaRx: $irq_dma_rx,
    {
      type Tokens = $name_rx_tokens<T, IrqDmaRx>;
      type Spi = $spi_ty;
      type DmaRx = $dma_rx_ty<T, IrqDmaRx>;

      #[inline(always)]
      fn new(tokens: Self::Tokens) -> Self {
        Self { tokens }
      }

      #[inline(always)]
      fn dma_init(&self) {
        let dr_ptr = self.tokens.$spi.dr().to_ptr();
        self.tokens.$dma_rx.set_peripheral_address(dr_ptr as usize);
        self.select_channel();
      }

      #[inline(always)]
      fn spi(&self) -> &Self::Spi {
        &self.tokens.$spi
      }

      #[inline(always)]
      fn dma_rx(&self) -> &Self::DmaRx {
        &self.tokens.$dma_rx
      }

      #[inline]
      fn transfer_complete(
        self,
        cr1: <<Self::Spi as Spi>::Cr1 as Reg<Ftt>>::Val,
      ) -> Box<Future<Item = Self, Error = Self> + Send> {
        let Self::Tokens {
          $spi,
          $dma_rx,
          ..
        } = self.into();
        $spi.spe_after(cr1, move |$spi| {
          let $dma_rx = $dma_rx.transfer_complete();
          Box::new(AsyncFuture::new(move || match await!($dma_rx) {
            Ok($dma_rx) => Ok(Self::new(Self::Tokens {
              $spi,
              $dma_rx,
            })),
            Err($dma_rx) => Err(Self::new(Self::Tokens {
              $spi,
              $dma_rx,
            })),
          }))
        })
      }
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
  Spi1,
  Dma1Ch3,
  Dma1Ch2,
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
  Spi1,
  Dma2Ch4,
  Dma2Ch3,
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
  Spi2,
  Dma1Ch5,
  Dma1Ch4,
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
  Spi3,
  Dma2Ch2,
  Dma2Ch1,
  peripheral_spi3,
  peripheral_dma2_ch2,
  peripheral_dma2_ch1,
  spi3,
  dma2_ch2,
  dma2_ch1,
  0b0011,
  0b0011,
}
