//! Serial peripheral interface.

use reg::prelude::*;

/// Generic SPI.
#[allow(missing_docs)]
pub trait Spi
where
  Self: Sized,
{
  type Cr1: Reg<Drt>;
  type Cr2: Reg<Drt>;
  type Crcpr: Reg<Srt>;
  type Dr: Reg<Srt>;
  type Rxcrcr: Reg<Srt>;
  type Sr: Reg<Srt>;
  type Txcrcr: Reg<Srt>;

  fn cr1(&self) -> &Self::Cr1;
  fn cr2(&self) -> &Self::Cr2;
  fn crcpr(&self) -> &Self::Crcpr;
  fn dr(&self) -> &Self::Dr;
  fn rxcrcr(&self) -> &Self::Rxcrcr;
  fn sr(&self) -> &Self::Sr;
  fn txcrcr(&self) -> &Self::Txcrcr;

  /// Moves `self` into `f` while `SPE` is cleared, and then sets `SPE`.
  fn spe_for<F, R>(self, cr1: <Self::Cr1 as Reg<Drt>>::Val, f: F) -> R
  where
    F: FnOnce(Self) -> R;

  /// Moves `self` into `f` while `TXDMAEN` is cleared, and then sets `TXDMAEN`.
  fn txdmaen_for<F, R>(self, cr2: <Self::Cr2 as Reg<Drt>>::Val, f: F) -> R
  where
    F: FnOnce(Self) -> R;
}

impl<I> Spi for I
where
  I: imp::Spi,
{
  type Cr1 = I::Cr1;
  type Cr2 = I::Cr2;
  type Crcpr = I::Crcpr;
  type Dr = I::Dr;
  type Rxcrcr = I::Rxcrcr;
  type Sr = I::Sr;
  type Txcrcr = I::Txcrcr;

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
  fn spe_for<F, R>(mut self, cr1: <Self::Cr1 as Reg<Drt>>::Val, f: F) -> R
  where
    F: FnOnce(Self) -> R,
  {
    let (disable, enable) = {
      let mut cr1 = self._cr1().hold(cr1);
      let disable = Self::_clear_spe(&mut cr1).val();
      let enable = Self::_set_spe(&mut cr1).val();
      (disable, enable)
    };
    let cr1 = self._cr1_mut().clone();
    cr1.store_val(disable);
    let result = f(self);
    cr1.store_val(enable);
    result
  }

  #[inline]
  fn txdmaen_for<F, R>(mut self, cr2: <Self::Cr2 as Reg<Drt>>::Val, f: F) -> R
  where
    F: FnOnce(Self) -> R,
  {
    let (disable, enable) = {
      let mut cr2 = self._cr2().hold(cr2);
      let disable = Self::_clear_txdmaen(&mut cr2).val();
      let enable = Self::_set_txdmaen(&mut cr2).val();
      (disable, enable)
    };
    let cr2 = self._cr2_mut().clone();
    cr2.store_val(disable);
    let result = f(self);
    cr2.store_val(enable);
    result
  }
}

#[doc(hidden)]
mod imp {
  use reg::prelude::*;

  pub trait Spi {
    type Cr1: RReg<Drt> + DReg + for<'a> WRegShared<'a, Drt>;
    type Cr2: RReg<Drt> + DReg + for<'a> WRegShared<'a, Drt>;
    type Crcpr: Reg<Srt>;
    type Dr: Reg<Srt>;
    type Rxcrcr: Reg<Srt>;
    type Sr: Reg<Srt>;
    type Txcrcr: Reg<Srt>;

    fn _cr1(&self) -> &Self::Cr1;
    fn _cr2(&self) -> &Self::Cr2;
    fn _crcpr(&self) -> &Self::Crcpr;
    fn _dr(&self) -> &Self::Dr;
    fn _rxcrcr(&self) -> &Self::Rxcrcr;
    fn _sr(&self) -> &Self::Sr;
    fn _txcrcr(&self) -> &Self::Txcrcr;

    fn _cr1_mut(&mut self) -> &mut Self::Cr1;
    fn _cr2_mut(&mut self) -> &mut Self::Cr2;

    fn _set_spe<'b, 'a>(
      cr1: &'b mut <Self::Cr1 as RegRef<'a, Drt>>::Hold,
    ) -> &'b mut <Self::Cr1 as RegRef<'a, Drt>>::Hold;

    fn _clear_spe<'b, 'a>(
      cr1: &'b mut <Self::Cr1 as RegRef<'a, Drt>>::Hold,
    ) -> &'b mut <Self::Cr1 as RegRef<'a, Drt>>::Hold;

    fn _set_txdmaen<'b, 'a>(
      cr2: &'b mut <Self::Cr2 as RegRef<'a, Drt>>::Hold,
    ) -> &'b mut <Self::Cr2 as RegRef<'a, Drt>>::Hold;

    fn _clear_txdmaen<'b, 'a>(
      cr2: &'b mut <Self::Cr2 as RegRef<'a, Drt>>::Hold,
    ) -> &'b mut <Self::Cr2 as RegRef<'a, Drt>>::Hold;
  }
}

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
macro impl_spi(
  $doc:expr,
  $name:ident,
  $driver:ident,
  $builder:ident,
  $build:ident,
  $mod_name:ident,
  $spi_cr1:ident,
  $spi_cr2:ident,
  $spi_crcpr:ident,
  $spi_dr:ident,
  $spi_rxcrcr:ident,
  $spi_sr:ident,
  $spi_txcrcr:ident,
) {
  pub use self::$mod_name::$driver as $name;

  #[doc = $doc]
  pub mod $mod_name {
    pub use self::$build as $driver;

    use super::imp;
    use reg::prelude::*;
    use reg::$mod_name as spi;

    #[doc = $doc]
    pub struct $driver {
      cr1: spi::Cr1<Drt>,
      cr2: spi::Cr2<Drt>,
      crcpr: spi::Crcpr<Srt>,
      dr: spi::Dr<Srt>,
      rxcrcr: spi::Rxcrcr<Srt>,
      sr: spi::Sr<Srt>,
      txcrcr: spi::Txcrcr<Srt>,
    }

    #[doc = $doc]
    #[allow(missing_docs)]
    pub struct $builder {
      pub $spi_cr1: spi::Cr1<Srt>,
      pub $spi_cr2: spi::Cr2<Srt>,
      pub $spi_crcpr: spi::Crcpr<Srt>,
      pub $spi_dr: spi::Dr<Srt>,
      pub $spi_rxcrcr: spi::Rxcrcr<Srt>,
      pub $spi_sr: spi::Sr<Srt>,
      pub $spi_txcrcr: spi::Txcrcr<Srt>,
    }

    impl $builder {
      /// Creates a new `Driver`.
      #[inline(always)]
      pub fn build(self) -> $driver {
        $driver {
          cr1: self.$spi_cr1.upgrade(),
          cr2: self.$spi_cr2.upgrade(),
          crcpr: self.$spi_crcpr,
          dr: self.$spi_dr,
          rxcrcr: self.$spi_rxcrcr,
          sr: self.$spi_sr,
          txcrcr: self.$spi_txcrcr,
        }
      }
    }

    #[doc = $doc]
    pub macro $build($bindings:ident) {
      $crate::peripherals::spi::$mod_name::$builder {
        $spi_cr1: $bindings.$spi_cr1,
        $spi_cr2: $bindings.$spi_cr2,
        $spi_crcpr: $bindings.$spi_crcpr,
        $spi_dr: $bindings.$spi_dr,
        $spi_rxcrcr: $bindings.$spi_rxcrcr,
        $spi_sr: $bindings.$spi_sr,
        $spi_txcrcr: $bindings.$spi_txcrcr,
      }.build()
    }

    impl imp::Spi for $driver {
      type Cr1 = spi::Cr1<Drt>;
      type Cr2 = spi::Cr2<Drt>;
      type Crcpr = spi::Crcpr<Srt>;
      type Dr = spi::Dr<Srt>;
      type Rxcrcr = spi::Rxcrcr<Srt>;
      type Sr = spi::Sr<Srt>;
      type Txcrcr = spi::Txcrcr<Srt>;

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
      fn _set_spe<'b, 'a>(
        cr1: &'b mut <Self::Cr1 as RegRef<'a, Drt>>::Hold,
      ) -> &'b mut <Self::Cr1 as RegRef<'a, Drt>>::Hold {
        cr1.set_spe()
      }

      #[inline(always)]
      fn _clear_spe<'b, 'a>(
        cr1: &'b mut <Self::Cr1 as RegRef<'a, Drt>>::Hold,
      ) -> &'b mut <Self::Cr1 as RegRef<'a, Drt>>::Hold {
        cr1.clear_spe()
      }

      #[inline(always)]
      fn _set_txdmaen<'b, 'a>(
        cr2: &'b mut <Self::Cr2 as RegRef<'a, Drt>>::Hold,
      ) -> &'b mut <Self::Cr2 as RegRef<'a, Drt>>::Hold {
        cr2.set_txdmaen()
      }

      #[inline(always)]
      fn _clear_txdmaen<'b, 'a>(
        cr2: &'b mut <Self::Cr2 as RegRef<'a, Drt>>::Hold,
      ) -> &'b mut <Self::Cr2 as RegRef<'a, Drt>>::Hold {
        cr2.clear_txdmaen()
      }
    }
  }
}

impl_spi! {
  "SPI1.",
  Spi1,
  Driver,
  Builder,
  build,
  spi1,
  spi1_cr1,
  spi1_cr2,
  spi1_crcpr,
  spi1_dr,
  spi1_rxcrcr,
  spi1_sr,
  spi1_txcrcr,
}

impl_spi! {
  "SPI2.",
  Spi2,
  Driver,
  Builder,
  build,
  spi2,
  spi2_cr1,
  spi2_cr2,
  spi2_crcpr,
  spi2_dr,
  spi2_rxcrcr,
  spi2_sr,
  spi2_txcrcr,
}

impl_spi! {
  "SPI3.",
  Spi3,
  Driver,
  Builder,
  build,
  spi3,
  spi3_cr1,
  spi3_cr2,
  spi3_crcpr,
  spi3_dr,
  spi3_rxcrcr,
  spi3_sr,
  spi3_txcrcr,
}
