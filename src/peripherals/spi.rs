//! Serial peripheral interface.

use reg::{spi1, spi2, spi3};
use reg::prelude::*;

/// Generic SPI.
#[allow(missing_docs)]
pub trait Spi: Sized {
  /// Concrete SPI input items.
  type Input;

  /// Concrete SPI output items.
  type Output;

  type Cr1: Reg<Fbt>;
  type Cr2: Reg<Fbt>;
  type Crcpr: Reg<Sbt>;
  type Dr: Reg<Sbt>;
  type Rxcrcr: Reg<Sbt>;
  type Sr: Reg<Sbt>;
  type Txcrcr: Reg<Sbt>;

  /// Composes a new `Spi` from pieces.
  fn compose(input: Self::Input) -> Self;

  /// Decomposes the `Spi` into pieces.
  fn decompose(self) -> Self::Output;

  fn cr1(&self) -> &Self::Cr1;
  fn cr2(&self) -> &Self::Cr2;
  fn crcpr(&self) -> &Self::Crcpr;
  fn dr(&self) -> &Self::Dr;
  fn rxcrcr(&self) -> &Self::Rxcrcr;
  fn sr(&self) -> &Self::Sr;
  fn txcrcr(&self) -> &Self::Txcrcr;

  /// Moves `self` into `f` while `SPE` is cleared, and then sets `SPE`.
  fn spe_for<F, R>(self, cr1_val: <Self::Cr1 as Reg<Fbt>>::Val, f: F) -> R
  where
    F: FnOnce(Self) -> R;

  /// Moves `self` into `f` while `TXDMAEN` is cleared, and then sets `TXDMAEN`.
  fn txdmaen_for<F, R>(self, cr2_val: <Self::Cr2 as Reg<Fbt>>::Val, f: F) -> R
  where
    F: FnOnce(Self) -> R;
}

impl<I> Spi for I
where
  I: imp::Spi,
{
  type Input = I::Input;
  type Output = I::Output;
  type Cr1 = I::Cr1;
  type Cr2 = I::Cr2;
  type Crcpr = I::Crcpr;
  type Dr = I::Dr;
  type Rxcrcr = I::Rxcrcr;
  type Sr = I::Sr;
  type Txcrcr = I::Txcrcr;

  #[inline(always)]
  fn compose(input: Self::Input) -> Self {
    Self::_compose(input)
  }

  #[inline(always)]
  fn decompose(self) -> Self::Output {
    self._decompose()
  }

  #[inline(always)]
  fn cr1(&self) -> &Self::Cr1 {
    self._cr1()
  }

  #[inline(always)]
  fn cr2(&self) -> &Self::Cr2 {
    self._cr2()
  }

  #[inline(always)]
  fn crcpr(&self) -> &Self::Crcpr {
    self._crcpr()
  }

  #[inline(always)]
  fn dr(&self) -> &Self::Dr {
    self._dr()
  }

  #[inline(always)]
  fn rxcrcr(&self) -> &Self::Rxcrcr {
    self._rxcrcr()
  }

  #[inline(always)]
  fn sr(&self) -> &Self::Sr {
    self._sr()
  }

  #[inline(always)]
  fn txcrcr(&self) -> &Self::Txcrcr {
    self._txcrcr()
  }

  #[inline]
  fn spe_for<F, R>(
    mut self,
    mut cr1_val: <Self::Cr1 as Reg<Fbt>>::Val,
    f: F,
  ) -> R
  where
    F: FnOnce(Self) -> R,
  {
    let cr1 = self._cr1_mut().fork();
    let cr1_spe = self._cr1_spe_mut().fork();
    cr1_spe.clear(&mut cr1_val);
    cr1.store_val(cr1_val);
    let result = f(self);
    cr1_spe.set(&mut cr1_val);
    cr1.store_val(cr1_val);
    result
  }

  #[inline]
  fn txdmaen_for<F, R>(
    mut self,
    mut cr2_val: <Self::Cr2 as Reg<Fbt>>::Val,
    f: F,
  ) -> R
  where
    F: FnOnce(Self) -> R,
  {
    let cr2 = self._cr2_mut().fork();
    let cr2_txdmaen = self._cr2_txdmaen_mut().fork();
    cr2_txdmaen.clear(&mut cr2_val);
    cr2.store_val(cr2_val);
    let result = f(self);
    cr2_txdmaen.set(&mut cr2_val);
    cr2.store_val(cr2_val);
    result
  }
}

#[doc(hidden)]
mod imp {
  use reg::prelude::*;

  pub trait Spi {
    type Input;
    type Output;
    type Cr1: RReg<Fbt> + RegFork + for<'a> WRegShared<'a, Fbt>;
    type Cr2: RReg<Fbt> + RegFork + for<'a> WRegShared<'a, Fbt>;
    type Crcpr: Reg<Sbt>;
    type Dr: Reg<Sbt>;
    type Rxcrcr: Reg<Sbt>;
    type Sr: Reg<Sbt>;
    type Txcrcr: Reg<Sbt>;
    type Cr1Spe: RegField<Fbt, Reg = Self::Cr1> + RegFork + WRegFieldBit<Fbt>;
    type Cr2Txdmaen: RegField<Fbt, Reg = Self::Cr2>
      + RegFork
      + WRegFieldBit<Fbt>;

    fn _compose(input: Self::Input) -> Self;
    fn _decompose(self) -> Self::Output;
    fn _cr1(&self) -> &Self::Cr1;
    fn _cr2(&self) -> &Self::Cr2;
    fn _crcpr(&self) -> &Self::Crcpr;
    fn _dr(&self) -> &Self::Dr;
    fn _rxcrcr(&self) -> &Self::Rxcrcr;
    fn _sr(&self) -> &Self::Sr;
    fn _txcrcr(&self) -> &Self::Txcrcr;
    fn _cr1_mut(&mut self) -> &mut Self::Cr1;
    fn _cr2_mut(&mut self) -> &mut Self::Cr2;
    fn _cr1_spe_mut(&mut self) -> &mut Self::Cr1Spe;
    fn _cr2_txdmaen_mut(&mut self) -> &mut Self::Cr2Txdmaen;
  }
}

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
macro spi(
  $doc:expr,
  $name:ident,
  $doc_items:expr,
  $name_items:ident,
  $spi:ident,
  $spi_cr1:ident,
  $spi_cr2:ident,
  $spi_crcpr:ident,
  $spi_dr:ident,
  $spi_rxcrcr:ident,
  $spi_sr:ident,
  $spi_txcrcr:ident,
) {
  #[doc = $doc]
  pub struct $name {
    cr1: $spi::Cr1<Fbt>,
    cr2: $spi::Cr2<Fbt>,
    crcpr: $spi::Crcpr<Sbt>,
    dr: $spi::Dr<Sbt>,
    rxcrcr: $spi::Rxcrcr<Sbt>,
    sr: $spi::Sr<Sbt>,
    txcrcr: $spi::Txcrcr<Sbt>,
  }

  #[doc = $doc_items]
  #[allow(missing_docs)]
  pub struct $name_items<T: RegTag> {
    pub $spi_cr1: $spi::Cr1<T>,
    pub $spi_cr2: $spi::Cr2<T>,
    pub $spi_crcpr: $spi::Crcpr<Sbt>,
    pub $spi_dr: $spi::Dr<Sbt>,
    pub $spi_rxcrcr: $spi::Rxcrcr<Sbt>,
    pub $spi_sr: $spi::Sr<Sbt>,
    pub $spi_txcrcr: $spi::Txcrcr<Sbt>,
  }

  /// Composes a new `Spi` from pieces.
  pub macro $name($bindings:ident) {
    $crate::peripherals::spi::Spi::compose(
      $crate::peripherals::spi::$name_items {
        $spi_cr1: $bindings.$spi_cr1,
        $spi_cr2: $bindings.$spi_cr2,
        $spi_crcpr: $bindings.$spi_crcpr,
        $spi_dr: $bindings.$spi_dr,
        $spi_rxcrcr: $bindings.$spi_rxcrcr,
        $spi_sr: $bindings.$spi_sr,
        $spi_txcrcr: $bindings.$spi_txcrcr,
      }
    )
  }

  impl imp::Spi for $name {
    type Input = $name_items<Sbt>;
    type Output = $name_items<Fbt>;
    type Cr1 = $spi::Cr1<Fbt>;
    type Cr2 = $spi::Cr2<Fbt>;
    type Crcpr = $spi::Crcpr<Sbt>;
    type Dr = $spi::Dr<Sbt>;
    type Rxcrcr = $spi::Rxcrcr<Sbt>;
    type Sr = $spi::Sr<Sbt>;
    type Txcrcr = $spi::Txcrcr<Sbt>;
    type Cr1Spe = $spi::cr1::Spe<Fbt>;
    type Cr2Txdmaen = $spi::cr2::Txdmaen<Fbt>;

    #[inline(always)]
    fn _compose(input: Self::Input) -> Self {
      Self {
        cr1: input.$spi_cr1.into(),
        cr2: input.$spi_cr2.into(),
        crcpr: input.$spi_crcpr,
        dr: input.$spi_dr,
        rxcrcr: input.$spi_rxcrcr,
        sr: input.$spi_sr,
        txcrcr: input.$spi_txcrcr,
      }
    }

    #[inline(always)]
    fn _decompose(self) -> Self::Output {
      Self::Output {
        $spi_cr1: self.cr1,
        $spi_cr2: self.cr2,
        $spi_crcpr: self.crcpr,
        $spi_dr: self.dr,
        $spi_rxcrcr: self.rxcrcr,
        $spi_sr: self.sr,
        $spi_txcrcr: self.txcrcr,
      }
    }

    #[inline(always)]
    fn _cr1(&self) -> &Self::Cr1 {
      &self.cr1
    }

    #[inline(always)]
    fn _cr2(&self) -> &Self::Cr2 {
      &self.cr2
    }

    #[inline(always)]
    fn _crcpr(&self) -> &Self::Crcpr {
      &self.crcpr
    }

    #[inline(always)]
    fn _dr(&self) -> &Self::Dr {
      &self.dr
    }

    #[inline(always)]
    fn _rxcrcr(&self) -> &Self::Rxcrcr {
      &self.rxcrcr
    }

    #[inline(always)]
    fn _sr(&self) -> &Self::Sr {
      &self.sr
    }

    #[inline(always)]
    fn _txcrcr(&self) -> &Self::Txcrcr {
      &self.txcrcr
    }

    #[inline(always)]
    fn _cr1_mut(&mut self) -> &mut Self::Cr1 {
      &mut self.cr1
    }

    #[inline(always)]
    fn _cr2_mut(&mut self) -> &mut Self::Cr2 {
      &mut self.cr2
    }

    #[inline(always)]
    fn _cr1_spe_mut(&mut self) -> &mut Self::Cr1Spe {
      &mut self.cr1.spe
    }

    #[inline(always)]
    fn _cr2_txdmaen_mut(&mut self) -> &mut Self::Cr2Txdmaen {
      &mut self.cr2.txdmaen
    }
  }
}

spi! {
  "SPI1.",
  Spi1,
  "SPI1 items.",
  Spi1Items,
  spi1,
  spi1_cr1,
  spi1_cr2,
  spi1_crcpr,
  spi1_dr,
  spi1_rxcrcr,
  spi1_sr,
  spi1_txcrcr,
}

spi! {
  "SPI2.",
  Spi2,
  "SPI2 items.",
  Spi2Items,
  spi2,
  spi2_cr1,
  spi2_cr2,
  spi2_crcpr,
  spi2_dr,
  spi2_rxcrcr,
  spi2_sr,
  spi2_txcrcr,
}

spi! {
  "SPI3.",
  Spi3,
  "SPI3 items.",
  Spi3Items,
  spi3,
  spi3_cr1,
  spi3_cr2,
  spi3_crcpr,
  spi3_dr,
  spi3_rxcrcr,
  spi3_sr,
  spi3_txcrcr,
}
