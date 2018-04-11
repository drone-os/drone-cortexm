//! Extended interrupts and events controller.

use drone_core::drv::Resource;
use fib::{self, Fiber};
use futures::prelude::*;
#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
use reg::afio;
#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
use reg::exti;
use reg::marker::*;
use reg::prelude::*;
#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
use reg::syscfg;
#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107",
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
use thr::int::{IntExti0, IntExti1, IntExti1510, IntExti2, IntExti3, IntExti4,
               IntExti95};
use thr::prelude::*;

/// Error returned from [`ExtiLn::stream`](ExtiLn::stream) on overflow.
#[derive(Debug, Fail)]
#[fail(display = "EXTI stream overflow.")]
pub struct ExtiLnOverflow;

/// EXTI line driver.
#[derive(Driver)]
pub struct ExtiLn<T: ExtiLnRes>(T);

/// EXTI line resource.
#[allow(missing_docs)]
pub trait ExtiLnRes: Resource {
  type Emr: SRwRegBitBand;
  type EmrMr: SRwRwRegFieldBitBand<Reg = Self::Emr>;
  type Imr: SRwRegBitBand;
  type ImrMr: SRwRwRegFieldBitBand<Reg = Self::Imr>;

  res_reg_decl!(EmrMr, emr_mr, emr_mr_mut);
  res_reg_decl!(ImrMr, imr_mr, imr_mr_mut);
}

/// Configurable EXTI line resource.
#[allow(missing_docs)]
pub trait ExtiLnConfRes: ExtiLnRes {
  type Ftsr: SRwRegBitBand;
  type FtsrFt: SRwRwRegFieldBitBand<Reg = Self::Ftsr>;
  type Pr: FRwRegBitBand;
  type PrPif: FRwRwRegFieldBitBand<Reg = Self::Pr>;
  type Rtsr: SRwRegBitBand;
  type RtsrRt: SRwRwRegFieldBitBand<Reg = Self::Rtsr>;
  type Swier: SRwRegBitBand;
  type SwierSwi: SRwRwRegFieldBitBand<Reg = Self::Swier>;

  res_reg_decl!(FtsrFt, ftsr_ft, ftsr_ft_mut);
  res_reg_decl!(PrPif, pr_pif, pr_pif_mut);
  res_reg_decl!(RtsrRt, rtsr_rt, rtsr_rt_mut);
  res_reg_decl!(SwierSwi, swier_swi, swier_swi_mut);
}

/// EXTI line resource with external interrupt support.
#[allow(missing_docs)]
pub trait ExtiLnExtRes: ExtiLnRes {
  type Int: IntToken<Ltt>;
  type Exticr: SRwRegBitBand;
  type ExticrExti: SRwRwRegFieldBits<Reg = Self::Exticr>;

  fn int(&self) -> Self::Int;

  res_reg_decl!(ExticrExti, exticr_exti, exticr_exti_mut);
}

#[allow(missing_docs)]
impl<T: ExtiLnRes> ExtiLn<T> {
  #[inline(always)]
  pub fn emr_mr(&self) -> &T::EmrMr {
    self.0.emr_mr()
  }

  #[inline(always)]
  pub fn imr_mr(&self) -> &T::ImrMr {
    self.0.imr_mr()
  }
}

#[allow(missing_docs)]
impl<T: ExtiLnConfRes> ExtiLn<T> {
  #[inline(always)]
  pub fn ftsr_ft(&self) -> &T::FtsrFt {
    self.0.ftsr_ft()
  }

  #[inline(always)]
  pub fn pr_pif(&self) -> &T::PrPif {
    self.0.pr_pif()
  }

  #[inline(always)]
  pub fn rtsr_rt(&self) -> &T::RtsrRt {
    self.0.rtsr_rt()
  }

  #[inline(always)]
  pub fn swier_swi(&self) -> &T::SwierSwi {
    self.0.swier_swi()
  }
}

#[allow(missing_docs)]
impl<T: ExtiLnExtRes> ExtiLn<T> {
  #[inline(always)]
  pub fn int(&self) -> T::Int {
    self.0.int()
  }

  #[inline(always)]
  pub fn exticr_exti(&self) -> &T::ExticrExti {
    self.0.exticr_exti()
  }
}

impl<T: ExtiLnExtRes + ExtiLnConfRes> ExtiLn<T> {
  /// Returns a future, which resolves to `Ok(())` when the event is triggered.
  pub fn add_future(&mut self) -> impl Future<Item = (), Error = !> {
    fib::add_future(self.0.int(), self.future_fib())
  }

  /// Returns a stream, which resolves to `Ok(())` each time the event is
  /// triggered. Resolves to `Err(ExtiLnOverflow)` on overflow.
  pub fn add_stream(
    &mut self,
  ) -> impl Stream<Item = (), Error = ExtiLnOverflow> {
    fib::add_stream(
      self.0.int(),
      || Err(ExtiLnOverflow),
      self.stream_fib(),
    )
  }

  /// Returns a stream, which resolves to `Ok(())` each time the event is
  /// triggered. Skips on overflow.
  pub fn add_stream_skip(&mut self) -> impl Stream<Item = (), Error = !> {
    fib::add_stream_skip(self.0.int(), self.stream_fib())
  }

  fn stream_fib<E: Send>(
    &mut self,
  ) -> impl Fiber<Input = (), Yield = Option<()>, Return = Result<Option<()>, E>>
  {
    let pif = self.0.pr_pif_mut().fork();
    fib::new(move || loop {
      if pif.read_bit_band() {
        pif.set_bit_band();
        yield Some(());
      }
      yield None;
    })
  }

  fn future_fib<E: Send>(
    &mut self,
  ) -> impl Fiber<Input = (), Yield = (), Return = Result<(), E>> {
    let pif = self.0.pr_pif_mut().fork();
    fib::new(move || loop {
      if pif.read_bit_band() {
        pif.set_bit_band();
        break Ok(());
      }
      yield;
    })
  }
}

#[allow(unused_macros)]
macro_rules! exti_line {
  (
    $doc:expr,
    $name:ident,
    $name_macro:ident,
    $doc_res:expr,
    $name_res:ident,
    $mr_ty:ident,
    $emr_path:ident,
    $imr_path:ident,
    $exti_emr:ident,
    $exti_imr:ident,
    $exti_emr_mr:ident,
    $exti_imr_mr:ident,
    $mr:ident,
    ($((
      $i_tp:ident: $int_ty:ident,
      ($($frt_i:ident)*),
      $exti_ty:ident,
      $int:ident,
      $($exticr_path:ident)::*,
      $exticr:ident,
      $exticr_exti:ident,
      $exti:ident,
    ))*),
    ($((
      ($($i_tp_c:ident: $int_ty_c:ident)*),
      $rt_tp:ident: $srt:ident $frt:ident,
      $ft_ty:ident,
      $pif_ty:ident,
      $rt_ty:ident,
      $swi_ty:ident,
      $ftsr_path:ident,
      $pr_path:ident,
      $rtsr_path:ident,
      $swier_path:ident,
      $exti_ftsr:ident,
      $exti_pr:ident,
      $exti_rtsr:ident,
      $exti_swier:ident,
      $exti_ftsr_ft:ident,
      $exti_pr_pif:ident,
      $exti_rtsr_rt:ident,
      $exti_swier_swi:ident,
      $ft:ident,
      $pif:ident,
      $rt:ident,
      $swi:ident,
    ))*),
  ) => {
    #[doc = $doc]
    pub type $name<$($i_tp,)*> = ExtiLn<$name_res<$($i_tp,)* $($frt,)*>>;

    #[doc = $doc_res]
    #[allow(missing_docs)]
    pub struct $name_res<$($i_tp: $int_ty<Ltt>,)* $($rt_tp: RegTag,)*> {
      $(
        pub $int: $i_tp,
        pub $exticr_exti: $($exticr_path)::*::$exti_ty<Srt>,
      )*
      $(
        pub $exti_ftsr_ft: exti::$ftsr_path::$ft_ty<Srt>,
        pub $exti_pr_pif: exti::$pr_path::$pif_ty<$rt_tp>,
        pub $exti_rtsr_rt: exti::$rtsr_path::$rt_ty<Srt>,
        pub $exti_swier_swi: exti::$swier_path::$swi_ty<Srt>,
      )*
      pub $exti_emr_mr: exti::$emr_path::$mr_ty<Srt>,
      pub $exti_imr_mr: exti::$imr_path::$mr_ty<Srt>,
    }

    /// Creates a new `ExtiLn`.
    #[macro_export]
    macro_rules! $name_macro {
      ($reg:ident, $thr:ident) => {
        $crate::drv::exti::ExtiLn::new(
          $crate::drv::exti::$name_res {
            $(
              $int: $thr.$int.into(),
              $exticr_exti: $reg.$exticr.$exti,
            )*
            $(
              $exti_ftsr_ft: $reg.$exti_ftsr.$ft,
              $exti_pr_pif: $reg.$exti_pr.$pif,
              $exti_rtsr_rt: $reg.$exti_rtsr.$rt,
              $exti_swier_swi: $reg.$exti_swier.$swi,
            )*
            $exti_emr_mr: $reg.$exti_emr.$mr,
            $exti_imr_mr: $reg.$exti_imr.$mr,
          }
        )
      }
    }

    impl<$($i_tp,)*> Resource for $name_res<$($i_tp,)* $($frt,)*>
    where
      $($i_tp: $int_ty<Ltt>,)*
    {
      type Source = $name_res<$($i_tp,)* $($srt,)*>;

      #[inline(always)]
      fn from_source(source: Self::Source) -> Self {
        Self {
          $(
            $int: source.$int,
            $exticr_exti: source.$exticr_exti,
          )*
          $(
            $exti_ftsr_ft: source.$exti_ftsr_ft,
            $exti_pr_pif: source.$exti_pr_pif.into(),
            $exti_rtsr_rt: source.$exti_rtsr_rt,
            $exti_swier_swi: source.$exti_swier_swi,
          )*
          $exti_emr_mr: source.$exti_emr_mr,
          $exti_imr_mr: source.$exti_imr_mr,
        }
      }
    }

    impl<$($i_tp,)*> ExtiLnRes for $name_res<$($i_tp,)* $($frt,)*>
    where
      $($i_tp: $int_ty<Ltt>,)*
    {
      type Emr = exti::$emr_path::Reg<Srt>;
      type EmrMr = exti::$emr_path::$mr_ty<Srt>;
      type Imr = exti::$imr_path::Reg<Srt>;
      type ImrMr = exti::$imr_path::$mr_ty<Srt>;

      res_reg_impl!(EmrMr, emr_mr, emr_mr_mut, $exti_emr_mr);
      res_reg_impl!(ImrMr, imr_mr, imr_mr_mut, $exti_imr_mr);
    }

    $(
      impl<$($i_tp_c,)*> ExtiLnConfRes for $name_res<$($i_tp_c,)* Frt>
      where
        $($i_tp_c: $int_ty_c<Ltt>,)*
      {
        type Ftsr = exti::$ftsr_path::Reg<Srt>;
        type FtsrFt = exti::$ftsr_path::$ft_ty<Srt>;
        type Pr = exti::$pr_path::Reg<Frt>;
        type PrPif = exti::$pr_path::$pif_ty<Frt>;
        type Rtsr = exti::$rtsr_path::Reg<Srt>;
        type RtsrRt = exti::$rtsr_path::$rt_ty<Srt>;
        type Swier = exti::$swier_path::Reg<Srt>;
        type SwierSwi = exti::$swier_path::$swi_ty<Srt>;

        res_reg_impl!(FtsrFt, ftsr_ft, ftsr_ft_mut, $exti_ftsr_ft);
        res_reg_impl!(PrPif, pr_pif, pr_pif_mut, $exti_pr_pif);
        res_reg_impl!(RtsrRt, rtsr_rt, rtsr_rt_mut, $exti_rtsr_rt);
        res_reg_impl!(SwierSwi, swier_swi, swier_swi_mut, $exti_swier_swi);
      }
    )*

    $(
      impl<$i_tp> ExtiLnExtRes for $name_res<$i_tp, $($frt_i,)*>
      where
        $i_tp: $int_ty<Ltt>,
      {
        type Int = $i_tp;
        type Exticr = $($exticr_path)::*::Reg<Srt>;
        type ExticrExti = $($exticr_path)::*::$exti_ty<Srt>;

        #[inline(always)]
        fn int(&self) -> Self::Int {
          self.$int
        }

        res_reg_impl!(ExticrExti, exticr_exti, exticr_exti_mut, $exticr_exti);
      }
    )*
  }
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 0 driver.",
  ExtiLn0,
  drv_exti_ln_0,
  "EXTI Line 0 resource.",
  ExtiLn0Res,
  Mr0,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr0,
  exti_imr_mr0,
  mr0,
  ((
    I: IntExti0,
    (Frt),
    Exti0,
    exti0,
    afio::exticr1,
    afio_exticr1,
    afio_exticr1_exti0,
    exti0,
  )),
  ((
    (I: IntExti0),
    Rt: Srt Frt,
    Tr0,
    Pr0,
    Tr0,
    Swier0,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr0,
    exti_pr_pr0,
    exti_rtsr_tr0,
    exti_swier_swier0,
    tr0,
    pr0,
    tr0,
    swier0,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 1 driver.",
  ExtiLn1,
  drv_exti_ln_1,
  "EXTI Line 1 resource.",
  ExtiLn1Res,
  Mr1,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr1,
  exti_imr_mr1,
  mr1,
  ((
    I: IntExti1,
    (Frt),
    Exti1,
    exti1,
    afio::exticr1,
    afio_exticr1,
    afio_exticr1_exti1,
    exti1,
  )),
  ((
    (I: IntExti1),
    Rt: Srt Frt,
    Tr1,
    Pr1,
    Tr1,
    Swier1,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr1,
    exti_pr_pr1,
    exti_rtsr_tr1,
    exti_swier_swier1,
    tr1,
    pr1,
    tr1,
    swier1,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 2 driver.",
  ExtiLn2,
  drv_exti_ln_2,
  "EXTI Line 2 resource.",
  ExtiLn2Res,
  Mr2,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr2,
  exti_imr_mr2,
  mr2,
  ((
    I: IntExti2,
    (Frt),
    Exti2,
    exti2,
    afio::exticr1,
    afio_exticr1,
    afio_exticr1_exti2,
    exti2,
  )),
  ((
    (I: IntExti2),
    Rt: Srt Frt,
    Tr2,
    Pr2,
    Tr2,
    Swier2,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr2,
    exti_pr_pr2,
    exti_rtsr_tr2,
    exti_swier_swier2,
    tr2,
    pr2,
    tr2,
    swier2,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 3 driver.",
  ExtiLn3,
  drv_exti_ln_3,
  "EXTI Line 3 resource.",
  ExtiLn3Res,
  Mr3,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr3,
  exti_imr_mr3,
  mr3,
  ((
    I: IntExti3,
    (Frt),
    Exti3,
    exti3,
    afio::exticr1,
    afio_exticr1,
    afio_exticr1_exti3,
    exti3,
  )),
  ((
    (I: IntExti3),
    Rt: Srt Frt,
    Tr3,
    Pr3,
    Tr3,
    Swier3,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr3,
    exti_pr_pr3,
    exti_rtsr_tr3,
    exti_swier_swier3,
    tr3,
    pr3,
    tr3,
    swier3,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 4 driver.",
  ExtiLn4,
  drv_exti_ln_4,
  "EXTI Line 4 resource.",
  ExtiLn4Res,
  Mr4,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr4,
  exti_imr_mr4,
  mr4,
  ((
    I: IntExti4,
    (Frt),
    Exti4,
    exti4,
    afio::exticr2,
    afio_exticr2,
    afio_exticr2_exti4,
    exti4,
  )),
  ((
    (I: IntExti4),
    Rt: Srt Frt,
    Tr4,
    Pr4,
    Tr4,
    Swier4,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr4,
    exti_pr_pr4,
    exti_rtsr_tr4,
    exti_swier_swier4,
    tr4,
    pr4,
    tr4,
    swier4,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 5 driver.",
  ExtiLn5,
  drv_exti_ln_5,
  "EXTI Line 5 resource.",
  ExtiLn5Res,
  Mr5,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr5,
  exti_imr_mr5,
  mr5,
  ((
    I: IntExti95,
    (Frt),
    Exti5,
    exti9_5,
    afio::exticr2,
    afio_exticr2,
    afio_exticr2_exti5,
    exti5,
  )),
  ((
    (I: IntExti95),
    Rt: Srt Frt,
    Tr5,
    Pr5,
    Tr5,
    Swier5,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr5,
    exti_pr_pr5,
    exti_rtsr_tr5,
    exti_swier_swier5,
    tr5,
    pr5,
    tr5,
    swier5,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 6 driver.",
  ExtiLn6,
  drv_exti_ln_6,
  "EXTI Line 6 resource.",
  ExtiLn6Res,
  Mr6,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr6,
  exti_imr_mr6,
  mr6,
  ((
    I: IntExti95,
    (Frt),
    Exti6,
    exti9_5,
    afio::exticr2,
    afio_exticr2,
    afio_exticr2_exti6,
    exti6,
  )),
  ((
    (I: IntExti95),
    Rt: Srt Frt,
    Tr6,
    Pr6,
    Tr6,
    Swier6,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr6,
    exti_pr_pr6,
    exti_rtsr_tr6,
    exti_swier_swier6,
    tr6,
    pr6,
    tr6,
    swier6,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 7 driver.",
  ExtiLn7,
  drv_exti_ln_7,
  "EXTI Line 7 resource.",
  ExtiLn7Res,
  Mr7,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr7,
  exti_imr_mr7,
  mr7,
  ((
    I: IntExti95,
    (Frt),
    Exti7,
    exti9_5,
    afio::exticr2,
    afio_exticr2,
    afio_exticr2_exti7,
    exti7,
  )),
  ((
    (I: IntExti95),
    Rt: Srt Frt,
    Tr7,
    Pr7,
    Tr7,
    Swier7,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr7,
    exti_pr_pr7,
    exti_rtsr_tr7,
    exti_swier_swier7,
    tr7,
    pr7,
    tr7,
    swier7,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 8 driver.",
  ExtiLn8,
  drv_exti_ln_8,
  "EXTI Line 8 resource.",
  ExtiLn8Res,
  Mr8,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr8,
  exti_imr_mr8,
  mr8,
  ((
    I: IntExti95,
    (Frt),
    Exti8,
    exti9_5,
    afio::exticr3,
    afio_exticr3,
    afio_exticr3_exti8,
    exti8,
  )),
  ((
    (I: IntExti95),
    Rt: Srt Frt,
    Tr8,
    Pr8,
    Tr8,
    Swier8,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr8,
    exti_pr_pr8,
    exti_rtsr_tr8,
    exti_swier_swier8,
    tr8,
    pr8,
    tr8,
    swier8,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 9 driver.",
  ExtiLn9,
  drv_exti_ln_9,
  "EXTI Line 9 resource.",
  ExtiLn9Res,
  Mr9,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr9,
  exti_imr_mr9,
  mr9,
  ((
    I: IntExti95,
    (Frt),
    Exti9,
    exti9_5,
    afio::exticr3,
    afio_exticr3,
    afio_exticr3_exti9,
    exti9,
  )),
  ((
    (I: IntExti95),
    Rt: Srt Frt,
    Tr9,
    Pr9,
    Tr9,
    Swier9,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr9,
    exti_pr_pr9,
    exti_rtsr_tr9,
    exti_swier_swier9,
    tr9,
    pr9,
    tr9,
    swier9,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 10 driver.",
  ExtiLn10,
  drv_exti_ln_10,
  "EXTI Line 10 resource.",
  ExtiLn10Res,
  Mr10,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr10,
  exti_imr_mr10,
  mr10,
  ((
    I: IntExti1510,
    (Frt),
    Exti10,
    exti15_10,
    afio::exticr3,
    afio_exticr3,
    afio_exticr3_exti10,
    exti10,
  )),
  ((
    (I: IntExti1510),
    Rt: Srt Frt,
    Tr10,
    Pr10,
    Tr10,
    Swier10,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr10,
    exti_pr_pr10,
    exti_rtsr_tr10,
    exti_swier_swier10,
    tr10,
    pr10,
    tr10,
    swier10,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 11 driver.",
  ExtiLn11,
  drv_exti_ln_11,
  "EXTI Line 11 resource.",
  ExtiLn11Res,
  Mr11,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr11,
  exti_imr_mr11,
  mr11,
  ((
    I: IntExti1510,
    (Frt),
    Exti11,
    exti15_10,
    afio::exticr3,
    afio_exticr3,
    afio_exticr3_exti11,
    exti11,
  )),
  ((
    (I: IntExti1510),
    Rt: Srt Frt,
    Tr11,
    Pr11,
    Tr11,
    Swier11,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr11,
    exti_pr_pr11,
    exti_rtsr_tr11,
    exti_swier_swier11,
    tr11,
    pr11,
    tr11,
    swier11,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 12 driver.",
  ExtiLn12,
  drv_exti_ln_12,
  "EXTI Line 12 resource.",
  ExtiLn12Res,
  Mr12,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr12,
  exti_imr_mr12,
  mr12,
  ((
    I: IntExti1510,
    (Frt),
    Exti12,
    exti15_10,
    afio::exticr4,
    afio_exticr4,
    afio_exticr4_exti12,
    exti12,
  )),
  ((
    (I: IntExti1510),
    Rt: Srt Frt,
    Tr12,
    Pr12,
    Tr12,
    Swier12,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr12,
    exti_pr_pr12,
    exti_rtsr_tr12,
    exti_swier_swier12,
    tr12,
    pr12,
    tr12,
    swier12,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 13 driver.",
  ExtiLn13,
  drv_exti_ln_13,
  "EXTI Line 13 resource.",
  ExtiLn13Res,
  Mr13,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr13,
  exti_imr_mr13,
  mr13,
  ((
    I: IntExti1510,
    (Frt),
    Exti13,
    exti15_10,
    afio::exticr4,
    afio_exticr4,
    afio_exticr4_exti13,
    exti13,
  )),
  ((
    (I: IntExti1510),
    Rt: Srt Frt,
    Tr13,
    Pr13,
    Tr13,
    Swier13,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr13,
    exti_pr_pr13,
    exti_rtsr_tr13,
    exti_swier_swier13,
    tr13,
    pr13,
    tr13,
    swier13,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 14 driver.",
  ExtiLn14,
  drv_exti_ln_14,
  "EXTI Line 14 resource.",
  ExtiLn14Res,
  Mr14,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr14,
  exti_imr_mr14,
  mr14,
  ((
    I: IntExti1510,
    (Frt),
    Exti14,
    exti15_10,
    afio::exticr4,
    afio_exticr4,
    afio_exticr4_exti14,
    exti14,
  )),
  ((
    (I: IntExti1510),
    Rt: Srt Frt,
    Tr14,
    Pr14,
    Tr14,
    Swier14,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr14,
    exti_pr_pr14,
    exti_rtsr_tr14,
    exti_swier_swier14,
    tr14,
    pr14,
    tr14,
    swier14,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 15 driver.",
  ExtiLn15,
  drv_exti_ln_15,
  "EXTI Line 15 resource.",
  ExtiLn15Res,
  Mr15,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr15,
  exti_imr_mr15,
  mr15,
  ((
    I: IntExti1510,
    (Frt),
    Exti15,
    exti15_10,
    afio::exticr4,
    afio_exticr4,
    afio_exticr4_exti15,
    exti15,
  )),
  ((
    (I: IntExti1510),
    Rt: Srt Frt,
    Tr15,
    Pr15,
    Tr15,
    Swier15,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr15,
    exti_pr_pr15,
    exti_rtsr_tr15,
    exti_swier_swier15,
    tr15,
    pr15,
    tr15,
    swier15,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 16 driver.",
  ExtiLn16,
  drv_exti_ln_16,
  "EXTI Line 16 resource.",
  ExtiLn16Res,
  Mr16,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr16,
  exti_imr_mr16,
  mr16,
  (),
  ((
    (),
    Rt: Srt Frt,
    Tr16,
    Pr16,
    Tr16,
    Swier16,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr16,
    exti_pr_pr16,
    exti_rtsr_tr16,
    exti_swier_swier16,
    tr16,
    pr16,
    tr16,
    swier16,
  )),
}

#[cfg(
  any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 17 driver.",
  ExtiLn17,
  drv_exti_ln_17,
  "EXTI Line 17 resource.",
  ExtiLn17Res,
  Mr17,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr17,
  exti_imr_mr17,
  mr17,
  (),
  ((
    (),
    Rt: Srt Frt,
    Tr17,
    Pr17,
    Tr17,
    Swier17,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr17,
    exti_pr_pr17,
    exti_rtsr_tr17,
    exti_swier_swier17,
    tr17,
    pr17,
    tr17,
    swier17,
  )),
}

#[cfg(
  any(
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  )
)]
exti_line! {
  "EXTI Line 18 driver.",
  ExtiLn18,
  drv_exti_ln_18,
  "EXTI Line 18 resource.",
  ExtiLn18Res,
  Mr18,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr18,
  exti_imr_mr18,
  mr18,
  (),
  ((
    (),
    Rt: Srt Frt,
    Tr18,
    Pr18,
    Tr18,
    Swier18,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr18,
    exti_pr_pr18,
    exti_rtsr_tr18,
    exti_swier_swier18,
    tr18,
    pr18,
    tr18,
    swier18,
  )),
}

#[cfg(any(feature = "stm32f107"))]
exti_line! {
  "EXTI Line 19 driver.",
  ExtiLn19,
  drv_exti_ln_19,
  "EXTI Line 19 resource.",
  ExtiLn19Res,
  Mr19,
  emr,
  imr,
  exti_emr,
  exti_imr,
  exti_emr_mr19,
  exti_imr_mr19,
  mr19,
  (),
  ((
    (),
    Rt: Srt Frt,
    Tr19,
    Pr19,
    Tr19,
    Swier19,
    ftsr,
    pr,
    rtsr,
    swier,
    exti_ftsr,
    exti_pr,
    exti_rtsr,
    exti_swier,
    exti_ftsr_tr19,
    exti_pr_pr19,
    exti_rtsr_tr19,
    exti_swier_swier19,
    tr19,
    pr19,
    tr19,
    swier19,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 0 driver.",
  ExtiLn0,
  drv_exti_ln_0,
  "EXTI Line 0 resource.",
  ExtiLn0Res,
  Mr0,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr0,
  exti_imr1_mr0,
  mr0,
  ((
    I: IntExti0,
    (Frt),
    Exti0,
    exti0,
    syscfg::exticr1,
    syscfg_exticr1,
    syscfg_exticr1_exti0,
    exti0,
  )),
  ((
    (I: IntExti0),
    Rt: Srt Frt,
    Tr0,
    Pr0,
    Tr0,
    Swier0,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr0,
    exti_pr1_pr0,
    exti_rtsr1_tr0,
    exti_swier1_swier0,
    tr0,
    pr0,
    tr0,
    swier0,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 1 driver.",
  ExtiLn1,
  drv_exti_ln_1,
  "EXTI Line 1 resource.",
  ExtiLn1Res,
  Mr1,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr1,
  exti_imr1_mr1,
  mr1,
  ((
    I: IntExti1,
    (Frt),
    Exti1,
    exti1,
    syscfg::exticr1,
    syscfg_exticr1,
    syscfg_exticr1_exti1,
    exti1,
  )),
  ((
    (I: IntExti1),
    Rt: Srt Frt,
    Tr1,
    Pr1,
    Tr1,
    Swier1,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr1,
    exti_pr1_pr1,
    exti_rtsr1_tr1,
    exti_swier1_swier1,
    tr1,
    pr1,
    tr1,
    swier1,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 2 driver.",
  ExtiLn2,
  drv_exti_ln_2,
  "EXTI Line 2 resource.",
  ExtiLn2Res,
  Mr2,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr2,
  exti_imr1_mr2,
  mr2,
  ((
    I: IntExti2,
    (Frt),
    Exti2,
    exti2,
    syscfg::exticr1,
    syscfg_exticr1,
    syscfg_exticr1_exti2,
    exti2,
  )),
  ((
    (I: IntExti2),
    Rt: Srt Frt,
    Tr2,
    Pr2,
    Tr2,
    Swier2,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr2,
    exti_pr1_pr2,
    exti_rtsr1_tr2,
    exti_swier1_swier2,
    tr2,
    pr2,
    tr2,
    swier2,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 3 driver.",
  ExtiLn3,
  drv_exti_ln_3,
  "EXTI Line 3 resource.",
  ExtiLn3Res,
  Mr3,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr3,
  exti_imr1_mr3,
  mr3,
  ((
    I: IntExti3,
    (Frt),
    Exti3,
    exti3,
    syscfg::exticr1,
    syscfg_exticr1,
    syscfg_exticr1_exti3,
    exti3,
  )),
  ((
    (I: IntExti3),
    Rt: Srt Frt,
    Tr3,
    Pr3,
    Tr3,
    Swier3,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr3,
    exti_pr1_pr3,
    exti_rtsr1_tr3,
    exti_swier1_swier3,
    tr3,
    pr3,
    tr3,
    swier3,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 4 driver.",
  ExtiLn4,
  drv_exti_ln_4,
  "EXTI Line 4 resource.",
  ExtiLn4Res,
  Mr4,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr4,
  exti_imr1_mr4,
  mr4,
  ((
    I: IntExti4,
    (Frt),
    Exti4,
    exti4,
    syscfg::exticr2,
    syscfg_exticr2,
    syscfg_exticr2_exti4,
    exti4,
  )),
  ((
    (I: IntExti4),
    Rt: Srt Frt,
    Tr4,
    Pr4,
    Tr4,
    Swier4,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr4,
    exti_pr1_pr4,
    exti_rtsr1_tr4,
    exti_swier1_swier4,
    tr4,
    pr4,
    tr4,
    swier4,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 5 driver.",
  ExtiLn5,
  drv_exti_ln_5,
  "EXTI Line 5 resource.",
  ExtiLn5Res,
  Mr5,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr5,
  exti_imr1_mr5,
  mr5,
  ((
    I: IntExti95,
    (Frt),
    Exti5,
    exti9_5,
    syscfg::exticr2,
    syscfg_exticr2,
    syscfg_exticr2_exti5,
    exti5,
  )),
  ((
    (I: IntExti95),
    Rt: Srt Frt,
    Tr5,
    Pr5,
    Tr5,
    Swier5,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr5,
    exti_pr1_pr5,
    exti_rtsr1_tr5,
    exti_swier1_swier5,
    tr5,
    pr5,
    tr5,
    swier5,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 6 driver.",
  ExtiLn6,
  drv_exti_ln_6,
  "EXTI Line 6 resource.",
  ExtiLn6Res,
  Mr6,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr6,
  exti_imr1_mr6,
  mr6,
  ((
    I: IntExti95,
    (Frt),
    Exti6,
    exti9_5,
    syscfg::exticr2,
    syscfg_exticr2,
    syscfg_exticr2_exti6,
    exti6,
  )),
  ((
    (I: IntExti95),
    Rt: Srt Frt,
    Tr6,
    Pr6,
    Tr6,
    Swier6,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr6,
    exti_pr1_pr6,
    exti_rtsr1_tr6,
    exti_swier1_swier6,
    tr6,
    pr6,
    tr6,
    swier6,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 7 driver.",
  ExtiLn7,
  drv_exti_ln_7,
  "EXTI Line 7 resource.",
  ExtiLn7Res,
  Mr7,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr7,
  exti_imr1_mr7,
  mr7,
  ((
    I: IntExti95,
    (Frt),
    Exti7,
    exti9_5,
    syscfg::exticr2,
    syscfg_exticr2,
    syscfg_exticr2_exti7,
    exti7,
  )),
  ((
    (I: IntExti95),
    Rt: Srt Frt,
    Tr7,
    Pr7,
    Tr7,
    Swier7,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr7,
    exti_pr1_pr7,
    exti_rtsr1_tr7,
    exti_swier1_swier7,
    tr7,
    pr7,
    tr7,
    swier7,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 8 driver.",
  ExtiLn8,
  drv_exti_ln_8,
  "EXTI Line 8 resource.",
  ExtiLn8Res,
  Mr8,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr8,
  exti_imr1_mr8,
  mr8,
  ((
    I: IntExti95,
    (Frt),
    Exti8,
    exti9_5,
    syscfg::exticr3,
    syscfg_exticr3,
    syscfg_exticr3_exti8,
    exti8,
  )),
  ((
    (I: IntExti95),
    Rt: Srt Frt,
    Tr8,
    Pr8,
    Tr8,
    Swier8,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr8,
    exti_pr1_pr8,
    exti_rtsr1_tr8,
    exti_swier1_swier8,
    tr8,
    pr8,
    tr8,
    swier8,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 9 driver.",
  ExtiLn9,
  drv_exti_ln_9,
  "EXTI Line 9 resource.",
  ExtiLn9Res,
  Mr9,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr9,
  exti_imr1_mr9,
  mr9,
  ((
    I: IntExti95,
    (Frt),
    Exti9,
    exti9_5,
    syscfg::exticr3,
    syscfg_exticr3,
    syscfg_exticr3_exti9,
    exti9,
  )),
  ((
    (I: IntExti95),
    Rt: Srt Frt,
    Tr9,
    Pr9,
    Tr9,
    Swier9,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr9,
    exti_pr1_pr9,
    exti_rtsr1_tr9,
    exti_swier1_swier9,
    tr9,
    pr9,
    tr9,
    swier9,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 10 driver.",
  ExtiLn10,
  drv_exti_ln_10,
  "EXTI Line 10 resource.",
  ExtiLn10Res,
  Mr10,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr10,
  exti_imr1_mr10,
  mr10,
  ((
    I: IntExti1510,
    (Frt),
    Exti10,
    exti15_10,
    syscfg::exticr3,
    syscfg_exticr3,
    syscfg_exticr3_exti10,
    exti10,
  )),
  ((
    (I: IntExti1510),
    Rt: Srt Frt,
    Tr10,
    Pr10,
    Tr10,
    Swier10,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr10,
    exti_pr1_pr10,
    exti_rtsr1_tr10,
    exti_swier1_swier10,
    tr10,
    pr10,
    tr10,
    swier10,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 11 driver.",
  ExtiLn11,
  drv_exti_ln_11,
  "EXTI Line 11 resource.",
  ExtiLn11Res,
  Mr11,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr11,
  exti_imr1_mr11,
  mr11,
  ((
    I: IntExti1510,
    (Frt),
    Exti11,
    exti15_10,
    syscfg::exticr3,
    syscfg_exticr3,
    syscfg_exticr3_exti11,
    exti11,
  )),
  ((
    (I: IntExti1510),
    Rt: Srt Frt,
    Tr11,
    Pr11,
    Tr11,
    Swier11,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr11,
    exti_pr1_pr11,
    exti_rtsr1_tr11,
    exti_swier1_swier11,
    tr11,
    pr11,
    tr11,
    swier11,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 12 driver.",
  ExtiLn12,
  drv_exti_ln_12,
  "EXTI Line 12 resource.",
  ExtiLn12Res,
  Mr12,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr12,
  exti_imr1_mr12,
  mr12,
  ((
    I: IntExti1510,
    (Frt),
    Exti12,
    exti15_10,
    syscfg::exticr4,
    syscfg_exticr4,
    syscfg_exticr4_exti12,
    exti12,
  )),
  ((
    (I: IntExti1510),
    Rt: Srt Frt,
    Tr12,
    Pr12,
    Tr12,
    Swier12,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr12,
    exti_pr1_pr12,
    exti_rtsr1_tr12,
    exti_swier1_swier12,
    tr12,
    pr12,
    tr12,
    swier12,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 13 driver.",
  ExtiLn13,
  drv_exti_ln_13,
  "EXTI Line 13 resource.",
  ExtiLn13Res,
  Mr13,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr13,
  exti_imr1_mr13,
  mr13,
  ((
    I: IntExti1510,
    (Frt),
    Exti13,
    exti15_10,
    syscfg::exticr4,
    syscfg_exticr4,
    syscfg_exticr4_exti13,
    exti13,
  )),
  ((
    (I: IntExti1510),
    Rt: Srt Frt,
    Tr13,
    Pr13,
    Tr13,
    Swier13,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr13,
    exti_pr1_pr13,
    exti_rtsr1_tr13,
    exti_swier1_swier13,
    tr13,
    pr13,
    tr13,
    swier13,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 14 driver.",
  ExtiLn14,
  drv_exti_ln_14,
  "EXTI Line 14 resource.",
  ExtiLn14Res,
  Mr14,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr14,
  exti_imr1_mr14,
  mr14,
  ((
    I: IntExti1510,
    (Frt),
    Exti14,
    exti15_10,
    syscfg::exticr4,
    syscfg_exticr4,
    syscfg_exticr4_exti14,
    exti14,
  )),
  ((
    (I: IntExti1510),
    Rt: Srt Frt,
    Tr14,
    Pr14,
    Tr14,
    Swier14,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr14,
    exti_pr1_pr14,
    exti_rtsr1_tr14,
    exti_swier1_swier14,
    tr14,
    pr14,
    tr14,
    swier14,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 15 driver.",
  ExtiLn15,
  drv_exti_ln_15,
  "EXTI Line 15 resource.",
  ExtiLn15Res,
  Mr15,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr15,
  exti_imr1_mr15,
  mr15,
  ((
    I: IntExti1510,
    (Frt),
    Exti15,
    exti15_10,
    syscfg::exticr4,
    syscfg_exticr4,
    syscfg_exticr4_exti15,
    exti15,
  )),
  ((
    (I: IntExti1510),
    Rt: Srt Frt,
    Tr15,
    Pr15,
    Tr15,
    Swier15,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr15,
    exti_pr1_pr15,
    exti_rtsr1_tr15,
    exti_swier1_swier15,
    tr15,
    pr15,
    tr15,
    swier15,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 16 driver.",
  ExtiLn16,
  drv_exti_ln_16,
  "EXTI Line 16 resource.",
  ExtiLn16Res,
  Mr16,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr16,
  exti_imr1_mr16,
  mr16,
  (),
  ((
    (),
    Rt: Srt Frt,
    Tr16,
    Pr16,
    Tr16,
    Swier16,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr16,
    exti_pr1_pr16,
    exti_rtsr1_tr16,
    exti_swier1_swier16,
    tr16,
    pr16,
    tr16,
    swier16,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 17 driver.",
  ExtiLn17,
  drv_exti_ln_17,
  "EXTI Line 17 resource.",
  ExtiLn17Res,
  Mr17,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr17,
  exti_imr1_mr17,
  mr17,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 18 driver.",
  ExtiLn18,
  drv_exti_ln_18,
  "EXTI Line 18 resource.",
  ExtiLn18Res,
  Mr18,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr18,
  exti_imr1_mr18,
  mr18,
  (),
  ((
    (),
    Rt: Srt Frt,
    Tr18,
    Pr18,
    Tr18,
    Swier18,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr18,
    exti_pr1_pr18,
    exti_rtsr1_tr18,
    exti_swier1_swier18,
    tr18,
    pr18,
    tr18,
    swier18,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 19 driver.",
  ExtiLn19,
  drv_exti_ln_19,
  "EXTI Line 19 resource.",
  ExtiLn19Res,
  Mr19,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr19,
  exti_imr1_mr19,
  mr19,
  (),
  ((
    (),
    Rt: Srt Frt,
    Tr19,
    Pr19,
    Tr19,
    Swier19,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr19,
    exti_pr1_pr19,
    exti_rtsr1_tr19,
    exti_swier1_swier19,
    tr19,
    pr19,
    tr19,
    swier19,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 20 driver.",
  ExtiLn20,
  drv_exti_ln_20,
  "EXTI Line 20 resource.",
  ExtiLn20Res,
  Mr20,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr20,
  exti_imr1_mr20,
  mr20,
  (),
  ((
    (),
    Rt: Srt Frt,
    Tr20,
    Pr20,
    Tr20,
    Swier20,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr20,
    exti_pr1_pr20,
    exti_rtsr1_tr20,
    exti_swier1_swier20,
    tr20,
    pr20,
    tr20,
    swier20,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 21 driver.",
  ExtiLn21,
  drv_exti_ln_21,
  "EXTI Line 21 resource.",
  ExtiLn21Res,
  Mr21,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr21,
  exti_imr1_mr21,
  mr21,
  (),
  ((
    (),
    Rt: Srt Frt,
    Tr21,
    Pr21,
    Tr21,
    Swier21,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr21,
    exti_pr1_pr21,
    exti_rtsr1_tr21,
    exti_swier1_swier21,
    tr21,
    pr21,
    tr21,
    swier21,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 22 driver.",
  ExtiLn22,
  drv_exti_ln_22,
  "EXTI Line 22 resource.",
  ExtiLn22Res,
  Mr22,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr22,
  exti_imr1_mr22,
  mr22,
  (),
  ((
    (),
    Rt: Srt Frt,
    Tr22,
    Pr22,
    Tr22,
    Swier22,
    ftsr1,
    pr1,
    rtsr1,
    swier1,
    exti_ftsr1,
    exti_pr1,
    exti_rtsr1,
    exti_swier1,
    exti_ftsr1_tr22,
    exti_pr1_pr22,
    exti_rtsr1_tr22,
    exti_swier1_swier22,
    tr22,
    pr22,
    tr22,
    swier22,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 23 driver.",
  ExtiLn23,
  drv_exti_ln_23,
  "EXTI Line 23 resource.",
  ExtiLn23Res,
  Mr23,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr23,
  exti_imr1_mr23,
  mr23,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 24 driver.",
  ExtiLn24,
  drv_exti_ln_24,
  "EXTI Line 24 resource.",
  ExtiLn24Res,
  Mr24,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr24,
  exti_imr1_mr24,
  mr24,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 25 driver.",
  ExtiLn25,
  drv_exti_ln_25,
  "EXTI Line 25 resource.",
  ExtiLn25Res,
  Mr25,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr25,
  exti_imr1_mr25,
  mr25,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 26 driver.",
  ExtiLn26,
  drv_exti_ln_26,
  "EXTI Line 26 resource.",
  ExtiLn26Res,
  Mr26,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr26,
  exti_imr1_mr26,
  mr26,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 27 driver.",
  ExtiLn27,
  drv_exti_ln_27,
  "EXTI Line 27 resource.",
  ExtiLn27Res,
  Mr27,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr27,
  exti_imr1_mr27,
  mr27,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 28 driver.",
  ExtiLn28,
  drv_exti_ln_28,
  "EXTI Line 28 resource.",
  ExtiLn28Res,
  Mr28,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr28,
  exti_imr1_mr28,
  mr28,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 29 driver.",
  ExtiLn29,
  drv_exti_ln_29,
  "EXTI Line 29 resource.",
  ExtiLn29Res,
  Mr29,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr29,
  exti_imr1_mr29,
  mr29,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 30 driver.",
  ExtiLn30,
  drv_exti_ln_30,
  "EXTI Line 30 resource.",
  ExtiLn30Res,
  Mr30,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr30,
  exti_imr1_mr30,
  mr30,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 31 driver.",
  ExtiLn31,
  drv_exti_ln_31,
  "EXTI Line 31 resource.",
  ExtiLn31Res,
  Mr31,
  emr1,
  imr1,
  exti_emr1,
  exti_imr1,
  exti_emr1_mr31,
  exti_imr1_mr31,
  mr31,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 32 driver.",
  ExtiLn32,
  drv_exti_ln_32,
  "EXTI Line 32 resource.",
  ExtiLn32Res,
  Mr32,
  emr2,
  imr2,
  exti_emr2,
  exti_imr2,
  exti_emr2_mr32,
  exti_imr2_mr32,
  mr32,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 33 driver.",
  ExtiLn33,
  drv_exti_ln_33,
  "EXTI Line 33 resource.",
  ExtiLn33Res,
  Mr33,
  emr2,
  imr2,
  exti_emr2,
  exti_imr2,
  exti_emr2_mr33,
  exti_imr2_mr33,
  mr33,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 34 driver.",
  ExtiLn34,
  drv_exti_ln_34,
  "EXTI Line 34 resource.",
  ExtiLn34Res,
  Mr34,
  emr2,
  imr2,
  exti_emr2,
  exti_imr2,
  exti_emr2_mr34,
  exti_imr2_mr34,
  mr34,
  (),
  (),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 35 driver.",
  ExtiLn35,
  drv_exti_ln_35,
  "EXTI Line 35 resource.",
  ExtiLn35Res,
  Mr35,
  emr2,
  imr2,
  exti_emr2,
  exti_imr2,
  exti_emr2_mr35,
  exti_imr2_mr35,
  mr35,
  (),
  ((
    (),
    Rt: Srt Frt,
    Ft35,
    Pif35,
    Rt35,
    Swi35,
    ftsr2,
    pr2,
    rtsr2,
    swier2,
    exti_ftsr2,
    exti_pr2,
    exti_rtsr2,
    exti_swier2,
    exti_ftsr2_ft35,
    exti_pr2_pif35,
    exti_rtsr2_rt35,
    exti_swier2_swi35,
    ft35,
    pif35,
    rt35,
    swi35,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 36 driver.",
  ExtiLn36,
  drv_exti_ln_36,
  "EXTI Line 36 resource.",
  ExtiLn36Res,
  Mr36,
  emr2,
  imr2,
  exti_emr2,
  exti_imr2,
  exti_emr2_mr36,
  exti_imr2_mr36,
  mr36,
  (),
  ((
    (),
    Rt: Srt Frt,
    Ft36,
    Pif36,
    Rt36,
    Swi36,
    ftsr2,
    pr2,
    rtsr2,
    swier2,
    exti_ftsr2,
    exti_pr2,
    exti_rtsr2,
    exti_swier2,
    exti_ftsr2_ft36,
    exti_pr2_pif36,
    exti_rtsr2_rt36,
    exti_swier2_swi36,
    ft36,
    pif36,
    rt36,
    swi36,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 37 driver.",
  ExtiLn37,
  drv_exti_ln_37,
  "EXTI Line 37 resource.",
  ExtiLn37Res,
  Mr37,
  emr2,
  imr2,
  exti_emr2,
  exti_imr2,
  exti_emr2_mr37,
  exti_imr2_mr37,
  mr37,
  (),
  ((
    (),
    Rt: Srt Frt,
    Ft37,
    Pif37,
    Rt37,
    Swi37,
    ftsr2,
    pr2,
    rtsr2,
    swier2,
    exti_ftsr2,
    exti_pr2,
    exti_rtsr2,
    exti_swier2,
    exti_ftsr2_ft37,
    exti_pr2_pif37,
    exti_rtsr2_rt37,
    exti_swier2_swi37,
    ft37,
    pif37,
    rt37,
    swi37,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 38 driver.",
  ExtiLn38,
  drv_exti_ln_38,
  "EXTI Line 38 resource.",
  ExtiLn38Res,
  Mr38,
  emr2,
  imr2,
  exti_emr2,
  exti_imr2,
  exti_emr2_mr38,
  exti_imr2_mr38,
  mr38,
  (),
  ((
    (),
    Rt: Srt Frt,
    Ft38,
    Pif38,
    Rt38,
    Swi38,
    ftsr2,
    pr2,
    rtsr2,
    swier2,
    exti_ftsr2,
    exti_pr2,
    exti_rtsr2,
    exti_swier2,
    exti_ftsr2_ft38,
    exti_pr2_pif38,
    exti_rtsr2_rt38,
    exti_swier2_swi38,
    ft38,
    pif38,
    rt38,
    swi38,
  )),
}

#[cfg(
  any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  )
)]
exti_line! {
  "EXTI Line 39 driver.",
  ExtiLn39,
  drv_exti_ln_39,
  "EXTI Line 39 resource.",
  ExtiLn39Res,
  Mr39,
  emr2,
  imr2,
  exti_emr2,
  exti_imr2,
  exti_emr2_mr39,
  exti_imr2_mr39,
  mr39,
  (),
  (),
}
