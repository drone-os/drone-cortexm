//! Direct memory access controller.

use drone::thread::RoutineFuture;
use reg::{dma1, dma2};
use reg::prelude::*;

/// Generic DMA.
#[allow(missing_docs)]
pub trait Dma: Sized {
  /// Concrete DMA items.
  type Items;

  type Ccr: Reg<Sbt>;
  type Cmar: Reg<Sbt>;
  type Cndtr: Reg<Sbt>;
  type Cpar: Reg<Sbt>;
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  type CselrCs: RegField<Sbt>;
  type IfcrCgif: RegField<Sbt>;
  type IfcrChtif: RegField<Sbt>;
  type IfcrCtcif: RegField<Sbt>;
  type IfcrCteif: RegField<Sbt>;
  type IsrGif: RegField<Sbt>;
  type IsrHtif: RegField<Sbt>;
  type IsrTcif: RegField<Sbt>;
  type IsrTeif: RegField<Sbt>;

  /// Composes a new `Dma` from pieces.
  fn compose(items: Self::Items) -> Self;

  /// Decomposes the `Dma` into pieces.
  fn decompose(self) -> Self::Items;

  fn ccr(&self) -> &Self::Ccr;
  fn cmar(&self) -> &Self::Cmar;
  fn cndtr(&self) -> &Self::Cndtr;
  fn cpar(&self) -> &Self::Cpar;
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  fn cselr_cs(&self) -> &Self::CselrCs;
  fn ifcr_cgif(&self) -> &Self::IfcrCgif;
  fn ifcr_chtif(&self) -> &Self::IfcrChtif;
  fn ifcr_ctcif(&self) -> &Self::IfcrCtcif;
  fn ifcr_cteif(&self) -> &Self::IfcrCteif;
  fn isr_gif(&self) -> &Self::IsrGif;
  fn isr_htif(&self) -> &Self::IsrHtif;
  fn isr_tcif(&self) -> &Self::IsrTcif;
  fn isr_teif(&self) -> &Self::IsrTeif;

  /// Returns a future, which resolves on DMA transfer complete event.
  fn transfer_complete<T: Thread>(
    self,
    thread: &T,
  ) -> RoutineFuture<Self, Self>;

  /// Returns a future, which resolves on DMA half transfer event.
  fn half_transfer<T: Thread>(self, thread: &T) -> RoutineFuture<Self, Self>;
}

impl<I> Dma for I
where
  I: imp::Dma,
  I: Send + 'static,
  <I::IfcrCgif as RegField<Sbt>>::Reg: WReg<Sbt> + RegBitBand<Sbt>,
  <I::IsrGif as RegField<Sbt>>::Reg: RReg<Sbt> + RegBitBand<Sbt>,
{
  type Items = I::Items;
  type Ccr = I::Ccr;
  type Cmar = I::Cmar;
  type Cndtr = I::Cndtr;
  type Cpar = I::Cpar;
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  type CselrCs = I::CselrCs;
  type IfcrCgif = I::IfcrCgif;
  type IfcrChtif = I::IfcrChtif;
  type IfcrCtcif = I::IfcrCtcif;
  type IfcrCteif = I::IfcrCteif;
  type IsrGif = I::IsrGif;
  type IsrHtif = I::IsrHtif;
  type IsrTcif = I::IsrTcif;
  type IsrTeif = I::IsrTeif;

  #[inline(always)]
  fn compose(items: Self::Items) -> Self {
    Self::_compose(items)
  }

  #[inline(always)]
  fn decompose(self) -> Self::Items {
    self._decompose()
  }

  #[inline(always)]
  fn ccr(&self) -> &Self::Ccr {
    self._ccr()
  }

  #[inline(always)]
  fn cmar(&self) -> &Self::Cmar {
    self._cmar()
  }

  #[inline(always)]
  fn cndtr(&self) -> &Self::Cndtr {
    self._cndtr()
  }

  #[inline(always)]
  fn cpar(&self) -> &Self::Cpar {
    self._cpar()
  }

  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  #[inline(always)]
  fn cselr_cs(&self) -> &Self::CselrCs {
    self._cselr_cs()
  }

  #[inline(always)]
  fn ifcr_cgif(&self) -> &Self::IfcrCgif {
    self._ifcr_cgif()
  }

  #[inline(always)]
  fn ifcr_chtif(&self) -> &Self::IfcrChtif {
    self._ifcr_chtif()
  }

  #[inline(always)]
  fn ifcr_ctcif(&self) -> &Self::IfcrCtcif {
    self._ifcr_ctcif()
  }

  #[inline(always)]
  fn ifcr_cteif(&self) -> &Self::IfcrCteif {
    self._ifcr_cteif()
  }

  #[inline(always)]
  fn isr_gif(&self) -> &Self::IsrGif {
    self._isr_gif()
  }

  #[inline(always)]
  fn isr_htif(&self) -> &Self::IsrHtif {
    self._isr_htif()
  }

  #[inline(always)]
  fn isr_tcif(&self) -> &Self::IsrTcif {
    self._isr_tcif()
  }

  #[inline(always)]
  fn isr_teif(&self) -> &Self::IsrTeif {
    self._isr_teif()
  }

  #[inline]
  fn transfer_complete<T: Thread>(
    self,
    thread: &T,
  ) -> RoutineFuture<Self, Self> {
    thread.future(move || {
      loop {
        if self._isr_teif().read_bit_band() {
          self._ifcr_cgif().set_bit_band();
          break Err(self);
        }
        if self._isr_tcif().read_bit_band() {
          self._ifcr_cgif().set_bit_band();
          break Ok(self);
        }
        yield;
      }
    })
  }

  #[inline]
  fn half_transfer<T: Thread>(self, thread: &T) -> RoutineFuture<Self, Self> {
    thread.future(move || {
      loop {
        if self._isr_teif().read_bit_band() {
          self._ifcr_cgif().set_bit_band();
          break Err(self);
        }
        if self._isr_htif().read_bit_band() {
          self._ifcr_cgif().set_bit_band();
          break Ok(self);
        }
        yield;
      }
    })
  }
}

#[doc(hidden)]
mod imp {
  use reg::prelude::*;

  pub trait Dma
  where
    <Self::IfcrCgif as RegField<Sbt>>::Reg: WReg<Sbt> + RegBitBand<Sbt>,
    <Self::IsrGif as RegField<Sbt>>::Reg: RReg<Sbt> + RegBitBand<Sbt>,
  {
    type Items;
    type Ccr: Reg<Sbt>;
    type Cmar: Reg<Sbt>;
    type Cndtr: Reg<Sbt>;
    type Cpar: Reg<Sbt>;
    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    type CselrCs: RegField<Sbt>;
    type IfcrCgif: WRegFieldBitBand<Sbt>;
    type IfcrChtif: RegField<Sbt, Reg = <Self::IfcrCgif as RegField<Sbt>>::Reg>
      + RegField<Sbt>;
    type IfcrCtcif: RegField<Sbt, Reg = <Self::IfcrCgif as RegField<Sbt>>::Reg>
      + RegField<Sbt>;
    type IfcrCteif: RegField<Sbt, Reg = <Self::IfcrCgif as RegField<Sbt>>::Reg>
      + RegField<Sbt>;
    type IsrGif: RegField<Sbt>;
    type IsrHtif: RegField<Sbt, Reg = <Self::IsrGif as RegField<Sbt>>::Reg>
      + RRegFieldBitBand<Sbt>;
    type IsrTcif: RegField<Sbt, Reg = <Self::IsrGif as RegField<Sbt>>::Reg>
      + RRegFieldBitBand<Sbt>;
    type IsrTeif: RegField<Sbt, Reg = <Self::IsrGif as RegField<Sbt>>::Reg>
      + RRegFieldBitBand<Sbt>;

    fn _compose(items: Self::Items) -> Self;
    fn _decompose(self) -> Self::Items;
    fn _ccr(&self) -> &Self::Ccr;
    fn _cmar(&self) -> &Self::Cmar;
    fn _cndtr(&self) -> &Self::Cndtr;
    fn _cpar(&self) -> &Self::Cpar;
    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    fn _cselr_cs(&self) -> &Self::CselrCs;
    fn _ifcr_cgif(&self) -> &Self::IfcrCgif;
    fn _ifcr_chtif(&self) -> &Self::IfcrChtif;
    fn _ifcr_ctcif(&self) -> &Self::IfcrCtcif;
    fn _ifcr_cteif(&self) -> &Self::IfcrCteif;
    fn _isr_gif(&self) -> &Self::IsrGif;
    fn _isr_htif(&self) -> &Self::IsrHtif;
    fn _isr_tcif(&self) -> &Self::IsrTcif;
    fn _isr_teif(&self) -> &Self::IsrTeif;
  }
}

#[doc(hidden)] // FIXME https://github.com/rust-lang/rust/issues/45266
macro dma_ch(
  $doc:expr,
  $name:ident,
  $doc_items:expr,
  $name_items:ident,
  $ccr_ty:ident,
  $cmar_ty:ident,
  $cndtr_ty:ident,
  $cpar_ty:ident,
  $cs_ty:ident,
  $cgif_ty:ident,
  $chtif_ty:ident,
  $ctcif_ty:ident,
  $cteif_ty:ident,
  $gif_ty:ident,
  $htif_ty:ident,
  $tcif_ty:ident,
  $teif_ty:ident,
  $dma:ident,
  $dma_ccr:ident,
  $dma_cmar:ident,
  $dma_cndtr:ident,
  $dma_cpar:ident,
  $dma_cselr:ident,
  $dma_ifcr:ident,
  $dma_isr:ident,
  $dma_cselr_cs:ident,
  $dma_ifcr_cgif:ident,
  $dma_ifcr_chtif:ident,
  $dma_ifcr_ctcif:ident,
  $dma_ifcr_cteif:ident,
  $dma_isr_gif:ident,
  $dma_isr_htif:ident,
  $dma_isr_tcif:ident,
  $dma_isr_teif:ident,
  $cs:ident,
  $cgif:ident,
  $chtif:ident,
  $ctcif:ident,
  $cteif:ident,
  $gif:ident,
  $htif:ident,
  $tcif:ident,
  $teif:ident,
) {
  #[doc = $doc]
  pub struct $name {
    ccr: $dma::$ccr_ty<Sbt>,
    cmar: $dma::$cmar_ty<Sbt>,
    cndtr: $dma::$cndtr_ty<Sbt>,
    cpar: $dma::$cpar_ty<Sbt>,
    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    cselr_cs: $dma::cselr::$cs_ty<Sbt>,
    ifcr_cgif: $dma::ifcr::$cgif_ty<Sbt>,
    ifcr_chtif: $dma::ifcr::$chtif_ty<Sbt>,
    ifcr_ctcif: $dma::ifcr::$ctcif_ty<Sbt>,
    ifcr_cteif: $dma::ifcr::$cteif_ty<Sbt>,
    isr_gif: $dma::isr::$gif_ty<Sbt>,
    isr_htif: $dma::isr::$htif_ty<Sbt>,
    isr_tcif: $dma::isr::$tcif_ty<Sbt>,
    isr_teif: $dma::isr::$teif_ty<Sbt>,
  }

  #[doc = $doc_items]
  #[allow(missing_docs)]
  pub struct $name_items {
    pub $dma_ccr: $dma::$ccr_ty<Sbt>,
    pub $dma_cmar: $dma::$cmar_ty<Sbt>,
    pub $dma_cndtr: $dma::$cndtr_ty<Sbt>,
    pub $dma_cpar: $dma::$cpar_ty<Sbt>,
    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    pub $dma_cselr_cs: $dma::cselr::$cs_ty<Sbt>,
    pub $dma_ifcr_cgif: $dma::ifcr::$cgif_ty<Sbt>,
    pub $dma_ifcr_chtif: $dma::ifcr::$chtif_ty<Sbt>,
    pub $dma_ifcr_ctcif: $dma::ifcr::$ctcif_ty<Sbt>,
    pub $dma_ifcr_cteif: $dma::ifcr::$cteif_ty<Sbt>,
    pub $dma_isr_gif: $dma::isr::$gif_ty<Sbt>,
    pub $dma_isr_htif: $dma::isr::$htif_ty<Sbt>,
    pub $dma_isr_tcif: $dma::isr::$tcif_ty<Sbt>,
    pub $dma_isr_teif: $dma::isr::$teif_ty<Sbt>,
  }

  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  /// Composes a new `Dma` from pieces.
  pub macro $name($bindings:ident) {
    $crate::peripherals::dma::Dma::compose(
      $crate::peripherals::dma::$name_items {
        $dma_ccr: $bindings.$dma_ccr,
        $dma_cmar: $bindings.$dma_cmar,
        $dma_cndtr: $bindings.$dma_cndtr,
        $dma_cpar: $bindings.$dma_cpar,
        $dma_cselr_cs: $bindings.$dma_cselr.$cs,
        $dma_ifcr_cgif: $bindings.$dma_ifcr.$cgif,
        $dma_ifcr_chtif: $bindings.$dma_ifcr.$chtif,
        $dma_ifcr_ctcif: $bindings.$dma_ifcr.$ctcif,
        $dma_ifcr_cteif: $bindings.$dma_ifcr.$cteif,
        $dma_isr_gif: $bindings.$dma_isr.$gif,
        $dma_isr_htif: $bindings.$dma_isr.$htif,
        $dma_isr_tcif: $bindings.$dma_isr.$tcif,
        $dma_isr_teif: $bindings.$dma_isr.$teif,
      }
    )
  }

  #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6")))]
  /// Composes a new `Dma` from pieces.
  pub macro $name($bindings:ident) {
    $crate::peripherals::dma::Dma::compose(
      $crate::peripherals::dma::$name_items {
        $dma_ccr: $bindings.$dma_ccr,
        $dma_cmar: $bindings.$dma_cmar,
        $dma_cndtr: $bindings.$dma_cndtr,
        $dma_cpar: $bindings.$dma_cpar,
        $dma_ifcr_cgif: $bindings.$dma_ifcr.$cgif,
        $dma_ifcr_chtif: $bindings.$dma_ifcr.$chtif,
        $dma_ifcr_ctcif: $bindings.$dma_ifcr.$ctcif,
        $dma_ifcr_cteif: $bindings.$dma_ifcr.$cteif,
        $dma_isr_gif: $bindings.$dma_isr.$gif,
        $dma_isr_htif: $bindings.$dma_isr.$htif,
        $dma_isr_tcif: $bindings.$dma_isr.$tcif,
        $dma_isr_teif: $bindings.$dma_isr.$teif,
      }
    )
  }

  impl imp::Dma for $name {
    type Items = $name_items;
    type Ccr = $dma::$ccr_ty<Sbt>;
    type Cmar = $dma::$cmar_ty<Sbt>;
    type Cndtr = $dma::$cndtr_ty<Sbt>;
    type Cpar = $dma::$cpar_ty<Sbt>;
    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    type CselrCs = $dma::cselr::$cs_ty<Sbt>;
    type IfcrCgif = $dma::ifcr::$cgif_ty<Sbt>;
    type IfcrChtif = $dma::ifcr::$chtif_ty<Sbt>;
    type IfcrCtcif = $dma::ifcr::$ctcif_ty<Sbt>;
    type IfcrCteif = $dma::ifcr::$cteif_ty<Sbt>;
    type IsrGif = $dma::isr::$gif_ty<Sbt>;
    type IsrHtif = $dma::isr::$htif_ty<Sbt>;
    type IsrTcif = $dma::isr::$tcif_ty<Sbt>;
    type IsrTeif = $dma::isr::$teif_ty<Sbt>;

    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    #[inline(always)]
    fn _compose(items: Self::Items) -> Self {
      Self {
        ccr: items.$dma_ccr,
        cmar: items.$dma_cmar,
        cndtr: items.$dma_cndtr,
        cpar: items.$dma_cpar,
        cselr_cs: items.$dma_cselr_cs,
        ifcr_cgif: items.$dma_ifcr_cgif,
        ifcr_chtif: items.$dma_ifcr_chtif,
        ifcr_ctcif: items.$dma_ifcr_ctcif,
        ifcr_cteif: items.$dma_ifcr_cteif,
        isr_gif: items.$dma_isr_gif,
        isr_htif: items.$dma_isr_htif,
        isr_tcif: items.$dma_isr_tcif,
        isr_teif: items.$dma_isr_teif,
      }
    }

    #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                  feature = "stm32l4x3", feature = "stm32l4x5",
                  feature = "stm32l4x6")))]
    #[inline(always)]
    fn _compose(items: Self::Items) -> Self {
      Self {
        ccr: items.$dma_ccr,
        cmar: items.$dma_cmar,
        cndtr: items.$dma_cndtr,
        cpar: items.$dma_cpar,
        ifcr_cgif: items.$dma_ifcr_cgif,
        ifcr_chtif: items.$dma_ifcr_chtif,
        ifcr_ctcif: items.$dma_ifcr_ctcif,
        ifcr_cteif: items.$dma_ifcr_cteif,
        isr_gif: items.$dma_isr_gif,
        isr_htif: items.$dma_isr_htif,
        isr_tcif: items.$dma_isr_tcif,
        isr_teif: items.$dma_isr_teif,
      }
    }

    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    #[inline(always)]
    fn _decompose(self) -> Self::Items {
      Self::Items {
        $dma_ccr: self.ccr,
        $dma_cmar: self.cmar,
        $dma_cndtr: self.cndtr,
        $dma_cpar: self.cpar,
        $dma_cselr_cs: self.cselr_cs,
        $dma_ifcr_cgif: self.ifcr_cgif,
        $dma_ifcr_chtif: self.ifcr_chtif,
        $dma_ifcr_ctcif: self.ifcr_ctcif,
        $dma_ifcr_cteif: self.ifcr_cteif,
        $dma_isr_gif: self.isr_gif,
        $dma_isr_htif: self.isr_htif,
        $dma_isr_tcif: self.isr_tcif,
        $dma_isr_teif: self.isr_teif,
      }
    }

    #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                  feature = "stm32l4x3", feature = "stm32l4x5",
                  feature = "stm32l4x6")))]
    #[inline(always)]
    fn _decompose(self) -> Self::Items {
      Self::Items {
        $dma_ccr: self.ccr,
        $dma_cmar: self.cmar,
        $dma_cndtr: self.cndtr,
        $dma_cpar: self.cpar,
        $dma_ifcr_cgif: self.ifcr_cgif,
        $dma_ifcr_chtif: self.ifcr_chtif,
        $dma_ifcr_ctcif: self.ifcr_ctcif,
        $dma_ifcr_cteif: self.ifcr_cteif,
        $dma_isr_gif: self.isr_gif,
        $dma_isr_htif: self.isr_htif,
        $dma_isr_tcif: self.isr_tcif,
        $dma_isr_teif: self.isr_teif,
      }
    }

    #[inline(always)]
    fn _ccr(&self) -> &Self::Ccr {
      &self.ccr
    }

    #[inline(always)]
    fn _cmar(&self) -> &Self::Cmar {
      &self.cmar
    }

    #[inline(always)]
    fn _cndtr(&self) -> &Self::Cndtr {
      &self.cndtr
    }

    #[inline(always)]
    fn _cpar(&self) -> &Self::Cpar {
      &self.cpar
    }

    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    #[inline(always)]
    fn _cselr_cs(&self) -> &Self::CselrCs {
      &self.cselr_cs
    }

    #[inline(always)]
    fn _ifcr_cgif(&self) -> &Self::IfcrCgif {
      &self.ifcr_cgif
    }

    #[inline(always)]
    fn _ifcr_chtif(&self) -> &Self::IfcrChtif {
      &self.ifcr_chtif
    }

    #[inline(always)]
    fn _ifcr_ctcif(&self) -> &Self::IfcrCtcif {
      &self.ifcr_ctcif
    }

    #[inline(always)]
    fn _ifcr_cteif(&self) -> &Self::IfcrCteif {
      &self.ifcr_cteif
    }

    #[inline(always)]
    fn _isr_gif(&self) -> &Self::IsrGif {
      &self.isr_gif
    }

    #[inline(always)]
    fn _isr_htif(&self) -> &Self::IsrHtif {
      &self.isr_htif
    }

    #[inline(always)]
    fn _isr_tcif(&self) -> &Self::IsrTcif {
      &self.isr_tcif
    }

    #[inline(always)]
    fn _isr_teif(&self) -> &Self::IsrTeif {
      &self.isr_teif
    }
  }
}

dma_ch! {
  "DMA1 Channel 1.",
  Dma1Ch1,
  "DMA1 Channel 1 items.",
  Dma1Ch1Items,
  Ccr1,
  Cmar1,
  Cndtr1,
  Cpar1,
  C1S,
  Cgif1,
  Chtif1,
  Ctcif1,
  Cteif1,
  Gif1,
  Htif1,
  Tcif1,
  Teif1,
  dma1,
  dma1_ccr1,
  dma1_cmar1,
  dma1_cndtr1,
  dma1_cpar1,
  dma1_cselr,
  dma1_ifcr,
  dma1_isr,
  dma1_cselr_c1s,
  dma1_ifcr_cgif1,
  dma1_ifcr_chtif1,
  dma1_ifcr_ctcif1,
  dma1_ifcr_cteif1,
  dma1_isr_gif1,
  dma1_isr_htif1,
  dma1_isr_tcif1,
  dma1_isr_teif1,
  c1s,
  cgif1,
  chtif1,
  ctcif1,
  cteif1,
  gif1,
  htif1,
  tcif1,
  teif1,
}

dma_ch! {
  "DMA1 Channel 2.",
  Dma1Ch2,
  "DMA1 Channel 2 items.",
  Dma1Ch2Items,
  Ccr2,
  Cmar2,
  Cndtr2,
  Cpar2,
  C2S,
  Cgif2,
  Chtif2,
  Ctcif2,
  Cteif2,
  Gif2,
  Htif2,
  Tcif2,
  Teif2,
  dma1,
  dma1_ccr2,
  dma1_cmar2,
  dma1_cndtr2,
  dma1_cpar2,
  dma1_cselr,
  dma1_ifcr,
  dma1_isr,
  dma1_cselr_c2s,
  dma1_ifcr_cgif2,
  dma1_ifcr_chtif2,
  dma1_ifcr_ctcif2,
  dma1_ifcr_cteif2,
  dma1_isr_gif2,
  dma1_isr_htif2,
  dma1_isr_tcif2,
  dma1_isr_teif2,
  c2s,
  cgif2,
  chtif2,
  ctcif2,
  cteif2,
  gif2,
  htif2,
  tcif2,
  teif2,
}

dma_ch! {
  "DMA1 Channel 3.",
  Dma1Ch3,
  "DMA1 Channel 3 items.",
  Dma1Ch3Items,
  Ccr3,
  Cmar3,
  Cndtr3,
  Cpar3,
  C3S,
  Cgif3,
  Chtif3,
  Ctcif3,
  Cteif3,
  Gif3,
  Htif3,
  Tcif3,
  Teif3,
  dma1,
  dma1_ccr3,
  dma1_cmar3,
  dma1_cndtr3,
  dma1_cpar3,
  dma1_cselr,
  dma1_ifcr,
  dma1_isr,
  dma1_cselr_c3s,
  dma1_ifcr_cgif3,
  dma1_ifcr_chtif3,
  dma1_ifcr_ctcif3,
  dma1_ifcr_cteif3,
  dma1_isr_gif3,
  dma1_isr_htif3,
  dma1_isr_tcif3,
  dma1_isr_teif3,
  c3s,
  cgif3,
  chtif3,
  ctcif3,
  cteif3,
  gif3,
  htif3,
  tcif3,
  teif3,
}

dma_ch! {
  "DMA1 Channel 4.",
  Dma1Ch4,
  "DMA1 Channel 4 items.",
  Dma1Ch4Items,
  Ccr4,
  Cmar4,
  Cndtr4,
  Cpar4,
  C4S,
  Cgif4,
  Chtif4,
  Ctcif4,
  Cteif4,
  Gif4,
  Htif4,
  Tcif4,
  Teif4,
  dma1,
  dma1_ccr4,
  dma1_cmar4,
  dma1_cndtr4,
  dma1_cpar4,
  dma1_cselr,
  dma1_ifcr,
  dma1_isr,
  dma1_cselr_c4s,
  dma1_ifcr_cgif4,
  dma1_ifcr_chtif4,
  dma1_ifcr_ctcif4,
  dma1_ifcr_cteif4,
  dma1_isr_gif4,
  dma1_isr_htif4,
  dma1_isr_tcif4,
  dma1_isr_teif4,
  c4s,
  cgif4,
  chtif4,
  ctcif4,
  cteif4,
  gif4,
  htif4,
  tcif4,
  teif4,
}

dma_ch! {
  "DMA1 Channel 5.",
  Dma1Ch5,
  "DMA1 Channel 5 items.",
  Dma1Ch5Items,
  Ccr5,
  Cmar5,
  Cndtr5,
  Cpar5,
  C5S,
  Cgif5,
  Chtif5,
  Ctcif5,
  Cteif5,
  Gif5,
  Htif5,
  Tcif5,
  Teif5,
  dma1,
  dma1_ccr5,
  dma1_cmar5,
  dma1_cndtr5,
  dma1_cpar5,
  dma1_cselr,
  dma1_ifcr,
  dma1_isr,
  dma1_cselr_c5s,
  dma1_ifcr_cgif5,
  dma1_ifcr_chtif5,
  dma1_ifcr_ctcif5,
  dma1_ifcr_cteif5,
  dma1_isr_gif5,
  dma1_isr_htif5,
  dma1_isr_tcif5,
  dma1_isr_teif5,
  c5s,
  cgif5,
  chtif5,
  ctcif5,
  cteif5,
  gif5,
  htif5,
  tcif5,
  teif5,
}

dma_ch! {
  "DMA1 Channel 6.",
  Dma1Ch6,
  "DMA1 Channel 6 items.",
  Dma1Ch6Items,
  Ccr6,
  Cmar6,
  Cndtr6,
  Cpar6,
  C6S,
  Cgif6,
  Chtif6,
  Ctcif6,
  Cteif6,
  Gif6,
  Htif6,
  Tcif6,
  Teif6,
  dma1,
  dma1_ccr6,
  dma1_cmar6,
  dma1_cndtr6,
  dma1_cpar6,
  dma1_cselr,
  dma1_ifcr,
  dma1_isr,
  dma1_cselr_c6s,
  dma1_ifcr_cgif6,
  dma1_ifcr_chtif6,
  dma1_ifcr_ctcif6,
  dma1_ifcr_cteif6,
  dma1_isr_gif6,
  dma1_isr_htif6,
  dma1_isr_tcif6,
  dma1_isr_teif6,
  c6s,
  cgif6,
  chtif6,
  ctcif6,
  cteif6,
  gif6,
  htif6,
  tcif6,
  teif6,
}

dma_ch! {
  "DMA1 Channel 7.",
  Dma1Ch7,
  "DMA1 Channel 7 items.",
  Dma1Ch7Items,
  Ccr7,
  Cmar7,
  Cndtr7,
  Cpar7,
  C7S,
  Cgif7,
  Chtif7,
  Ctcif7,
  Cteif7,
  Gif7,
  Htif7,
  Tcif7,
  Teif7,
  dma1,
  dma1_ccr7,
  dma1_cmar7,
  dma1_cndtr7,
  dma1_cpar7,
  dma1_cselr,
  dma1_ifcr,
  dma1_isr,
  dma1_cselr_c7s,
  dma1_ifcr_cgif7,
  dma1_ifcr_chtif7,
  dma1_ifcr_ctcif7,
  dma1_ifcr_cteif7,
  dma1_isr_gif7,
  dma1_isr_htif7,
  dma1_isr_tcif7,
  dma1_isr_teif7,
  c7s,
  cgif7,
  chtif7,
  ctcif7,
  cteif7,
  gif7,
  htif7,
  tcif7,
  teif7,
}

dma_ch! {
  "DMA2 Channel 1.",
  Dma2Ch1,
  "DMA2 Channel 1 items.",
  Dma2Ch1Items,
  Ccr1,
  Cmar1,
  Cndtr1,
  Cpar1,
  C1S,
  Cgif1,
  Chtif1,
  Ctcif1,
  Cteif1,
  Gif1,
  Htif1,
  Tcif1,
  Teif1,
  dma2,
  dma2_ccr1,
  dma2_cmar1,
  dma2_cndtr1,
  dma2_cpar1,
  dma2_cselr,
  dma2_ifcr,
  dma2_isr,
  dma2_cselr_c1s,
  dma2_ifcr_cgif1,
  dma2_ifcr_chtif1,
  dma2_ifcr_ctcif1,
  dma2_ifcr_cteif1,
  dma2_isr_gif1,
  dma2_isr_htif1,
  dma2_isr_tcif1,
  dma2_isr_teif1,
  c1s,
  cgif1,
  chtif1,
  ctcif1,
  cteif1,
  gif1,
  htif1,
  tcif1,
  teif1,
}

dma_ch! {
  "DMA2 Channel 2.",
  Dma2Ch2,
  "DMA2 Channel 2 items.",
  Dma2Ch2Items,
  Ccr2,
  Cmar2,
  Cndtr2,
  Cpar2,
  C2S,
  Cgif2,
  Chtif2,
  Ctcif2,
  Cteif2,
  Gif2,
  Htif2,
  Tcif2,
  Teif2,
  dma2,
  dma2_ccr2,
  dma2_cmar2,
  dma2_cndtr2,
  dma2_cpar2,
  dma2_cselr,
  dma2_ifcr,
  dma2_isr,
  dma2_cselr_c2s,
  dma2_ifcr_cgif2,
  dma2_ifcr_chtif2,
  dma2_ifcr_ctcif2,
  dma2_ifcr_cteif2,
  dma2_isr_gif2,
  dma2_isr_htif2,
  dma2_isr_tcif2,
  dma2_isr_teif2,
  c2s,
  cgif2,
  chtif2,
  ctcif2,
  cteif2,
  gif2,
  htif2,
  tcif2,
  teif2,
}

dma_ch! {
  "DMA2 Channel 3.",
  Dma2Ch3,
  "DMA2 Channel 3 items.",
  Dma2Ch3Items,
  Ccr3,
  Cmar3,
  Cndtr3,
  Cpar3,
  C3S,
  Cgif3,
  Chtif3,
  Ctcif3,
  Cteif3,
  Gif3,
  Htif3,
  Tcif3,
  Teif3,
  dma2,
  dma2_ccr3,
  dma2_cmar3,
  dma2_cndtr3,
  dma2_cpar3,
  dma2_cselr,
  dma2_ifcr,
  dma2_isr,
  dma2_cselr_c3s,
  dma2_ifcr_cgif3,
  dma2_ifcr_chtif3,
  dma2_ifcr_ctcif3,
  dma2_ifcr_cteif3,
  dma2_isr_gif3,
  dma2_isr_htif3,
  dma2_isr_tcif3,
  dma2_isr_teif3,
  c3s,
  cgif3,
  chtif3,
  ctcif3,
  cteif3,
  gif3,
  htif3,
  tcif3,
  teif3,
}

dma_ch! {
  "DMA2 Channel 4.",
  Dma2Ch4,
  "DMA2 Channel 4 items.",
  Dma2Ch4Items,
  Ccr4,
  Cmar4,
  Cndtr4,
  Cpar4,
  C4S,
  Cgif4,
  Chtif4,
  Ctcif4,
  Cteif4,
  Gif4,
  Htif4,
  Tcif4,
  Teif4,
  dma2,
  dma2_ccr4,
  dma2_cmar4,
  dma2_cndtr4,
  dma2_cpar4,
  dma2_cselr,
  dma2_ifcr,
  dma2_isr,
  dma2_cselr_c4s,
  dma2_ifcr_cgif4,
  dma2_ifcr_chtif4,
  dma2_ifcr_ctcif4,
  dma2_ifcr_cteif4,
  dma2_isr_gif4,
  dma2_isr_htif4,
  dma2_isr_tcif4,
  dma2_isr_teif4,
  c4s,
  cgif4,
  chtif4,
  ctcif4,
  cteif4,
  gif4,
  htif4,
  tcif4,
  teif4,
}

dma_ch! {
  "DMA2 Channel 5.",
  Dma2Ch5,
  "DMA2 Channel 5 items.",
  Dma2Ch5Items,
  Ccr5,
  Cmar5,
  Cndtr5,
  Cpar5,
  C5S,
  Cgif5,
  Chtif5,
  Ctcif5,
  Cteif5,
  Gif5,
  Htif5,
  Tcif5,
  Teif5,
  dma2,
  dma2_ccr5,
  dma2_cmar5,
  dma2_cndtr5,
  dma2_cpar5,
  dma2_cselr,
  dma2_ifcr,
  dma2_isr,
  dma2_cselr_c5s,
  dma2_ifcr_cgif5,
  dma2_ifcr_chtif5,
  dma2_ifcr_ctcif5,
  dma2_ifcr_cteif5,
  dma2_isr_gif5,
  dma2_isr_htif5,
  dma2_isr_tcif5,
  dma2_isr_teif5,
  c5s,
  cgif5,
  chtif5,
  ctcif5,
  cteif5,
  gif5,
  htif5,
  tcif5,
  teif5,
}

#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
dma_ch! {
  "DMA2 Channel 6.",
  Dma2Ch6,
  "DMA2 Channel 6 items.",
  Dma2Ch6Items,
  Ccr6,
  Cmar6,
  Cndtr6,
  Cpar6,
  C6S,
  Cgif6,
  Chtif6,
  Ctcif6,
  Cteif6,
  Gif6,
  Htif6,
  Tcif6,
  Teif6,
  dma2,
  dma2_ccr6,
  dma2_cmar6,
  dma2_cndtr6,
  dma2_cpar6,
  dma2_cselr,
  dma2_ifcr,
  dma2_isr,
  dma2_cselr_c6s,
  dma2_ifcr_cgif6,
  dma2_ifcr_chtif6,
  dma2_ifcr_ctcif6,
  dma2_ifcr_cteif6,
  dma2_isr_gif6,
  dma2_isr_htif6,
  dma2_isr_tcif6,
  dma2_isr_teif6,
  c6s,
  cgif6,
  chtif6,
  ctcif6,
  cteif6,
  gif6,
  htif6,
  tcif6,
  teif6,
}

#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
dma_ch! {
  "DMA2 Channel 7.",
  Dma2Ch7,
  "DMA2 Channel 7 items.",
  Dma2Ch7Items,
  Ccr7,
  Cmar7,
  Cndtr7,
  Cpar7,
  C7S,
  Cgif7,
  Chtif7,
  Ctcif7,
  Cteif7,
  Gif7,
  Htif7,
  Tcif7,
  Teif7,
  dma2,
  dma2_ccr7,
  dma2_cmar7,
  dma2_cndtr7,
  dma2_cpar7,
  dma2_cselr,
  dma2_ifcr,
  dma2_isr,
  dma2_cselr_c7s,
  dma2_ifcr_cgif7,
  dma2_ifcr_chtif7,
  dma2_ifcr_ctcif7,
  dma2_ifcr_cteif7,
  dma2_isr_gif7,
  dma2_isr_htif7,
  dma2_isr_tcif7,
  dma2_isr_teif7,
  c7s,
  cgif7,
  chtif7,
  ctcif7,
  cteif7,
  gif7,
  htif7,
  tcif7,
  teif7,
}
