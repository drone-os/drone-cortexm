//! Nested vectored interrupt controller.

#[cfg(any(feature = "stm32f100", feature = "stm32f101",
          feature = "stm32f102", feature = "stm32f103",
          feature = "stm32f107", feature = "stm32l4x1",
          feature = "stm32l4x2", feature = "stm32l4x3",
          feature = "stm32l4x5", feature = "stm32l4x6"))]
use reg::nvic;
#[allow(unused_imports)]
use reg::prelude::*;

/// Nested vectored interrupt controller.
#[allow(missing_docs)]
pub struct Nvic {
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub iser0: nvic::Iser0<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub iser1: nvic::Iser1<Srt>,
  #[cfg(any(feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub iser2: nvic::Iser2<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub icer0: nvic::Icer0<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub icer1: nvic::Icer1<Srt>,
  #[cfg(any(feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub icer2: nvic::Icer2<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ispr0: nvic::Ispr0<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ispr1: nvic::Ispr1<Srt>,
  #[cfg(any(feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ispr2: nvic::Ispr2<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub icpr0: nvic::Icpr0<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub icpr1: nvic::Icpr1<Srt>,
  #[cfg(any(feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub icpr2: nvic::Icpr2<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub iabr0: nvic::Iabr0<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub iabr1: nvic::Iabr1<Srt>,
  #[cfg(any(feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub iabr2: nvic::Iabr2<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr0: nvic::Ipr0<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr1: nvic::Ipr1<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr2: nvic::Ipr2<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr3: nvic::Ipr3<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr4: nvic::Ipr4<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr5: nvic::Ipr5<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr6: nvic::Ipr6<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr7: nvic::Ipr7<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr8: nvic::Ipr8<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr9: nvic::Ipr9<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr10: nvic::Ipr10<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr11: nvic::Ipr11<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr12: nvic::Ipr12<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f101",
            feature = "stm32f102", feature = "stm32f103",
            feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr13: nvic::Ipr13<Srt>,
  #[cfg(any(feature = "stm32f100", feature = "stm32f102",
            feature = "stm32f103", feature = "stm32f107",
            feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  pub ipr14: nvic::Ipr14<Srt>,
  #[cfg(any(feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr15: nvic::Ipr15<Srt>,
  #[cfg(any(feature = "stm32f107", feature = "stm32l4x1",
            feature = "stm32l4x2", feature = "stm32l4x3",
            feature = "stm32l4x5", feature = "stm32l4x6"))]
  pub ipr16: nvic::Ipr16<Srt>,
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  pub ipr17: nvic::Ipr17<Srt>,
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  pub ipr18: nvic::Ipr18<Srt>,
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  pub ipr19: nvic::Ipr19<Srt>,
  #[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
            feature = "stm32l4x3", feature = "stm32l4x5",
            feature = "stm32l4x6"))]
  pub ipr20: nvic::Ipr20<Srt>,
}

#[cfg(any(feature = "stm32f101"))]
/// Creates a new `Nvic` driver from tokens.
#[macro_export]
macro_rules! peripheral_nvic {
  ($regs:ident) => {
    $crate::peripherals::nvic::Nvic {
      iser0: $regs.nvic_iser0,
      iser1: $regs.nvic_iser1,
      icer0: $regs.nvic_icer0,
      icer1: $regs.nvic_icer1,
      ispr0: $regs.nvic_ispr0,
      ispr1: $regs.nvic_ispr1,
      icpr0: $regs.nvic_icpr0,
      icpr1: $regs.nvic_icpr1,
      iabr0: $regs.nvic_iabr0,
      iabr1: $regs.nvic_iabr1,
      ipr0: $regs.nvic_ipr0,
      ipr1: $regs.nvic_ipr1,
      ipr2: $regs.nvic_ipr2,
      ipr3: $regs.nvic_ipr3,
      ipr4: $regs.nvic_ipr4,
      ipr5: $regs.nvic_ipr5,
      ipr6: $regs.nvic_ipr6,
      ipr7: $regs.nvic_ipr7,
      ipr8: $regs.nvic_ipr8,
      ipr9: $regs.nvic_ipr9,
      ipr10: $regs.nvic_ipr10,
      ipr11: $regs.nvic_ipr11,
      ipr12: $regs.nvic_ipr12,
      ipr13: $regs.nvic_ipr13,
    }
  }
}

#[cfg(any(feature = "stm32f100", feature = "stm32f102",
          feature = "stm32f103"))]
/// Creates a new `Nvic` driver from tokens.
#[macro_export]
macro_rules! peripheral_nvic {
  ($regs:ident) => {
    $crate::peripherals::nvic::Nvic {
      iser0: $regs.nvic_iser0,
      iser1: $regs.nvic_iser1,
      icer0: $regs.nvic_icer0,
      icer1: $regs.nvic_icer1,
      ispr0: $regs.nvic_ispr0,
      ispr1: $regs.nvic_ispr1,
      icpr0: $regs.nvic_icpr0,
      icpr1: $regs.nvic_icpr1,
      iabr0: $regs.nvic_iabr0,
      iabr1: $regs.nvic_iabr1,
      ipr0: $regs.nvic_ipr0,
      ipr1: $regs.nvic_ipr1,
      ipr2: $regs.nvic_ipr2,
      ipr3: $regs.nvic_ipr3,
      ipr4: $regs.nvic_ipr4,
      ipr5: $regs.nvic_ipr5,
      ipr6: $regs.nvic_ipr6,
      ipr7: $regs.nvic_ipr7,
      ipr8: $regs.nvic_ipr8,
      ipr9: $regs.nvic_ipr9,
      ipr10: $regs.nvic_ipr10,
      ipr11: $regs.nvic_ipr11,
      ipr12: $regs.nvic_ipr12,
      ipr13: $regs.nvic_ipr13,
      ipr14: $regs.nvic_ipr14,
    }
  }
}

#[cfg(any(feature = "stm32f107"))]
/// Creates a new `Nvic` driver from tokens.
#[macro_export]
macro_rules! peripheral_nvic {
  ($regs:ident) => {
    $crate::peripherals::nvic::Nvic {
      iser0: $regs.nvic_iser0,
      iser1: $regs.nvic_iser1,
      iser2: $regs.nvic_iser2,
      icer0: $regs.nvic_icer0,
      icer1: $regs.nvic_icer1,
      icer2: $regs.nvic_icer2,
      ispr0: $regs.nvic_ispr0,
      ispr1: $regs.nvic_ispr1,
      ispr2: $regs.nvic_ispr2,
      icpr0: $regs.nvic_icpr0,
      icpr1: $regs.nvic_icpr1,
      icpr2: $regs.nvic_icpr2,
      iabr0: $regs.nvic_iabr0,
      iabr1: $regs.nvic_iabr1,
      iabr2: $regs.nvic_iabr2,
      ipr0: $regs.nvic_ipr0,
      ipr1: $regs.nvic_ipr1,
      ipr2: $regs.nvic_ipr2,
      ipr3: $regs.nvic_ipr3,
      ipr4: $regs.nvic_ipr4,
      ipr5: $regs.nvic_ipr5,
      ipr6: $regs.nvic_ipr6,
      ipr7: $regs.nvic_ipr7,
      ipr8: $regs.nvic_ipr8,
      ipr9: $regs.nvic_ipr9,
      ipr10: $regs.nvic_ipr10,
      ipr11: $regs.nvic_ipr11,
      ipr12: $regs.nvic_ipr12,
      ipr13: $regs.nvic_ipr13,
      ipr14: $regs.nvic_ipr14,
      ipr15: $regs.nvic_ipr15,
      ipr16: $regs.nvic_ipr16,
    }
  }
}

#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2",
          feature = "stm32l4x3", feature = "stm32l4x5",
          feature = "stm32l4x6"))]
/// Creates a new `Nvic` driver from tokens.
#[macro_export]
macro_rules! peripheral_nvic {
  ($regs:ident) => {
    $crate::peripherals::nvic::Nvic {
      iser0: $regs.nvic_iser0,
      iser1: $regs.nvic_iser1,
      iser2: $regs.nvic_iser2,
      icer0: $regs.nvic_icer0,
      icer1: $regs.nvic_icer1,
      icer2: $regs.nvic_icer2,
      ispr0: $regs.nvic_ispr0,
      ispr1: $regs.nvic_ispr1,
      ispr2: $regs.nvic_ispr2,
      icpr0: $regs.nvic_icpr0,
      icpr1: $regs.nvic_icpr1,
      icpr2: $regs.nvic_icpr2,
      iabr0: $regs.nvic_iabr0,
      iabr1: $regs.nvic_iabr1,
      iabr2: $regs.nvic_iabr2,
      ipr0: $regs.nvic_ipr0,
      ipr1: $regs.nvic_ipr1,
      ipr2: $regs.nvic_ipr2,
      ipr3: $regs.nvic_ipr3,
      ipr4: $regs.nvic_ipr4,
      ipr5: $regs.nvic_ipr5,
      ipr6: $regs.nvic_ipr6,
      ipr7: $regs.nvic_ipr7,
      ipr8: $regs.nvic_ipr8,
      ipr9: $regs.nvic_ipr9,
      ipr10: $regs.nvic_ipr10,
      ipr11: $regs.nvic_ipr11,
      ipr12: $regs.nvic_ipr12,
      ipr13: $regs.nvic_ipr13,
      ipr14: $regs.nvic_ipr14,
      ipr15: $regs.nvic_ipr15,
      ipr16: $regs.nvic_ipr16,
      ipr17: $regs.nvic_ipr17,
      ipr18: $regs.nvic_ipr18,
      ipr19: $regs.nvic_ipr19,
      ipr20: $regs.nvic_ipr20,
    }
  }
}
