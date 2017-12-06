//! Direct memory access controller.

use drone::thread::RoutineFuture;
use reg::{dma1, dma2};
use reg::prelude::*;

/// Generic DMA.
#[allow(missing_docs)]
pub trait Dma
where
  Self: Sized,
{
  type Ccr: Reg<Srt>;
  type Cmar: Reg<Srt>;
  type Cndtr: Reg<Srt>;
  type Cpar: Reg<Srt>;
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  type CselrCs: RegField<Srt>;
  type IfcrCgif: RegField<Srt>;
  type IfcrChtif: RegField<Srt>;
  type IfcrCtcif: RegField<Srt>;
  type IfcrCteif: RegField<Srt>;
  type IsrGif: RegField<Srt>;
  type IsrHtif: RegField<Srt>;
  type IsrTcif: RegField<Srt>;
  type IsrTeif: RegField<Srt>;

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
  fn transfer_complete<T>(self, thread: &T) -> RoutineFuture<Self, Self>
  where
    T: Thread;

  /// Returns a future, which resolves on DMA half transfer event.
  fn half_transfer<T>(self, thread: &T) -> RoutineFuture<Self, Self>
  where
    T: Thread;
}

impl<I> Dma for I
where
  I: imp::Dma,
  I: Send + 'static,
  <I::IfcrCgif as RegField<Srt>>::Reg: WReg<Srt> + RegBitBand<Srt>,
  <I::IsrHtif as RegField<Srt>>::Reg: RReg<Srt> + RegBitBand<Srt>,
  <I::IsrTcif as RegField<Srt>>::Reg: RReg<Srt> + RegBitBand<Srt>,
  <I::IsrTeif as RegField<Srt>>::Reg: RReg<Srt> + RegBitBand<Srt>,
{
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
  fn transfer_complete<T>(self, thread: &T) -> RoutineFuture<Self, Self>
  where
    T: Thread,
  {
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
  fn half_transfer<T>(self, thread: &T) -> RoutineFuture<Self, Self>
  where
    T: Thread,
  {
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
    <Self::IfcrCgif as RegField<Srt>>::Reg: WReg<Srt> + RegBitBand<Srt>,
    <Self::IsrHtif as RegField<Srt>>::Reg: RReg<Srt> + RegBitBand<Srt>,
    <Self::IsrTcif as RegField<Srt>>::Reg: RReg<Srt> + RegBitBand<Srt>,
    <Self::IsrTeif as RegField<Srt>>::Reg: RReg<Srt> + RegBitBand<Srt>,
  {
    type Ccr: Reg<Srt>;
    type Cmar: Reg<Srt>;
    type Cndtr: Reg<Srt>;
    type Cpar: Reg<Srt>;
    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    type CselrCs: RegField<Srt>;
    type IfcrCgif: WRegFieldBitBand<Srt>;
    type IfcrChtif: RegField<Srt>;
    type IfcrCtcif: RegField<Srt>;
    type IfcrCteif: RegField<Srt>;
    type IsrGif: RegField<Srt>;
    type IsrHtif: RRegFieldBitBand<Srt>;
    type IsrTcif: RRegFieldBitBand<Srt>;
    type IsrTeif: RRegFieldBitBand<Srt>;

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
macro impl_dma_ch(
  $doc:expr,
  $name:ident,
  $driver:ident,
  $builder:ident,
  $build:ident,
  $mod_name:ident,
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
  pub use self::$mod_name::$driver as $name;

  #[doc = $doc]
  pub mod $mod_name {
    pub use self::$build as $driver;

    use super::imp;
    use reg::prelude::*;

    #[doc = $doc]
    pub struct $driver {
      ccr: $dma::$ccr_ty<Srt>,
      cmar: $dma::$cmar_ty<Srt>,
      cndtr: $dma::$cndtr_ty<Srt>,
      cpar: $dma::$cpar_ty<Srt>,
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      cselr_cs: $dma::cselr::$cs_ty<Srt>,
      ifcr_cgif: $dma::ifcr::$cgif_ty<Srt>,
      ifcr_chtif: $dma::ifcr::$chtif_ty<Srt>,
      ifcr_ctcif: $dma::ifcr::$ctcif_ty<Srt>,
      ifcr_cteif: $dma::ifcr::$cteif_ty<Srt>,
      isr_gif: $dma::isr::$gif_ty<Srt>,
      isr_htif: $dma::isr::$htif_ty<Srt>,
      isr_tcif: $dma::isr::$tcif_ty<Srt>,
      isr_teif: $dma::isr::$teif_ty<Srt>,
    }

    #[doc = $doc]
    #[allow(missing_docs)]
    pub struct $builder {
      pub $dma_ccr: $dma::$ccr_ty<Srt>,
      pub $dma_cmar: $dma::$cmar_ty<Srt>,
      pub $dma_cndtr: $dma::$cndtr_ty<Srt>,
      pub $dma_cpar: $dma::$cpar_ty<Srt>,
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      pub $dma_cselr_cs: $dma::cselr::$cs_ty<Srt>,
      pub $dma_ifcr_cgif: $dma::ifcr::$cgif_ty<Srt>,
      pub $dma_ifcr_chtif: $dma::ifcr::$chtif_ty<Srt>,
      pub $dma_ifcr_ctcif: $dma::ifcr::$ctcif_ty<Srt>,
      pub $dma_ifcr_cteif: $dma::ifcr::$cteif_ty<Srt>,
      pub $dma_isr_gif: $dma::isr::$gif_ty<Srt>,
      pub $dma_isr_htif: $dma::isr::$htif_ty<Srt>,
      pub $dma_isr_tcif: $dma::isr::$tcif_ty<Srt>,
      pub $dma_isr_teif: $dma::isr::$teif_ty<Srt>,
    }

    impl $builder {
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      /// Creates a new `Driver`.
      #[inline(always)]
      pub fn build(self) -> $driver {
        $driver {
          ccr: self.$dma_ccr,
          cmar: self.$dma_cmar,
          cndtr: self.$dma_cndtr,
          cpar: self.$dma_cpar,
          cselr_cs: self.$dma_cselr_cs,
          ifcr_cgif: self.$dma_ifcr_cgif,
          ifcr_chtif: self.$dma_ifcr_chtif,
          ifcr_ctcif: self.$dma_ifcr_ctcif,
          ifcr_cteif: self.$dma_ifcr_cteif,
          isr_gif: self.$dma_isr_gif,
          isr_htif: self.$dma_isr_htif,
          isr_tcif: self.$dma_isr_tcif,
          isr_teif: self.$dma_isr_teif,
        }
      }

      #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                    feature = "stm32l4x3", feature = "stm32l4x5",
                    feature = "stm32l4x6")))]
      /// Creates a new `Driver`.
      #[inline(always)]
      pub fn build(self) -> $driver {
        $driver {
          ccr: self.$dma_ccr,
          cmar: self.$dma_cmar,
          cndtr: self.$dma_cndtr,
          cpar: self.$dma_cpar,
          ifcr_cgif: self.$dma_ifcr_cgif,
          ifcr_chtif: self.$dma_ifcr_chtif,
          ifcr_ctcif: self.$dma_ifcr_ctcif,
          ifcr_cteif: self.$dma_ifcr_cteif,
          isr_gif: self.$dma_isr_gif,
          isr_htif: self.$dma_isr_htif,
          isr_tcif: self.$dma_isr_tcif,
          isr_teif: self.$dma_isr_teif,
        }
      }
    }

    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    #[doc = $doc]
    pub macro $build($bindings:ident) {
      $crate::peripherals::dma::$mod_name::$builder {
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
      }.build()
    }

    #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                  feature = "stm32l4x3", feature = "stm32l4x5",
                  feature = "stm32l4x6")))]
    #[doc = $doc]
    pub macro $build($bindings:ident) {
      $crate::peripherals::dma::$mod_name::$builder {
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
      }.build()
    }

    impl imp::Dma for $driver {
      type Ccr = $dma::$ccr_ty<Srt>;
      type Cmar = $dma::$cmar_ty<Srt>;
      type Cndtr = $dma::$cndtr_ty<Srt>;
      type Cpar = $dma::$cpar_ty<Srt>;
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      type CselrCs = $dma::cselr::$cs_ty<Srt>;
      type IfcrCgif = $dma::ifcr::$cgif_ty<Srt>;
      type IfcrChtif = $dma::ifcr::$chtif_ty<Srt>;
      type IfcrCtcif = $dma::ifcr::$ctcif_ty<Srt>;
      type IfcrCteif = $dma::ifcr::$cteif_ty<Srt>;
      type IsrGif = $dma::isr::$gif_ty<Srt>;
      type IsrHtif = $dma::isr::$htif_ty<Srt>;
      type IsrTcif = $dma::isr::$tcif_ty<Srt>;
      type IsrTeif = $dma::isr::$teif_ty<Srt>;

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
}

impl_dma_ch! {
  "DMA1 Channel 1.",
  Dma1Ch1,
  Driver,
  Builder,
  build,
  dma1_ch1,
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

impl_dma_ch! {
  "DMA1 Channel 2.",
  Dma1Ch2,
  Driver,
  Builder,
  build,
  dma1_ch2,
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

impl_dma_ch! {
  "DMA1 Channel 3.",
  Dma1Ch3,
  Driver,
  Builder,
  build,
  dma1_ch3,
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

impl_dma_ch! {
  "DMA1 Channel 4.",
  Dma1Ch4,
  Driver,
  Builder,
  build,
  dma1_ch4,
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

impl_dma_ch! {
  "DMA1 Channel 5.",
  Dma1Ch5,
  Driver,
  Builder,
  build,
  dma1_ch5,
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

impl_dma_ch! {
  "DMA1 Channel 6.",
  Dma1Ch6,
  Driver,
  Builder,
  build,
  dma1_ch6,
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

impl_dma_ch! {
  "DMA1 Channel 7.",
  Dma1Ch7,
  Driver,
  Builder,
  build,
  dma1_ch7,
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

impl_dma_ch! {
  "DMA2 Channel 1.",
  Dma2Ch1,
  Driver,
  Builder,
  build,
  dma2_ch1,
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

impl_dma_ch! {
  "DMA2 Channel 2.",
  Dma2Ch2,
  Driver,
  Builder,
  build,
  dma2_ch2,
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

impl_dma_ch! {
  "DMA2 Channel 3.",
  Dma2Ch3,
  Driver,
  Builder,
  build,
  dma2_ch3,
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

impl_dma_ch! {
  "DMA2 Channel 4.",
  Dma2Ch4,
  Driver,
  Builder,
  build,
  dma2_ch4,
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

impl_dma_ch! {
  "DMA2 Channel 5.",
  Dma2Ch5,
  Driver,
  Builder,
  build,
  dma2_ch5,
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
impl_dma_ch! {
  "DMA2 Channel 6.",
  Dma2Ch6,
  Driver,
  Builder,
  build,
  dma2_ch6,
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
impl_dma_ch! {
  "DMA2 Channel 7.",
  Dma2Ch7,
  Driver,
  Builder,
  build,
  dma2_ch7,
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
