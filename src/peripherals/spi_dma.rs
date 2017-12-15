//! SPI with DMA.

#[allow(unused_imports)]
use core::marker::PhantomData;
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
use thread::interrupts::{IrqDma1Ch2, IrqDma1Ch3, IrqDma1Ch4, IrqDma1Ch5,
                         IrqDma2Ch1, IrqDma2Ch2, IrqDma2Ch3, IrqDma2Ch4};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x3",
          feature = "stm32l4x5"))]
use thread::interrupts::{IrqDma1Channel2 as IrqDma1Ch2,
                         IrqDma1Channel3 as IrqDma1Ch3,
                         IrqDma1Channel4 as IrqDma1Ch4,
                         IrqDma1Channel5 as IrqDma1Ch5,
                         IrqDma2Channel1 as IrqDma2Ch1,
                         IrqDma2Channel2 as IrqDma2Ch2};
#[cfg(any(feature = "stm32l4x3", feature = "stm32l4x5"))]
use thread::interrupts::{IrqDma2Channel3 as IrqDma2Ch3,
                         IrqDma2Channel4 as IrqDma2Ch4};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use thread::interrupts::{IrqSpi1, IrqSpi2, IrqSpi3};

/// Generic SPI with duplex DMA.
pub trait SpiDma<T, IrqSpi, IrqDmaTx, IrqDmaRx>
where
  Self: Sized,
  T: Thread,
  IrqSpi: ThreadBinding<T>,
  IrqDmaTx: ThreadBinding<T>,
  IrqDmaRx: ThreadBinding<T>,
{
  /// SPI.
  type Spi: Spi<T, IrqSpi>;

  /// DMA transmitting channel.
  type DmaTx: Dma<T, IrqDmaTx>;

  /// DMA receiving channel.
  type DmaRx: Dma<T, IrqDmaRx>;

  /// Composes a new `SpiDma` from pieces.
  fn compose(spi: Self::Spi, dma_tx: Self::DmaTx, dma_rx: Self::DmaRx) -> Self;

  /// Decomposes the `SpiDma` into pieces.
  fn decompose(self) -> (Self::Spi, Self::DmaTx, Self::DmaRx);

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
    cr1: <<Self::Spi as Spi<T, IrqSpi>>::Cr1 as Reg<Fbt>>::Val,
    cr2: <<Self::Spi as Spi<T, IrqSpi>>::Cr2 as Reg<Fbt>>::Val,
  ) -> Box<Future<Item = Self, Error = Self>>;
}

/// Generic SPI with transmit-only DMA.
pub trait SpiDmaTx<T, IrqSpi, IrqDmaTx>
where
  Self: Sized,
  T: Thread,
  IrqSpi: ThreadBinding<T>,
  IrqDmaTx: ThreadBinding<T>,
{
  /// SPI.
  type Spi: Spi<T, IrqSpi>;

  /// DMA transmitting channel.
  type DmaTx: Dma<T, IrqDmaTx>;

  /// Composes a new `SpiDmaTx` from pieces.
  fn compose(spi: Self::Spi, dma_tx: Self::DmaTx) -> Self;

  /// Decomposes the `SpiDmaTx` into pieces.
  fn decompose(self) -> (Self::Spi, Self::DmaTx);

  /// Initializes DMA to use with SPI.
  fn dma_init(&self);

  /// Returns SPI.
  fn spi(&self) -> &Self::Spi;

  /// Returns DMA transmitting channel.
  fn dma_tx(&self) -> &Self::DmaTx;

  /// Returns a future, which resolves on DMA transmit complete.
  fn transfer_complete(
    self,
    cr1: <<Self::Spi as Spi<T, IrqSpi>>::Cr1 as Reg<Fbt>>::Val,
    cr2: <<Self::Spi as Spi<T, IrqSpi>>::Cr2 as Reg<Fbt>>::Val,
  ) -> Box<Future<Item = Self, Error = Self>>;
}

/// Generic SPI with receive-only DMA.
pub trait SpiDmaRx<T, IrqSpi, IrqDmaRx>
where
  Self: Sized,
  T: Thread,
  IrqSpi: ThreadBinding<T>,
  IrqDmaRx: ThreadBinding<T>,
{
  /// SPI.
  type Spi: Spi<T, IrqSpi>;

  /// DMA receiving channel.
  type DmaRx: Dma<T, IrqDmaRx>;

  /// Composes a new `SpiDmaRx` from pieces.
  fn compose(spi: Self::Spi, dma_rx: Self::DmaRx) -> Self;

  /// Decomposes the `SpiDmaRx` into pieces.
  fn decompose(self) -> (Self::Spi, Self::DmaRx);

  /// Initializes DMA to use with SPI.
  fn dma_init(&self);

  /// Returns SPI.
  fn spi(&self) -> &Self::Spi;

  /// Returns DMA receiving channel.
  fn dma_rx(&self) -> &Self::DmaRx;

  /// Returns a future, which resolves on DMA receive complete.
  fn transfer_complete(
    self,
    cr1: <<Self::Spi as Spi<T, IrqSpi>>::Cr1 as Reg<Fbt>>::Val,
  ) -> Box<Future<Item = Self, Error = Self>>;
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
macro_rules! spi_dma {
  (
    $doc:expr,
    $name:ident,
    $doc_tx:expr,
    $name_tx:ident,
    $doc_rx:expr,
    $name_rx:ident,
    $irq_spi:ident,
    $irq_dma_tx:ident,
    $irq_dma_rx:ident,
    $spi:ident,
    $dma_tx:ident,
    $dma_rx:ident,
    $dma_tx_cs:expr,
    $dma_rx_cs:expr,
  ) => {
    #[doc = $doc]
    pub struct $name<T, IrqSpi, IrqDmaTx, IrqDmaRx>
    where
      T: Thread,
      IrqSpi: $irq_spi<T>,
      IrqDmaTx: $irq_dma_tx<T>,
      IrqDmaRx: $irq_dma_rx<T>,
    {
      _thread: PhantomData<&'static T>,
      spi: $spi<T, IrqSpi>,
      dma_tx: $dma_tx<T, IrqDmaTx>,
      dma_rx: $dma_rx<T, IrqDmaRx>,
    }

    #[doc = $doc_tx]
    pub struct $name_tx<T, IrqSpi, IrqDmaTx>
    where
      T: Thread,
      IrqSpi: $irq_spi<T>,
      IrqDmaTx: $irq_dma_tx<T>,
    {
      _thread: PhantomData<&'static T>,
      spi: $spi<T, IrqSpi>,
      dma_tx: $dma_tx<T, IrqDmaTx>,
    }

    #[doc = $doc_rx]
    pub struct $name_rx<T, IrqSpi, IrqDmaRx>
    where
      T: Thread,
      IrqSpi: $irq_spi<T>,
      IrqDmaRx: $irq_dma_rx<T>,
    {
      _thread: PhantomData<&'static T>,
      spi: $spi<T, IrqSpi>,
      dma_rx: $dma_rx<T, IrqDmaRx>,
    }

    impl<T, IrqSpi, IrqDmaTx, IrqDmaRx> $name<T, IrqSpi, IrqDmaTx, IrqDmaRx>
    where
      T: Thread,
      IrqSpi: $irq_spi<T>,
      IrqDmaTx: $irq_dma_tx<T>,
      IrqDmaRx: $irq_dma_rx<T>,
    {
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      #[inline(always)]
      fn select_channels(&self) {
        self.dma_tx.cselr_cs().modify(|r| {
          self.dma_tx.cselr_cs().write(r, $dma_tx_cs);
          self.dma_rx.cselr_cs().write(r, $dma_rx_cs);
        });
      }

      #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                    feature = "stm32l4x3", feature = "stm32l4x5",
                    feature = "stm32l4x6")))]
      #[inline(always)]
      fn select_channels(&self) {}
    }

    impl<T, IrqSpi, IrqDmaTx> $name_tx<T, IrqSpi, IrqDmaTx>
    where
      T: Thread,
      IrqSpi: $irq_spi<T>,
      IrqDmaTx: $irq_dma_tx<T>,
    {
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      #[inline(always)]
      fn select_channel(&self) {
        self.dma_tx.cselr_cs().write_bits($dma_tx_cs);
      }

      #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                    feature = "stm32l4x3", feature = "stm32l4x5",
                    feature = "stm32l4x6")))]
      #[inline(always)]
      fn select_channel(&self) {}
    }

    impl<T, IrqSpi, IrqDmaRx> $name_rx<T, IrqSpi, IrqDmaRx>
    where
      T: Thread,
      IrqSpi: $irq_spi<T>,
      IrqDmaRx: $irq_dma_rx<T>,
    {
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      #[inline(always)]
      fn select_channel(&self) {
        self.dma_rx.cselr_cs().write_bits($dma_rx_cs);
      }

      #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                    feature = "stm32l4x3", feature = "stm32l4x5",
                    feature = "stm32l4x6")))]
      #[inline(always)]
      fn select_channel(&self) {}
    }

    impl<T, IrqSpi, IrqDmaTx, IrqDmaRx> SpiDma<T, IrqSpi, IrqDmaTx, IrqDmaRx>
      for $name<T, IrqSpi, IrqDmaTx, IrqDmaRx>
    where
      T: Thread,
      IrqSpi: $irq_spi<T>,
      IrqDmaTx: $irq_dma_tx<T>,
      IrqDmaRx: $irq_dma_rx<T>,
    {
      type Spi = $spi<T, IrqSpi>;
      type DmaTx = $dma_tx<T, IrqDmaTx>;
      type DmaRx = $dma_rx<T, IrqDmaRx>;

      #[inline(always)]
      fn compose(
        spi: Self::Spi,
        dma_tx: Self::DmaTx,
        dma_rx: Self::DmaRx,
      ) -> Self {
        Self {
          _thread: PhantomData,
          spi,
          dma_tx,
          dma_rx,
        }
      }

      #[inline(always)]
      fn decompose(self) -> (Self::Spi, Self::DmaTx, Self::DmaRx) {
        (self.spi, self.dma_tx, self.dma_rx)
      }

      #[inline(always)]
      fn dma_init(&self) {
        let dr = self.spi.dr();
        self.dma_rx.cpar().reset(|r| r.write_pa(dr.to_ptr() as u32));
        self.dma_tx.cpar().reset(|r| r.write_pa(dr.to_mut_ptr() as u32));
        self.select_channels();
      }

      #[inline(always)]
      fn spi(&self) -> &Self::Spi {
        &self.spi
      }

      #[inline(always)]
      fn dma_tx(&self) -> &Self::DmaTx {
        &self.dma_tx
      }

      #[inline(always)]
      fn dma_rx(&self) -> &Self::DmaRx {
        &self.dma_rx
      }

      #[inline]
      fn transfer_complete(
        self,
        cr1: <<Self::Spi as Spi<T, IrqSpi>>::Cr1 as Reg<Fbt>>::Val,
        cr2: <<Self::Spi as Spi<T, IrqSpi>>::Cr2 as Reg<Fbt>>::Val,
      ) -> Box<Future<Item = Self, Error = Self>> {
        let Self { _thread, spi, dma_tx, dma_rx } = self;
        spi.spe_after(cr1, move |spi| {
          spi.txdmaen_after(cr2, move |spi| {
            let dma_tx = dma_tx.transfer_complete();
            let dma_rx = dma_rx.transfer_complete();
            Box::new(AsyncFuture::new(move || {
              let dma_rx = await!(dma_rx);
              let dma_tx = await!(dma_tx);
              match (dma_tx, dma_rx) {
                (Ok(dma_tx), Ok(dma_rx)) => {
                  Ok(Self::compose(spi, dma_tx, dma_rx))
                }
                (Ok(dma_tx), Err(dma_rx)) |
                (Err(dma_tx), Ok(dma_rx)) |
                (Err(dma_tx), Err(dma_rx)) => {
                  Err(Self::compose(spi, dma_tx, dma_rx))
                }
              }
            }))
          })
        })
      }
    }

    impl<T, IrqSpi, IrqDmaTx> SpiDmaTx<T, IrqSpi, IrqDmaTx>
      for $name_tx<T, IrqSpi, IrqDmaTx>
    where
      T: Thread,
      IrqSpi: $irq_spi<T>,
      IrqDmaTx: $irq_dma_tx<T>,
    {
      type Spi = $spi<T, IrqSpi>;
      type DmaTx = $dma_tx<T, IrqDmaTx>;

      #[inline(always)]
      fn compose(spi: Self::Spi, dma_tx: Self::DmaTx) -> Self {
        Self {
          _thread: PhantomData,
          spi,
          dma_tx,
        }
      }

      #[inline(always)]
      fn decompose(self) -> (Self::Spi, Self::DmaTx) {
        (self.spi, self.dma_tx)
      }

      #[inline(always)]
      fn dma_init(&self) {
        let dr = self.spi.dr();
        self.dma_tx.cpar().reset(|r| r.write_pa(dr.to_mut_ptr() as u32));
        self.select_channel();
      }

      #[inline(always)]
      fn spi(&self) -> &Self::Spi {
        &self.spi
      }

      #[inline(always)]
      fn dma_tx(&self) -> &Self::DmaTx {
        &self.dma_tx
      }

      #[inline]
      fn transfer_complete(
        self,
        cr1: <<Self::Spi as Spi<T, IrqSpi>>::Cr1 as Reg<Fbt>>::Val,
        cr2: <<Self::Spi as Spi<T, IrqSpi>>::Cr2 as Reg<Fbt>>::Val,
      ) -> Box<Future<Item = Self, Error = Self>> {
        let Self { _thread, spi, dma_tx } = self;
        spi.spe_after(cr1, move |spi| {
          spi.txdmaen_after(cr2, move |spi| {
            let dma_tx = dma_tx.transfer_complete();
            Box::new(AsyncFuture::new(move || match await!(dma_tx) {
              Ok(dma_tx) => Ok(Self::compose(spi, dma_tx)),
              Err(dma_tx) => Err(Self::compose(spi, dma_tx)),
            }))
          })
        })
      }
    }

    impl<T, IrqSpi, IrqDmaRx> SpiDmaRx<T, IrqSpi, IrqDmaRx>
      for $name_rx<T, IrqSpi, IrqDmaRx>
    where
      T: Thread,
      IrqSpi: $irq_spi<T>,
      IrqDmaRx: $irq_dma_rx<T>,
    {
      type Spi = $spi<T, IrqSpi>;
      type DmaRx = $dma_rx<T, IrqDmaRx>;

      #[inline(always)]
      fn compose(spi: Self::Spi, dma_rx: Self::DmaRx) -> Self {
        Self {
          _thread: PhantomData,
          spi,
          dma_rx,
        }
      }

      #[inline(always)]
      fn decompose(self) -> (Self::Spi, Self::DmaRx) {
        (self.spi, self.dma_rx)
      }

      #[inline(always)]
      fn dma_init(&self) {
        let dr = self.spi.dr();
        self.dma_rx.cpar().reset(|r| r.write_pa(dr.to_ptr() as u32));
        self.select_channel();
      }

      #[inline(always)]
      fn spi(&self) -> &Self::Spi {
        &self.spi
      }

      #[inline(always)]
      fn dma_rx(&self) -> &Self::DmaRx {
        &self.dma_rx
      }

      #[inline]
      fn transfer_complete(
        self,
        cr1: <<Self::Spi as Spi<T, IrqSpi>>::Cr1 as Reg<Fbt>>::Val,
      ) -> Box<Future<Item = Self, Error = Self>> {
        let Self { _thread, spi, dma_rx } = self;
        spi.spe_after(cr1, move |spi| {
          let dma_rx = dma_rx.transfer_complete();
          Box::new(AsyncFuture::new(move || match await!(dma_rx) {
            Ok(dma_rx) => Ok(Self::compose(spi, dma_rx)),
            Err(dma_rx) => Err(Self::compose(spi, dma_rx)),
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
  Spi1Dma1,
  "SPI1 with transmit-only DMA1",
  Spi1Dma1Tx,
  "SPI1 with receive-only DMA1",
  Spi1Dma1Rx,
  IrqSpi1,
  IrqDma1Ch3,
  IrqDma1Ch2,
  Spi1,
  Dma1Ch3,
  Dma1Ch2,
  0b0001,
  0b0001,
}

#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
spi_dma! {
  "SPI1 with duplex DMA2",
  Spi1Dma2,
  "SPI1 with transmit-only DMA2",
  Spi1Dma2Tx,
  "SPI1 with receive-only DMA2",
  Spi1Dma2Rx,
  IrqSpi1,
  IrqDma2Ch4,
  IrqDma2Ch3,
  Spi1,
  Dma2Ch4,
  Dma2Ch3,
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
  Spi2Dma1,
  "SPI2 with transmit-only DMA1",
  Spi2Dma1Tx,
  "SPI2 with receive-only DMA1",
  Spi2Dma1Rx,
  IrqSpi2,
  IrqDma1Ch5,
  IrqDma1Ch4,
  Spi2,
  Dma1Ch5,
  Dma1Ch4,
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
  Spi3Dma2,
  "SPI3 with transmit-only DMA2",
  Spi3Dma2Tx,
  "SPI3 with receive-only DMA2",
  Spi3Dma2Rx,
  IrqSpi3,
  IrqDma2Ch2,
  IrqDma2Ch1,
  Spi3,
  Dma2Ch2,
  Dma2Ch1,
  0b0011,
  0b0011,
}
