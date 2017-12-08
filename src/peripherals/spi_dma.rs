//! SPI with DMA.

use core::mem;
use drone::thread::RoutineFuture;
use peripherals::dma::{Dma, Dma1Ch2, Dma1Ch3, Dma1Ch4, Dma1Ch5, Dma2Ch1,
                       Dma2Ch2};
#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
use peripherals::dma::{Dma2Ch3, Dma2Ch4};
use peripherals::spi::{Spi, Spi1, Spi2, Spi3};
use reg::prelude::*;

/// Generic SPI with duplex DMA.
pub trait SpiDma: Sized {
  /// SPI.
  type Spi: Spi;

  /// DMA transmitting channel.
  type DmaTx: Dma;

  /// DMA receiving channel.
  type DmaRx: Dma;

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
  fn transfer_complete<T: Thread>(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Fbt>>::Val,
    cr2: <<Self::Spi as Spi>::Cr2 as Reg<Fbt>>::Val,
    dma_tx_thread: &T,
    dma_rx_thread: &T,
  ) -> SpiDmaTransferComplete<Self>;
}

/// Generic SPI with transmit-only DMA.
pub trait SpiDmaTx: Sized {
  /// SPI.
  type Spi: Spi;

  /// DMA transmitting channel.
  type DmaTx: Dma;

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
  fn transfer_complete<T: Thread>(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Fbt>>::Val,
    cr2: <<Self::Spi as Spi>::Cr2 as Reg<Fbt>>::Val,
    dma_tx_thread: &T,
  ) -> SpiDmaTxTransferComplete<Self>;
}

/// Generic SPI with receive-only DMA.
pub trait SpiDmaRx: Sized {
  /// SPI.
  type Spi: Spi;

  /// DMA receiving channel.
  type DmaRx: Dma;

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
  fn transfer_complete<T: Thread>(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Fbt>>::Val,
    dma_rx_thread: &T,
  ) -> SpiDmaRxTransferComplete<Self>;
}

/// Future created by [`SpiDma::complete`] method.
///
/// [`SpiDma::complete`]: trait.SpiDma.html#method.complete
#[must_use]
pub struct SpiDmaTransferComplete<T: SpiDma>(CompleteState<T>);

enum CompleteState<T: SpiDma> {
  Tx(
    T::Spi,
    RoutineFuture<T::DmaTx, T::DmaTx>,
    RoutineFuture<T::DmaRx, T::DmaRx>,
  ),
  Rx(
    T::Spi,
    Result<T::DmaTx, T::DmaTx>,
    RoutineFuture<T::DmaRx, T::DmaRx>,
  ),
  Poisoned,
}

/// Future created by [`SpiDma::tx_complete`] method.
///
/// [`SpiDma::tx_complete`]: trait.SpiDma.html#method.tx_complete
#[must_use]
pub struct SpiDmaTxTransferComplete<T: SpiDmaTx>(TxCompleteState<T>);

enum TxCompleteState<T: SpiDmaTx> {
  Tx(T::Spi, RoutineFuture<T::DmaTx, T::DmaTx>),
  Poisoned,
}

/// Future created by [`SpiDma::rx_complete`] method.
///
/// [`SpiDma::rx_complete`]: trait.SpiDma.html#method.rx_complete
#[must_use]
pub struct SpiDmaRxTransferComplete<T: SpiDmaRx>(RxCompleteState<T>);

enum RxCompleteState<T: SpiDmaRx> {
  Rx(T::Spi, RoutineFuture<T::DmaRx, T::DmaRx>),
  Poisoned,
}

impl<I: imp::SpiDma> SpiDma for I {
  type Spi = I::Spi;
  type DmaTx = I::DmaTx;
  type DmaRx = I::DmaRx;

  #[inline(always)]
  fn compose(spi: Self::Spi, dma_tx: Self::DmaTx, dma_rx: Self::DmaRx) -> Self {
    Self::_compose(spi, dma_tx, dma_rx)
  }

  #[inline(always)]
  fn decompose(self) -> (Self::Spi, Self::DmaTx, Self::DmaRx) {
    self._decompose()
  }

  #[inline(always)]
  fn dma_init(&self) {
    self._dma_init()
  }

  #[inline(always)]
  fn spi(&self) -> &Self::Spi {
    self._spi()
  }

  #[inline(always)]
  fn dma_tx(&self) -> &Self::DmaTx {
    self._dma_tx()
  }

  #[inline(always)]
  fn dma_rx(&self) -> &Self::DmaRx {
    self._dma_rx()
  }

  #[inline]
  fn transfer_complete<T: Thread>(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Fbt>>::Val,
    cr2: <<Self::Spi as Spi>::Cr2 as Reg<Fbt>>::Val,
    dma_tx_thread: &T,
    dma_rx_thread: &T,
  ) -> SpiDmaTransferComplete<Self> {
    let (spi, dma_tx, dma_rx) = self._decompose();
    spi.spe_for(cr1, move |spi| {
      spi.txdmaen_for(cr2, move |spi| {
        let dma_tx = dma_tx.transfer_complete(dma_tx_thread);
        let dma_rx = dma_rx.transfer_complete(dma_rx_thread);
        SpiDmaTransferComplete(CompleteState::Tx(spi, dma_tx, dma_rx))
      })
    })
  }
}

impl<I: imp::SpiDmaTx> SpiDmaTx for I {
  type Spi = I::Spi;
  type DmaTx = I::DmaTx;

  #[inline(always)]
  fn compose(spi: Self::Spi, dma_tx: Self::DmaTx) -> Self {
    Self::_compose(spi, dma_tx)
  }

  #[inline(always)]
  fn decompose(self) -> (Self::Spi, Self::DmaTx) {
    self._decompose()
  }

  #[inline(always)]
  fn dma_init(&self) {
    self._dma_init()
  }

  #[inline(always)]
  fn spi(&self) -> &Self::Spi {
    self._spi()
  }

  #[inline(always)]
  fn dma_tx(&self) -> &Self::DmaTx {
    self._dma_tx()
  }

  #[inline]
  fn transfer_complete<T: Thread>(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Fbt>>::Val,
    cr2: <<Self::Spi as Spi>::Cr2 as Reg<Fbt>>::Val,
    dma_tx_thread: &T,
  ) -> SpiDmaTxTransferComplete<Self> {
    let (spi, dma_tx) = self._decompose();
    spi.spe_for(cr1, move |spi| {
      spi.txdmaen_for(cr2, move |spi| {
        let dma_tx = dma_tx.transfer_complete(dma_tx_thread);
        SpiDmaTxTransferComplete(TxCompleteState::Tx(spi, dma_tx))
      })
    })
  }
}

impl<I: imp::SpiDmaRx> SpiDmaRx for I {
  type Spi = I::Spi;
  type DmaRx = I::DmaRx;

  #[inline(always)]
  fn compose(spi: Self::Spi, dma_rx: Self::DmaRx) -> Self {
    Self::_compose(spi, dma_rx)
  }

  #[inline(always)]
  fn decompose(self) -> (Self::Spi, Self::DmaRx) {
    self._decompose()
  }

  #[inline(always)]
  fn dma_init(&self) {
    self._dma_init()
  }

  #[inline(always)]
  fn spi(&self) -> &Self::Spi {
    self._spi()
  }

  #[inline(always)]
  fn dma_rx(&self) -> &Self::DmaRx {
    self._dma_rx()
  }

  #[inline]
  fn transfer_complete<T: Thread>(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Fbt>>::Val,
    dma_rx_thread: &T,
  ) -> SpiDmaRxTransferComplete<Self> {
    let (spi, dma_rx) = self._decompose();
    spi.spe_for(cr1, move |spi| {
      let dma_rx = dma_rx.transfer_complete(dma_rx_thread);
      SpiDmaRxTransferComplete(RxCompleteState::Rx(spi, dma_rx))
    })
  }
}

impl<T: SpiDma> Future for SpiDmaTransferComplete<T> {
  type Item = T;
  type Error = T;

  fn poll(&mut self) -> Poll<T, T> {
    #[allow(unions_with_drop_fields)]
    union Advance<T: SpiDma> {
      dma_tx: Result<T::DmaTx, T::DmaTx>,
      dma_rx: Result<T::DmaRx, T::DmaRx>,
    }
    let advance: Advance<T> = match self.0 {
      CompleteState::Tx(_, ref mut dma_tx, _) => match dma_tx.poll() {
        Ok(Async::NotReady) => return Ok(Async::NotReady),
        Ok(Async::Ready(dma_tx)) => Advance { dma_tx: Ok(dma_tx) },
        Err(dma_tx) => Advance {
          dma_tx: Err(dma_tx),
        },
      },
      CompleteState::Rx(_, _, ref mut dma_rx) => match dma_rx.poll() {
        Ok(Async::NotReady) => return Ok(Async::NotReady),
        Ok(Async::Ready(dma_rx)) => Advance { dma_rx: Ok(dma_rx) },
        Err(dma_rx) => Advance {
          dma_rx: Err(dma_rx),
        },
      },
      CompleteState::Poisoned => panic!("cannot poll a future twice"),
    };
    match mem::replace(&mut self.0, CompleteState::Poisoned) {
      CompleteState::Tx(spi, _, mut dma_rx) => {
        let dma_tx = unsafe { advance.dma_tx };
        match dma_rx.poll() {
          Ok(Async::NotReady) => {
            self.0 = CompleteState::Rx(spi, dma_tx, dma_rx);
            Ok(Async::NotReady)
          }
          Ok(Async::Ready(dma_rx)) => match dma_tx {
            Ok(dma_tx) => Ok(Async::Ready(T::compose(spi, dma_tx, dma_rx))),
            Err(dma_tx) => Err(T::compose(spi, dma_tx, dma_rx)),
          },
          Err(dma_rx) => match dma_tx {
            Ok(dma_tx) | Err(dma_tx) => Err(T::compose(spi, dma_tx, dma_rx)),
          },
        }
      }
      CompleteState::Rx(spi, dma_tx, _) => {
        let dma_rx = unsafe { advance.dma_rx };
        match dma_rx {
          Ok(dma_rx) => match dma_tx {
            Ok(dma_tx) => Ok(Async::Ready(T::compose(spi, dma_tx, dma_rx))),
            Err(dma_tx) => Err(T::compose(spi, dma_tx, dma_rx)),
          },
          Err(dma_rx) => match dma_tx {
            Ok(dma_tx) | Err(dma_tx) => Err(T::compose(spi, dma_tx, dma_rx)),
          },
        }
      }
      CompleteState::Poisoned => unsafe { mem::unreachable() },
    }
  }
}

impl<T: SpiDmaTx> Future for SpiDmaTxTransferComplete<T> {
  type Item = T;
  type Error = T;

  fn poll(&mut self) -> Poll<T, T> {
    let dma_tx = match self.0 {
      TxCompleteState::Tx(_, ref mut dma_tx) => match dma_tx.poll() {
        Ok(Async::NotReady) => return Ok(Async::NotReady),
        Ok(Async::Ready(dma_tx)) => Ok(dma_tx),
        Err(dma_tx) => Err(dma_tx),
      },
      TxCompleteState::Poisoned => panic!("cannot poll a future twice"),
    };
    match mem::replace(&mut self.0, TxCompleteState::Poisoned) {
      TxCompleteState::Tx(spi, _) => match dma_tx {
        Ok(dma_tx) => Ok(Async::Ready(T::compose(spi, dma_tx))),
        Err(dma_tx) => Err(T::compose(spi, dma_tx)),
      },
      TxCompleteState::Poisoned => unsafe { mem::unreachable() },
    }
  }
}

impl<T: SpiDmaRx> Future for SpiDmaRxTransferComplete<T> {
  type Item = T;
  type Error = T;

  fn poll(&mut self) -> Poll<T, T> {
    let dma_rx = match self.0 {
      RxCompleteState::Rx(_, ref mut dma_rx) => match dma_rx.poll() {
        Ok(Async::NotReady) => return Ok(Async::NotReady),
        Ok(Async::Ready(dma_rx)) => Ok(dma_rx),
        Err(dma_rx) => Err(dma_rx),
      },
      RxCompleteState::Poisoned => panic!("cannot poll a future twice"),
    };
    match mem::replace(&mut self.0, RxCompleteState::Poisoned) {
      RxCompleteState::Rx(spi, _) => match dma_rx {
        Ok(dma_rx) => Ok(Async::Ready(T::compose(spi, dma_rx))),
        Err(dma_rx) => Err(T::compose(spi, dma_rx)),
      },
      RxCompleteState::Poisoned => unsafe { mem::unreachable() },
    }
  }
}

#[doc(hidden)]
mod imp {
  use peripherals::dma::Dma;
  use peripherals::spi::Spi;

  pub trait SpiDma: Sized {
    type Spi: Spi;
    type DmaTx: Dma;
    type DmaRx: Dma;

    fn _compose(
      spi: Self::Spi,
      dma_tx: Self::DmaTx,
      dma_rx: Self::DmaRx,
    ) -> Self;

    fn _decompose(self) -> (Self::Spi, Self::DmaTx, Self::DmaRx);

    fn _dma_init(&self);

    fn _spi(&self) -> &Self::Spi;
    fn _dma_tx(&self) -> &Self::DmaTx;
    fn _dma_rx(&self) -> &Self::DmaRx;
  }

  pub trait SpiDmaTx: Sized {
    type Spi: Spi;
    type DmaTx: Dma;

    fn _compose(spi: Self::Spi, dma_tx: Self::DmaTx) -> Self;

    fn _decompose(self) -> (Self::Spi, Self::DmaTx);

    fn _dma_init(&self);

    fn _spi(&self) -> &Self::Spi;
    fn _dma_tx(&self) -> &Self::DmaTx;
  }

  pub trait SpiDmaRx: Sized {
    type Spi: Spi;
    type DmaRx: Dma;

    fn _compose(spi: Self::Spi, dma_rx: Self::DmaRx) -> Self;

    fn _decompose(self) -> (Self::Spi, Self::DmaRx);

    fn _dma_init(&self);

    fn _spi(&self) -> &Self::Spi;
    fn _dma_rx(&self) -> &Self::DmaRx;
  }
}

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
macro spi_dma(
  $doc:expr,
  $name:ident,
  $doc_tx:expr,
  $name_tx:ident,
  $doc_rx:expr,
  $name_rx:ident,
  $spi:ident,
  $dma_tx:ident,
  $dma_rx:ident,
  $dma_tx_cs:expr,
  $dma_rx_cs:expr,
) {
  #[doc = $doc]
  pub struct $name {
    spi: $spi,
    dma_tx: $dma_tx,
    dma_rx: $dma_rx,
  }

  #[doc = $doc_tx]
  pub struct $name_tx {
    spi: $spi,
    dma_tx: $dma_tx,
  }

  #[doc = $doc_rx]
  pub struct $name_rx {
    spi: $spi,
    dma_rx: $dma_rx,
  }

  impl $name {
    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    #[inline(always)]
    fn select_channel(&self) {
      self.dma_tx.cselr_cs().modify(|r| {
        self.dma_tx.cselr_cs().write(r, $dma_tx_cs);
        self.dma_rx.cselr_cs().write(r, $dma_rx_cs);
      });
    }

    #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                  feature = "stm32l4x3", feature = "stm32l4x5",
                  feature = "stm32l4x6")))]
    #[inline(always)]
    fn select_channel(&self) {}
  }

  impl $name_tx {
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

  impl $name_rx {
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

  impl imp::SpiDma for $name {
    type Spi = $spi;
    type DmaTx = $dma_tx;
    type DmaRx = $dma_rx;

    #[inline(always)]
    fn _compose(
      spi: Self::Spi,
      dma_tx: Self::DmaTx,
      dma_rx: Self::DmaRx,
    ) -> Self {
      Self { spi, dma_tx, dma_rx }
    }

    #[inline(always)]
    fn _decompose(self) -> (Self::Spi, Self::DmaTx, Self::DmaRx) {
      (self.spi, self.dma_tx, self.dma_rx)
    }

    #[inline(always)]
    fn _dma_init(&self) {
      let dr = self.spi.dr();
      self.dma_rx.cpar().reset(|r| r.write_pa(dr.to_ptr() as u32));
      self.dma_tx.cpar().reset(|r| r.write_pa(dr.to_mut_ptr() as u32));
      self.select_channel();
    }

    #[inline(always)]
    fn _spi(&self) -> &Self::Spi {
      &self.spi
    }

    #[inline(always)]
    fn _dma_tx(&self) -> &Self::DmaTx {
      &self.dma_tx
    }

    #[inline(always)]
    fn _dma_rx(&self) -> &Self::DmaRx {
      &self.dma_rx
    }
  }

  impl imp::SpiDmaTx for $name_tx {
    type Spi = $spi;
    type DmaTx = $dma_tx;

    #[inline(always)]
    fn _compose(spi: Self::Spi, dma_tx: Self::DmaTx) -> Self {
      Self { spi, dma_tx }
    }

    #[inline(always)]
    fn _decompose(self) -> (Self::Spi, Self::DmaTx) {
      (self.spi, self.dma_tx)
    }

    #[inline(always)]
    fn _dma_init(&self) {
      let dr = self.spi.dr();
      self.dma_tx.cpar().reset(|r| r.write_pa(dr.to_mut_ptr() as u32));
      self.select_channel();
    }

    #[inline(always)]
    fn _spi(&self) -> &Self::Spi {
      &self.spi
    }

    #[inline(always)]
    fn _dma_tx(&self) -> &Self::DmaTx {
      &self.dma_tx
    }
  }

  impl imp::SpiDmaRx for $name_rx {
    type Spi = $spi;
    type DmaRx = $dma_rx;

    #[inline(always)]
    fn _compose(spi: Self::Spi, dma_rx: Self::DmaRx) -> Self {
      Self { spi, dma_rx }
    }

    #[inline(always)]
    fn _decompose(self) -> (Self::Spi, Self::DmaRx) {
      (self.spi, self.dma_rx)
    }

    #[inline(always)]
    fn _dma_init(&self) {
      let dr = self.spi.dr();
      self.dma_rx.cpar().reset(|r| r.write_pa(dr.to_ptr() as u32));
      self.select_channel();
    }

    #[inline(always)]
    fn _spi(&self) -> &Self::Spi {
      &self.spi
    }

    #[inline(always)]
    fn _dma_rx(&self) -> &Self::DmaRx {
      &self.dma_rx
    }
  }
}

spi_dma! {
  "SPI1 with duplex DMA1",
  Spi1Dma1,
  "SPI1 with transmit-only DMA1",
  Spi1Dma1Tx,
  "SPI1 with receive-only DMA1",
  Spi1Dma1Rx,
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
  Spi1,
  Dma2Ch4,
  Dma2Ch3,
  0b0100,
  0b0100,
}

spi_dma! {
  "SPI2 with duplex DMA1",
  Spi2Dma1,
  "SPI2 with transmit-only DMA1",
  Spi2Dma1Tx,
  "SPI2 with receive-only DMA1",
  Spi2Dma1Rx,
  Spi2,
  Dma1Ch5,
  Dma1Ch4,
  0b0001,
  0b0001,
}

spi_dma! {
  "SPI3 with duplex DMA2",
  Spi3Dma2,
  "SPI3 with transmit-only DMA2",
  Spi3Dma2Tx,
  "SPI3 with receive-only DMA2",
  Spi3Dma2Rx,
  Spi3,
  Dma2Ch2,
  Dma2Ch1,
  0b0011,
  0b0011,
}
