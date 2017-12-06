//! SPI through DMA.

use core::mem;
use drone::thread::RoutineFuture;
use peripherals::dma::{Dma, dma1_ch2, dma1_ch3, dma1_ch4, dma1_ch5, dma2_ch1,
                       dma2_ch2};
#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
use peripherals::dma::{dma2_ch3, dma2_ch4};
use peripherals::spi::{Spi, spi1, spi2, spi3};
use reg::prelude::*;

/// Generic SPI through DMA.
pub trait SpiDma
where
  Self: Sized,
{
  /// SPI.
  type Spi: Spi;

  /// DMA transmitting channel.
  type DmaTx: Dma;

  /// DMA receiving channel.
  type DmaRx: Dma;

  /// Creates a new `SpiDma`.
  fn integrate(
    spi: Self::Spi,
    dma_tx: Self::DmaTx,
    dma_rx: Self::DmaRx,
  ) -> Self;

  /// Frees underlying resources.
  fn disintegrate(self) -> (Self::Spi, Self::DmaTx, Self::DmaRx);

  /// Returns SPI.
  fn spi(&self) -> &Self::Spi;

  /// Returns DMA transmitting channel.
  fn dma_tx(&self) -> &Self::DmaTx;

  /// Returns DMA receiving channel.
  fn dma_rx(&self) -> &Self::DmaRx;

  /// Returns a future, which resolves on both DMA transmit and receive
  /// complete.
  fn complete<T>(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Drt>>::Val,
    cr2: <<Self::Spi as Spi>::Cr2 as Reg<Drt>>::Val,
    dma_tx_thread: &T,
    dma_rx_thread: &T,
  ) -> SpiDmaComplete<Self>
  where
    T: Thread;

  /// Returns a future, which resolves on DMA transmit complete.
  fn tx_complete<T>(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Drt>>::Val,
    cr2: <<Self::Spi as Spi>::Cr2 as Reg<Drt>>::Val,
    dma_tx_thread: &T,
  ) -> SpiDmaTxComplete<Self>
  where
    T: Thread;
}

/// Future created by [`SpiDma::complete`] method.
///
/// [`SpiDma::complete`]: trait.SpiDma.html#method.complete
#[must_use]
pub struct SpiDmaComplete<T>(CompleteState<T>)
where
  T: SpiDma;

enum CompleteState<T>
where
  T: SpiDma,
{
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
pub struct SpiDmaTxComplete<T>(TxCompleteState<T>)
where
  T: SpiDma;

enum TxCompleteState<T>
where
  T: SpiDma,
{
  Tx(T::Spi, RoutineFuture<T::DmaTx, T::DmaTx>, T::DmaRx),
  Poisoned,
}

impl<I> SpiDma for I
where
  I: imp::SpiDma,
{
  type Spi = I::Spi;
  type DmaTx = I::DmaTx;
  type DmaRx = I::DmaRx;

  #[inline(always)]
  fn integrate(
    spi: Self::Spi,
    dma_tx: Self::DmaTx,
    dma_rx: Self::DmaRx,
  ) -> Self {
    Self::_integrate(spi, dma_tx, dma_rx)
  }

  #[inline(always)]
  fn disintegrate(self) -> (Self::Spi, Self::DmaTx, Self::DmaRx) {
    self._disintegrate()
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
  fn complete<T>(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Drt>>::Val,
    cr2: <<Self::Spi as Spi>::Cr2 as Reg<Drt>>::Val,
    dma_tx_thread: &T,
    dma_rx_thread: &T,
  ) -> SpiDmaComplete<Self>
  where
    T: Thread,
  {
    let (spi, dma_tx, dma_rx) = self._disintegrate();
    spi.spe_for(cr1, move |spi| {
      spi.txdmaen_for(cr2, move |spi| {
        let dma_tx = dma_tx.transfer_complete(dma_tx_thread);
        let dma_rx = dma_rx.transfer_complete(dma_rx_thread);
        SpiDmaComplete::new(spi, dma_tx, dma_rx)
      })
    })
  }

  #[inline]
  fn tx_complete<T>(
    self,
    cr1: <<Self::Spi as Spi>::Cr1 as Reg<Drt>>::Val,
    cr2: <<Self::Spi as Spi>::Cr2 as Reg<Drt>>::Val,
    dma_tx_thread: &T,
  ) -> SpiDmaTxComplete<Self>
  where
    T: Thread,
  {
    let (spi, dma_tx, dma_rx) = self._disintegrate();
    spi.spe_for(cr1, move |spi| {
      spi.txdmaen_for(cr2, move |spi| {
        let dma_tx = dma_tx.transfer_complete(dma_tx_thread);
        SpiDmaTxComplete::new(spi, dma_tx, dma_rx)
      })
    })
  }
}

impl<T> SpiDmaComplete<T>
where
  T: SpiDma,
{
  #[inline(always)]
  fn new(
    spi: T::Spi,
    dma_tx: RoutineFuture<T::DmaTx, T::DmaTx>,
    dma_rx: RoutineFuture<T::DmaRx, T::DmaRx>,
  ) -> Self {
    SpiDmaComplete(CompleteState::Tx(spi, dma_tx, dma_rx))
  }
}

impl<T> Future for SpiDmaComplete<T>
where
  T: SpiDma,
{
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
            Ok(dma_tx) => Ok(Async::Ready(T::integrate(spi, dma_tx, dma_rx))),
            Err(dma_tx) => Err(T::integrate(spi, dma_tx, dma_rx)),
          },
          Err(dma_rx) => match dma_tx {
            Ok(dma_tx) | Err(dma_tx) => Err(T::integrate(spi, dma_tx, dma_rx)),
          },
        }
      }
      CompleteState::Rx(spi, dma_tx, _) => {
        let dma_rx = unsafe { advance.dma_rx };
        match dma_rx {
          Ok(dma_rx) => match dma_tx {
            Ok(dma_tx) => Ok(Async::Ready(T::integrate(spi, dma_tx, dma_rx))),
            Err(dma_tx) => Err(T::integrate(spi, dma_tx, dma_rx)),
          },
          Err(dma_rx) => match dma_tx {
            Ok(dma_tx) | Err(dma_tx) => Err(T::integrate(spi, dma_tx, dma_rx)),
          },
        }
      }
      CompleteState::Poisoned => unsafe { mem::unreachable() },
    }
  }
}

impl<T> SpiDmaTxComplete<T>
where
  T: SpiDma,
{
  #[inline(always)]
  fn new(
    spi: T::Spi,
    dma_tx: RoutineFuture<T::DmaTx, T::DmaTx>,
    dma_rx: T::DmaRx,
  ) -> Self {
    SpiDmaTxComplete(TxCompleteState::Tx(spi, dma_tx, dma_rx))
  }
}

impl<T> Future for SpiDmaTxComplete<T>
where
  T: SpiDma,
{
  type Item = T;
  type Error = T;

  fn poll(&mut self) -> Poll<T, T> {
    let dma_tx = match self.0 {
      TxCompleteState::Tx(_, ref mut dma_tx, _) => match dma_tx.poll() {
        Ok(Async::NotReady) => return Ok(Async::NotReady),
        Ok(Async::Ready(dma_tx)) => Ok(dma_tx),
        Err(dma_tx) => Err(dma_tx),
      },
      TxCompleteState::Poisoned => panic!("cannot poll a future twice"),
    };
    match mem::replace(&mut self.0, TxCompleteState::Poisoned) {
      TxCompleteState::Tx(spi, _, dma_rx) => match dma_tx {
        Ok(dma_tx) => Ok(Async::Ready(T::integrate(spi, dma_tx, dma_rx))),
        Err(dma_tx) => Err(T::integrate(spi, dma_tx, dma_rx)),
      },
      TxCompleteState::Poisoned => unsafe { mem::unreachable() },
    }
  }
}

#[doc(hidden)]
mod imp {
  use peripherals::dma::Dma;
  use peripherals::spi::Spi;

  pub trait SpiDma
  where
    Self: Sized,
  {
    type Spi: Spi;
    type DmaTx: Dma;
    type DmaRx: Dma;

    fn _integrate(
      spi: Self::Spi,
      dma_tx: Self::DmaTx,
      dma_rx: Self::DmaRx,
    ) -> Self;

    fn _disintegrate(self) -> (Self::Spi, Self::DmaTx, Self::DmaRx);

    fn _spi(&self) -> &Self::Spi;
    fn _dma_tx(&self) -> &Self::DmaTx;
    fn _dma_rx(&self) -> &Self::DmaRx;
  }
}

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
macro impl_spi_dma(
  $doc:expr,
  $name:ident,
  $driver:ident,
  $mod_name:ident,
  $spi:ident,
  $dma_tx:ident,
  $dma_rx:ident,
) {
  pub use self::$mod_name::$driver as $name;

  #[doc = $doc]
  pub mod $mod_name {
    use super::imp;

    #[doc = $doc]
    pub struct $driver {
      spi: $spi::Driver,
      dma_tx: $dma_tx::Driver,
      dma_rx: $dma_rx::Driver,
    }

    impl imp::SpiDma for $driver {
      type Spi = $spi::Driver;
      type DmaTx = $dma_tx::Driver;
      type DmaRx = $dma_rx::Driver;

      #[inline(always)]
      fn _integrate(
        spi: Self::Spi,
        dma_tx: Self::DmaTx,
        dma_rx: Self::DmaRx,
      ) -> Self {
        Self {
          spi,
          dma_tx,
          dma_rx,
        }
      }

      #[inline(always)]
      fn _disintegrate(self) -> (Self::Spi, Self::DmaTx, Self::DmaRx) {
        (self.spi, self.dma_tx, self.dma_rx)
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
  }
}

impl_spi_dma! {
  "SPI1 over DMA1",
  Spi1Dma1,
  Driver,
  spi1_dma1,
  spi1,
  dma1_ch3,
  dma1_ch2,
}

#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
impl_spi_dma! {
  "SPI1 over DMA2",
  Spi1Dma2,
  Driver,
  spi1_dma2,
  spi1,
  dma2_ch4,
  dma2_ch3,
}

impl_spi_dma! {
  "SPI2 over DMA1",
  Spi2Dma1,
  Driver,
  spi2_dma1,
  spi2,
  dma1_ch5,
  dma1_ch4,
}

impl_spi_dma! {
  "SPI3 over DMA2",
  Spi3Dma2,
  Driver,
  spi3_dma2,
  spi3,
  dma2_ch2,
  dma2_ch1,
}
