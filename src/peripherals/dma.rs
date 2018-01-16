//! Direct memory access controller.

use drone_core::peripheral::{PeripheralDevice, PeripheralTokens};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use reg::{dma1, dma2};
use reg::prelude::*;
#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x6"))]
use thread::irq::{IrqDma1Ch1, IrqDma1Ch2, IrqDma1Ch3, IrqDma1Ch4, IrqDma1Ch5,
                  IrqDma1Ch6, IrqDma1Ch7, IrqDma2Ch1, IrqDma2Ch2, IrqDma2Ch3,
                  IrqDma2Ch4, IrqDma2Ch5, IrqDma2Ch6, IrqDma2Ch7};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x3",
          feature = "stm32l4x5"))]
use thread::irq::{IrqDma1Channel1 as IrqDma1Ch1,
                  IrqDma1Channel2 as IrqDma1Ch2,
                  IrqDma1Channel3 as IrqDma1Ch3,
                  IrqDma1Channel4 as IrqDma1Ch4,
                  IrqDma1Channel5 as IrqDma1Ch5,
                  IrqDma1Channel6 as IrqDma1Ch6,
                  IrqDma1Channel7 as IrqDma1Ch7,
                  IrqDma2Channel1 as IrqDma2Ch1,
                  IrqDma2Channel2 as IrqDma2Ch2, IrqDma2Channel3 as IrqDma2Ch3};
#[cfg(any(feature = "stm32f107", feature = "stm32l4x3",
          feature = "stm32l4x5"))]
use thread::irq::{IrqDma2Channel4 as IrqDma2Ch4, IrqDma2Channel5 as IrqDma2Ch5};
#[cfg(any(feature = "stm32l4x3", feature = "stm32l4x5"))]
use thread::irq::{IrqDma2Channel6 as IrqDma2Ch6, IrqDma2Channel7 as IrqDma2Ch7};
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103"))]
use thread::irq::IrqDma2Channel45 as IrqDma2Ch4;
#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103"))]
use thread::irq::IrqDma2Channel45 as IrqDma2Ch5;
use thread::prelude::*;

/// Generic DMA.
pub struct Dma<T: DmaTokens>(T);

/// Generic DMA tokens.
#[allow(missing_docs)]
pub trait DmaTokens: PeripheralTokens<InputTokens = Self> {
  type Irq: IrqToken<Ltt>;
  type Ccr: for<'a> RwRegSharedRef<'a, Srt>;
  type CmarVal: RegVal<Raw = u32>;
  type Cmar: Reg<Srt, Val = Self::CmarVal> + for<'a> RwRegSharedRef<'a, Srt>;
  type CmarMa: RegField<Srt, Reg = Self::Cmar>
    + RRegFieldBits<Srt>
    + WRwRegFieldBitsShared<Srt>;
  type CndtrVal: RegVal<Raw = u32>;
  type Cndtr: Reg<Srt, Val = Self::CndtrVal> + for<'a> RwRegSharedRef<'a, Srt>;
  type CndtrNdt: RegField<Srt, Reg = Self::Cndtr>
    + RRegFieldBits<Srt>
    + WRwRegFieldBitsShared<Srt>;
  type CparVal: RegVal<Raw = u32>;
  type Cpar: Reg<Srt, Val = Self::CparVal> + for<'a> RwRegSharedRef<'a, Srt>;
  type CparPa: RegField<Srt, Reg = Self::Cpar>
    + RRegFieldBits<Srt>
    + WRwRegFieldBitsShared<Srt>;
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  type Cselr: for<'a> RwRegSharedRef<'a, Srt>;
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  type CselrCs: RegField<Srt, Reg = Self::Cselr>
    + RRegFieldBits<Srt>
    + WRwRegFieldBitsShared<Srt>;
  type Ifcr: WoReg<Srt> + for<'a> WRegShared<'a, Srt> + RegBitBand<Srt>;
  type IfcrCgif: RegField<Srt, Reg = Self::Ifcr>
    + WoWoRegFieldBit<Srt>
    + WRegFieldBitBand<Srt>;
  type IfcrChtif: RegField<Srt, Reg = Self::Ifcr>
    + WoWoRegFieldBit<Srt>
    + WRegFieldBitBand<Srt>;
  type IfcrCtcif: RegField<Srt, Reg = Self::Ifcr>
    + WoWoRegFieldBit<Srt>
    + WRegFieldBitBand<Srt>;
  type IfcrCteif: RegField<Srt, Reg = Self::Ifcr>
    + WoWoRegFieldBit<Srt>
    + WRegFieldBitBand<Srt>;
  type Isr: RoReg<Srt> + RegBitBand<Srt>;
  type IsrGif: RegField<Srt, Reg = Self::Isr> + RRegFieldBitBand<Srt>;
  type IsrHtif: RegField<Srt, Reg = Self::Isr> + RRegFieldBitBand<Srt>;
  type IsrTcif: RegField<Srt, Reg = Self::Isr> + RRegFieldBitBand<Srt>;
  type IsrTeif: RegField<Srt, Reg = Self::Isr> + RRegFieldBitBand<Srt>;

  fn irq(&self) -> Self::Irq;
  fn ccr(&self) -> &Self::Ccr;
  fn cmar(&self) -> &Self::Cmar;
  fn cmar_ma(&self) -> &Self::CmarMa;
  fn cndtr(&self) -> &Self::Cndtr;
  fn cndtr_ndt(&self) -> &Self::CndtrNdt;
  fn cpar(&self) -> &Self::Cpar;
  fn cpar_pa(&self) -> &Self::CparPa;
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
}

impl<T: DmaTokens> PeripheralDevice<T> for Dma<T> {
  #[inline(always)]
  fn from_tokens(tokens: T::InputTokens) -> Self {
    Dma(tokens)
  }

  #[inline(always)]
  fn into_tokens(self) -> T {
    self.0
  }
}

#[allow(missing_docs)]
impl<T: DmaTokens> Dma<T> {
  #[inline(always)]
  pub fn irq(&self) -> T::Irq {
    self.0.irq()
  }

  #[inline(always)]
  pub fn ccr(&self) -> &T::Ccr {
    self.0.ccr()
  }

  #[inline(always)]
  pub fn cmar(&self) -> &T::Cmar {
    self.0.cmar()
  }

  #[inline(always)]
  pub fn cndtr(&self) -> &T::Cndtr {
    self.0.cndtr()
  }

  #[inline(always)]
  pub fn cpar(&self) -> &T::Cpar {
    self.0.cpar()
  }

  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  #[inline(always)]
  pub fn cselr_cs(&self) -> &T::CselrCs {
    self.0.cselr_cs()
  }

  #[inline(always)]
  pub fn ifcr_cgif(&self) -> &T::IfcrCgif {
    self.0.ifcr_cgif()
  }

  #[inline(always)]
  pub fn ifcr_chtif(&self) -> &T::IfcrChtif {
    self.0.ifcr_chtif()
  }

  #[inline(always)]
  pub fn ifcr_ctcif(&self) -> &T::IfcrCtcif {
    self.0.ifcr_ctcif()
  }

  #[inline(always)]
  pub fn ifcr_cteif(&self) -> &T::IfcrCteif {
    self.0.ifcr_cteif()
  }

  #[inline(always)]
  pub fn isr_gif(&self) -> &T::IsrGif {
    self.0.isr_gif()
  }

  #[inline(always)]
  pub fn isr_htif(&self) -> &T::IsrHtif {
    self.0.isr_htif()
  }

  #[inline(always)]
  pub fn isr_tcif(&self) -> &T::IsrTcif {
    self.0.isr_tcif()
  }

  #[inline(always)]
  pub fn isr_teif(&self) -> &T::IsrTeif {
    self.0.isr_teif()
  }

  /// Returns a number of data to transfer.
  #[inline(always)]
  pub fn number_of_data(&self) -> u16 {
    self.0.cndtr_ndt().read_bits() as u16
  }

  /// Sets the number of data to transfer.
  #[inline(always)]
  pub fn set_number_of_data(&self, number: u16) {
    self.0.cndtr_ndt().write_bits(u32::from(number));
  }

  /// Returns a peripheral address.
  #[inline(always)]
  pub fn peripheral_address(&self) -> usize {
    self.0.cpar_pa().read_bits() as usize
  }

  /// Sets the peripheral address.
  #[inline(always)]
  pub fn set_peripheral_address(&self, address: usize) {
    self.0.cpar_pa().write_bits(address as u32);
  }

  /// Returns a memory address.
  #[inline(always)]
  pub fn memory_address(&self) -> usize {
    self.0.cmar_ma().read_bits() as usize
  }

  /// Sets the memory address.
  #[inline(always)]
  pub fn set_memory_address(&self, address: usize) {
    self.0.cmar_ma().write_bits(address as u32);
  }

  /// Returns a future, which resolves on DMA transfer complete event.
  pub fn transfer_complete(self) -> impl Future<Item = Self, Error = Self> {
    self.0.irq().future(move || loop {
      if self.0.isr_teif().read_bit_band() {
        self.0.ifcr_cgif().set_bit_band();
        break Err(self);
      }
      if self.0.isr_tcif().read_bit_band() {
        self.0.ifcr_cgif().set_bit_band();
        break Ok(self);
      }
      yield;
    })
  }

  /// Returns a future, which resolves on DMA half transfer event.
  pub fn half_transfer(self) -> impl Future<Item = Self, Error = Self> {
    self.0.irq().future(move || loop {
      if self.0.isr_teif().read_bit_band() {
        self.0.ifcr_cgif().set_bit_band();
        break Err(self);
      }
      if self.0.isr_htif().read_bit_band() {
        self.0.ifcr_cgif().set_bit_band();
        break Ok(self);
      }
      yield;
    })
  }
}

#[allow(unused_macros)]
macro_rules! dma_ch {
  (
    $doc:expr,
    $name:ident,
    $name_macro:ident,
    $doc_tokens:expr,
    $name_tokens:ident,
    $irq_ty:ident,
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
    $irq:ident,
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
    $cmar_path:ident,
    $cndtr_path:ident,
    $cpar_path:ident,
    $cs:ident,
    $cgif:ident,
    $chtif:ident,
    $ctcif:ident,
    $cteif:ident,
    $gif:ident,
    $htif:ident,
    $tcif:ident,
    $teif:ident,
  ) => {
    #[doc = $doc]
    pub type $name<I> = Dma<$name_tokens<I>>;

    #[doc = $doc_tokens]
    #[allow(missing_docs)]
    pub struct $name_tokens<I: $irq_ty<Ltt>> {
      pub $irq: I,
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

    #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
              feature = "stm32l4x3", feature = "stm32l4x5",
              feature = "stm32l4x6"))]
    /// Creates a new `Dma`.
    #[macro_export]
    macro_rules! $name_macro {
      ($regs:ident, $thrd:ident) => {
        $crate::peripherals::dma::Dma::from_tokens(
          $crate::peripherals::dma::$name_tokens {
            $irq: $thrd.$irq.into(),
            $dma_ccr: $regs.$dma_ccr,
            $dma_cmar: $regs.$dma_cmar,
            $dma_cndtr: $regs.$dma_cndtr,
            $dma_cpar: $regs.$dma_cpar,
            $dma_cselr_cs: $regs.$dma_cselr.$cs,
            $dma_ifcr_cgif: $regs.$dma_ifcr.$cgif,
            $dma_ifcr_chtif: $regs.$dma_ifcr.$chtif,
            $dma_ifcr_ctcif: $regs.$dma_ifcr.$ctcif,
            $dma_ifcr_cteif: $regs.$dma_ifcr.$cteif,
            $dma_isr_gif: $regs.$dma_isr.$gif,
            $dma_isr_htif: $regs.$dma_isr.$htif,
            $dma_isr_tcif: $regs.$dma_isr.$tcif,
            $dma_isr_teif: $regs.$dma_isr.$teif,
          }
        )
      }
    }

    #[cfg(not(any(feature = "stm32l4x1", feature = "stm32l4x2",
                  feature = "stm32l4x3", feature = "stm32l4x5",
                  feature = "stm32l4x6")))]
    /// Creates a new `Dma`.
    #[macro_export]
    macro_rules! $name_macro {
      ($regs:ident, $thrd:ident) => {
        $crate::peripherals::dma::Dma::from_tokens(
          $crate::peripherals::dma::$name_tokens {
            $irq: $thrd.$irq.into(),
            $dma_ccr: $regs.$dma_ccr,
            $dma_cmar: $regs.$dma_cmar,
            $dma_cndtr: $regs.$dma_cndtr,
            $dma_cpar: $regs.$dma_cpar,
            $dma_ifcr_cgif: $regs.$dma_ifcr.$cgif,
            $dma_ifcr_chtif: $regs.$dma_ifcr.$chtif,
            $dma_ifcr_ctcif: $regs.$dma_ifcr.$ctcif,
            $dma_ifcr_cteif: $regs.$dma_ifcr.$cteif,
            $dma_isr_gif: $regs.$dma_isr.$gif,
            $dma_isr_htif: $regs.$dma_isr.$htif,
            $dma_isr_tcif: $regs.$dma_isr.$tcif,
            $dma_isr_teif: $regs.$dma_isr.$teif,
          }
        )
      }
    }

    impl<I: $irq_ty<Ltt>> PeripheralTokens for $name_tokens<I> {
      // FIXME https://github.com/rust-lang/rust/issues/47385
      type InputTokens = Self;
    }

    impl<I: $irq_ty<Ltt>> DmaTokens for $name_tokens<I> {
      type Irq = I;
      type Ccr = $dma::$ccr_ty<Srt>;
      type CmarVal = $dma::$cmar_path::Val;
      type Cmar = $dma::$cmar_ty<Srt>;
      type CmarMa = $dma::$cmar_path::Ma<Srt>;
      type CndtrVal = $dma::$cndtr_path::Val;
      type Cndtr = $dma::$cndtr_ty<Srt>;
      type CndtrNdt = $dma::$cndtr_path::Ndt<Srt>;
      type CparVal = $dma::$cpar_path::Val;
      type Cpar = $dma::$cpar_ty<Srt>;
      type CparPa = $dma::$cpar_path::Pa<Srt>;
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      type Cselr = $dma::cselr::Reg<Srt>;
      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      type CselrCs = $dma::cselr::$cs_ty<Srt>;
      type Ifcr = $dma::Ifcr<Srt>;
      type IfcrCgif = $dma::ifcr::$cgif_ty<Srt>;
      type IfcrChtif = $dma::ifcr::$chtif_ty<Srt>;
      type IfcrCtcif = $dma::ifcr::$ctcif_ty<Srt>;
      type IfcrCteif = $dma::ifcr::$cteif_ty<Srt>;
      type Isr = $dma::Isr<Srt>;
      type IsrGif = $dma::isr::$gif_ty<Srt>;
      type IsrHtif = $dma::isr::$htif_ty<Srt>;
      type IsrTcif = $dma::isr::$tcif_ty<Srt>;
      type IsrTeif = $dma::isr::$teif_ty<Srt>;

      #[inline(always)]
      fn irq(&self) -> Self::Irq {
        self.$irq
      }

      #[inline(always)]
      fn ccr(&self) -> &Self::Ccr {
        &self.$dma_ccr
      }

      #[inline(always)]
      fn cmar(&self) -> &Self::Cmar {
        &self.$dma_cmar
      }

      #[inline(always)]
      fn cmar_ma(&self) -> &Self::CmarMa {
        &self.$dma_cmar.ma
      }

      #[inline(always)]
      fn cndtr(&self) -> &Self::Cndtr {
        &self.$dma_cndtr
      }

      #[inline(always)]
      fn cndtr_ndt(&self) -> &Self::CndtrNdt {
        &self.$dma_cndtr.ndt
      }

      #[inline(always)]
      fn cpar(&self) -> &Self::Cpar {
        &self.$dma_cpar
      }

      #[inline(always)]
      fn cpar_pa(&self) -> &Self::CparPa {
        &self.$dma_cpar.pa
      }

      #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
                feature = "stm32l4x3", feature = "stm32l4x5",
                feature = "stm32l4x6"))]
      #[inline(always)]
      fn cselr_cs(&self) -> &Self::CselrCs {
        &self.$dma_cselr_cs
      }

      #[inline(always)]
      fn ifcr_cgif(&self) -> &Self::IfcrCgif {
        &self.$dma_ifcr_cgif
      }

      #[inline(always)]
      fn ifcr_chtif(&self) -> &Self::IfcrChtif {
        &self.$dma_ifcr_chtif
      }

      #[inline(always)]
      fn ifcr_ctcif(&self) -> &Self::IfcrCtcif {
        &self.$dma_ifcr_ctcif
      }

      #[inline(always)]
      fn ifcr_cteif(&self) -> &Self::IfcrCteif {
        &self.$dma_ifcr_cteif
      }

      #[inline(always)]
      fn isr_gif(&self) -> &Self::IsrGif {
        &self.$dma_isr_gif
      }

      #[inline(always)]
      fn isr_htif(&self) -> &Self::IsrHtif {
        &self.$dma_isr_htif
      }

      #[inline(always)]
      fn isr_tcif(&self) -> &Self::IsrTcif {
        &self.$dma_isr_tcif
      }

      #[inline(always)]
      fn isr_teif(&self) -> &Self::IsrTeif {
        &self.$dma_isr_teif
      }
    }
  }
}

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
dma_ch! {
  "DMA1 Channel 1.",
  Dma1Ch1,
  peripheral_dma1_ch1,
  "DMA1 Channel 1 tokens.",
  Dma1Ch1Tokens,
  IrqDma1Ch1,
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
  dma1_ch1,
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
  cmar1,
  cndtr1,
  cpar1,
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

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
dma_ch! {
  "DMA1 Channel 2.",
  Dma1Ch2,
  peripheral_dma1_ch2,
  "DMA1 Channel 2 tokens.",
  Dma1Ch2Tokens,
  IrqDma1Ch2,
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
  dma1_ch2,
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
  cmar2,
  cndtr2,
  cpar2,
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

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
dma_ch! {
  "DMA1 Channel 3.",
  Dma1Ch3,
  peripheral_dma1_ch3,
  "DMA1 Channel 3 tokens.",
  Dma1Ch3Tokens,
  IrqDma1Ch3,
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
  dma1_ch3,
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
  cmar3,
  cndtr3,
  cpar3,
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

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
dma_ch! {
  "DMA1 Channel 4.",
  Dma1Ch4,
  peripheral_dma1_ch4,
  "DMA1 Channel 4 tokens.",
  Dma1Ch4Tokens,
  IrqDma1Ch4,
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
  dma1_ch4,
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
  cmar4,
  cndtr4,
  cpar4,
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

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
dma_ch! {
  "DMA1 Channel 5.",
  Dma1Ch5,
  peripheral_dma1_ch5,
  "DMA1 Channel 5 tokens.",
  Dma1Ch5Tokens,
  IrqDma1Ch5,
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
  dma1_ch5,
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
  cmar5,
  cndtr5,
  cpar5,
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

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
dma_ch! {
  "DMA1 Channel 6.",
  Dma1Ch6,
  peripheral_dma1_ch6,
  "DMA1 Channel 6 tokens.",
  Dma1Ch6Tokens,
  IrqDma1Ch6,
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
  dma1_ch6,
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
  cmar6,
  cndtr6,
  cpar6,
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

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
dma_ch! {
  "DMA1 Channel 7.",
  Dma1Ch7,
  peripheral_dma1_ch7,
  "DMA1 Channel 7 tokens.",
  Dma1Ch7Tokens,
  IrqDma1Ch7,
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
  dma1_ch7,
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
  cmar7,
  cndtr7,
  cpar7,
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

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
dma_ch! {
  "DMA2 Channel 1.",
  Dma2Ch1,
  peripheral_dma2_ch1,
  "DMA2 Channel 1 tokens.",
  Dma2Ch1Tokens,
  IrqDma2Ch1,
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
  dma2_ch1,
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
  cmar1,
  cndtr1,
  cpar1,
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

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
dma_ch! {
  "DMA2 Channel 2.",
  Dma2Ch2,
  peripheral_dma2_ch2,
  "DMA2 Channel 2 tokens.",
  Dma2Ch2Tokens,
  IrqDma2Ch2,
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
  dma2_ch2,
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
  cmar2,
  cndtr2,
  cpar2,
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

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
dma_ch! {
  "DMA2 Channel 3.",
  Dma2Ch3,
  peripheral_dma2_ch3,
  "DMA2 Channel 3 tokens.",
  Dma2Ch3Tokens,
  IrqDma2Ch3,
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
  dma2_ch3,
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
  cmar3,
  cndtr3,
  cpar3,
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

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
dma_ch! {
  "DMA2 Channel 4.",
  Dma2Ch4,
  peripheral_dma2_ch4,
  "DMA2 Channel 4 tokens.",
  Dma2Ch4Tokens,
  IrqDma2Ch4,
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
  dma2_ch4,
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
  cmar4,
  cndtr4,
  cpar4,
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

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
dma_ch! {
  "DMA2 Channel 5.",
  Dma2Ch5,
  peripheral_dma2_ch5,
  "DMA2 Channel 5 tokens.",
  Dma2Ch5Tokens,
  IrqDma2Ch5,
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
  dma2_ch5,
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
  cmar5,
  cndtr5,
  cpar5,
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
  peripheral_dma2_ch6,
  "DMA2 Channel 6 tokens.",
  Dma2Ch6Tokens,
  IrqDma2Ch6,
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
  dma2_ch6,
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
  cmar6,
  cndtr6,
  cpar6,
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
  peripheral_dma2_ch7,
  "DMA2 Channel 7 tokens.",
  Dma2Ch7Tokens,
  IrqDma2Ch7,
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
  dma2_ch7,
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
  cmar7,
  cndtr7,
  cpar7,
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
