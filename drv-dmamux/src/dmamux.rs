//! DMA request multiplexer.

use core::marker::PhantomData;
use drone_core::bitfield::Bitfield;
use drone_core::drv::Resource;
use drone_stm32_device::reg::marker::*;
use drone_stm32_device::reg::prelude::*;
use drone_stm32_device::reg::{dmamux1, rcc};
use drone_stm32_device::reg::{RegGuard, RegGuardCnt, RegGuardRes};

/// DMAMUX channel driver.
#[derive(Driver)]
pub struct DmamuxCh<T: DmamuxChRes>(T);

/// DMAMUX channel resource.
#[allow(missing_docs)]
pub trait DmamuxChRes: Resource {
  type RccRes: DmamuxRccRes;
  type CrVal: Bitfield<Bits = u32>;
  type Cr: SRwReg<Val = Self::CrVal>;
  type CrSyncId: SRwRwRegFieldBits<Reg = Self::Cr>;
  type CrNbreq: SRwRwRegFieldBits<Reg = Self::Cr>;
  type CrSpol: SRwRwRegFieldBits<Reg = Self::Cr>;
  type CrSe: SRwRwRegFieldBit<Reg = Self::Cr>;
  type CrEge: SRwRwRegFieldBit<Reg = Self::Cr>;
  type CrSoie: SRwRwRegFieldBit<Reg = Self::Cr>;
  type CrDmareqId: SRwRwRegFieldBits<Reg = Self::Cr>;
  type Csr: SRoReg;
  type CsrSof: SRoRoRegFieldBit<Reg = Self::Csr>;
  type Cfr: SWoReg;
  type CfrCsof: SWoWoRegFieldBit<Reg = Self::Cfr>;

  res_decl!(Cr, cr);
  res_decl!(CrSyncId, cr_sync_id);
  res_decl!(CrNbreq, cr_nbreq);
  res_decl!(CrSpol, cr_spol);
  res_decl!(CrSe, cr_se);
  res_decl!(CrEge, cr_ege);
  res_decl!(CrSoie, cr_soie);
  res_decl!(CrDmareqId, cr_dmareq_id);
  res_decl!(CsrSof, csr_sof);
  res_decl!(CfrCsof, cfr_csof);
}

/// DMAMUX request generator driver.
#[derive(Driver)]
pub struct DmamuxRg<T: DmamuxRgRes>(T);

/// DMAMUX request generator resource.
#[allow(missing_docs)]
pub trait DmamuxRgRes: Resource {
  type RccRes: DmamuxRccRes;
  type CrVal: Bitfield<Bits = u32>;
  type Cr: SRwReg<Val = Self::CrVal>;
  type CrGnbreq: SRwRwRegFieldBits<Reg = Self::Cr>;
  type CrGpol: SRwRwRegFieldBits<Reg = Self::Cr>;
  type CrGe: SRwRwRegFieldBit<Reg = Self::Cr>;
  type CrOie: SRwRwRegFieldBit<Reg = Self::Cr>;
  type CrSigId: SRwRwRegFieldBits<Reg = Self::Cr>;
  type Rgsr: SRoReg;
  type RgsrOf: SRoRoRegFieldBit<Reg = Self::Rgsr>;
  type Rgcfr: SWoReg;
  type RgcfrCof: SWoWoRegFieldBit<Reg = Self::Rgcfr>;

  res_decl!(Cr, cr);
  res_decl!(CrGnbreq, cr_gnbreq);
  res_decl!(CrGpol, cr_gpol);
  res_decl!(CrGe, cr_ge);
  res_decl!(CrOie, cr_oie);
  res_decl!(CrSigId, cr_sig_id);
  res_decl!(RgsrOf, rgsr_of);
  res_decl!(RgcfrCof, rgcfr_cof);
}

/// DMAMUX reset and clock control driver.
#[derive(Driver)]
pub struct DmamuxRcc<T, C>(T, PhantomData<C>)
where
  T: DmamuxRccRes,
  C: RegGuardCnt<DmamuxOn<T>>;

/// DMAMUX reset and clock control resource.
#[allow(missing_docs)]
pub trait DmamuxRccRes: Resource {
  type RccAhb1EnrVal: Bitfield<Bits = u32>;
  type RccAhb1Enr: CRwRegBitBand<Val = Self::RccAhb1EnrVal>;
  type RccAhb1EnrDmamuxEn: CRwRwRegFieldBitBand<Reg = Self::RccAhb1Enr>;

  res_decl!(RccAhb1EnrDmamuxEn, en);
}

/// DMAMUX clock on guard resource.
pub struct DmamuxOn<T: DmamuxRccRes>(T::RccAhb1EnrDmamuxEn);

#[allow(missing_docs)]
impl<T: DmamuxChRes> DmamuxCh<T> {
  #[inline(always)]
  pub fn cr(&self) -> &T::Cr {
    self.0.cr()
  }

  #[inline(always)]
  pub fn cr_sync_id(&self) -> &T::CrSyncId {
    self.0.cr_sync_id()
  }

  #[inline(always)]
  pub fn cr_nbreq(&self) -> &T::CrNbreq {
    self.0.cr_nbreq()
  }

  #[inline(always)]
  pub fn cr_spol(&self) -> &T::CrSpol {
    self.0.cr_spol()
  }

  #[inline(always)]
  pub fn cr_se(&self) -> &T::CrSe {
    self.0.cr_se()
  }

  #[inline(always)]
  pub fn cr_ege(&self) -> &T::CrEge {
    self.0.cr_ege()
  }

  #[inline(always)]
  pub fn cr_soie(&self) -> &T::CrSoie {
    self.0.cr_soie()
  }

  #[inline(always)]
  pub fn cr_dmareq_id(&self) -> &T::CrDmareqId {
    self.0.cr_dmareq_id()
  }

  #[inline(always)]
  pub fn csr_sof(&self) -> &T::CsrSof {
    self.0.csr_sof()
  }

  #[inline(always)]
  pub fn cfr_csof(&self) -> &T::CfrCsof {
    self.0.cfr_csof()
  }
}

#[allow(missing_docs)]
impl<T: DmamuxRgRes> DmamuxRg<T> {
  #[inline(always)]
  pub fn cr(&self) -> &T::Cr {
    self.0.cr()
  }

  #[inline(always)]
  pub fn cr_gnbreq(&self) -> &T::CrGnbreq {
    self.0.cr_gnbreq()
  }

  #[inline(always)]
  pub fn cr_gpol(&self) -> &T::CrGpol {
    self.0.cr_gpol()
  }

  #[inline(always)]
  pub fn cr_ge(&self) -> &T::CrGe {
    self.0.cr_ge()
  }

  #[inline(always)]
  pub fn cr_oie(&self) -> &T::CrOie {
    self.0.cr_oie()
  }

  #[inline(always)]
  pub fn cr_sig_id(&self) -> &T::CrSigId {
    self.0.cr_sig_id()
  }

  #[inline(always)]
  pub fn rgsr_of(&self) -> &T::RgsrOf {
    self.0.rgsr_of()
  }

  #[inline(always)]
  pub fn rgcfr_cof(&self) -> &T::RgcfrCof {
    self.0.rgcfr_cof()
  }
}

impl<T, C> DmamuxRcc<T, C>
where
  T: DmamuxRccRes,
  C: RegGuardCnt<DmamuxOn<T>>,
{
  /// Enables the clock.
  pub fn on(&self) -> RegGuard<DmamuxOn<T>, C> {
    RegGuard::new(DmamuxOn(*self.0.en()))
  }
}

impl<T: DmamuxRccRes> Clone for DmamuxOn<T> {
  #[inline(always)]
  fn clone(&self) -> Self {
    Self(self.0)
  }
}

impl<T: DmamuxRccRes> RegGuardRes for DmamuxOn<T> {
  type Reg = T::RccAhb1Enr;
  type Field = T::RccAhb1EnrDmamuxEn;

  #[inline(always)]
  fn field(&self) -> &Self::Field {
    &self.0
  }

  #[inline(always)]
  fn up(&self, val: &mut <Self::Reg as Reg<Crt>>::Val) {
    self.0.set(val)
  }

  #[inline(always)]
  fn down(&self, val: &mut <Self::Reg as Reg<Crt>>::Val) {
    self.0.clear(val)
  }
}

macro_rules! dmamux {
  (
    $doc:expr,
    $name:ident,
    $name_macro:ident,
    $doc_res:expr,
    $name_res:ident,
    $doc_on:expr,
    $name_on:ident,
    $dmamuxen_ty:ident,
    $dmamuxen:ident,
    $rcc_ahb1enr_dmamuxen:ident,
    ($((
      $doc_ch:expr,
      $name_ch:ident,
      $name_ch_macro:ident,
      $doc_ch_res:expr,
      $name_ch_res:ident,
      $sof_ty:ident,
      $csof_ty:ident,
      $dmamux_chcr:ident,
      $dmamux_csr_sof:ident,
      $dmamux_cfr_csof:ident,
      $chcr:ident,
      $sof:ident,
      $csof:ident,
    )),*),
    ($((
      $doc_rg:expr,
      $name_rg:ident,
      $name_rg_macro:ident,
      $doc_rg_res:expr,
      $name_rg_res:ident,
      $of_ty:ident,
      $cof_ty:ident,
      $dmamux_rgcr:ident,
      $dmamux_rgsr_of:ident,
      $dmamux_rgcfr_cof:ident,
      $rgcr:ident,
      $of:ident,
      $cof:ident,
    )),*),
  ) => {
    #[doc = $doc]
    pub type $name<C> = DmamuxRcc<$name_res<Crt>, C>;

    #[doc = $doc_res]
    #[allow(missing_docs)]
    pub struct $name_res<Rt: RegTag> {
      pub $rcc_ahb1enr_dmamuxen: rcc::ahb1enr::$dmamuxen_ty<Rt>,
    }

    #[doc = $doc_on]
    pub type $name_on = DmamuxOn<$name_res<Crt>>;

    /// Creates a new `DmamuxRcc`.
    #[macro_export]
    macro_rules! $name_macro {
      ($reg:ident, $rgc:path) => {
        <$crate::dmamux::DmamuxRcc<_, $rgc> as ::drone_core::drv::Driver>::new(
          $crate::dmamux::$name_res {
            $rcc_ahb1enr_dmamuxen: $reg.rcc_ahb1enr.$dmamuxen,
          },
        )
      };
    }

    impl Resource for $name_res<Crt> {
      type Source = $name_res<Srt>;

      #[inline(always)]
      fn from_source(source: Self::Source) -> Self {
        Self {
          $rcc_ahb1enr_dmamuxen: source.$rcc_ahb1enr_dmamuxen.to_copy(),
        }
      }
    }

    impl DmamuxRccRes for $name_res<Crt> {
      type RccAhb1EnrVal = rcc::ahb1enr::Val;
      type RccAhb1Enr = rcc::ahb1enr::Reg<Crt>;
      type RccAhb1EnrDmamuxEn = rcc::ahb1enr::$dmamuxen_ty<Crt>;

      res_impl!(RccAhb1EnrDmamuxEn, en, $rcc_ahb1enr_dmamuxen);
    }

    $(
      #[doc = $doc_ch]
      pub type $name_ch = DmamuxCh<$name_ch_res>;

      #[doc = $doc_ch_res]
      #[allow(missing_docs)]
      #[derive(Resource)]
      pub struct $name_ch_res {
        pub $dmamux_chcr: dmamux1::$chcr::Reg<Srt>,
        pub $dmamux_csr_sof: dmamux1::csr::$sof_ty<Srt>,
        pub $dmamux_cfr_csof: dmamux1::cfr::$csof_ty<Srt>,
      }

      /// Creates a new `DmamuxCh`.
      #[macro_export]
      macro_rules! $name_ch_macro {
        ($reg: ident) => {
          <$crate::dmamux::DmamuxCh<_> as ::drone_core::drv::Driver>::new(
            $crate::dmamux::$name_ch_res {
              $dmamux_chcr: $reg.$dmamux_chcr,
              $dmamux_csr_sof: $reg.dmamux1_csr.$sof,
              $dmamux_cfr_csof: $reg.dmamux1_cfr.$csof,
            },
          )
        };
      }

      impl DmamuxChRes for $name_ch_res {
        type RccRes = $name_res<Crt>;
        type CrVal = dmamux1::$chcr::Val;
        type Cr = dmamux1::$chcr::Reg<Srt>;
        type CrSyncId = dmamux1::$chcr::SyncId<Srt>;
        type CrNbreq = dmamux1::$chcr::Nbreq<Srt>;
        type CrSpol = dmamux1::$chcr::Spol<Srt>;
        type CrSe = dmamux1::$chcr::Se<Srt>;
        type CrEge = dmamux1::$chcr::Ege<Srt>;
        type CrSoie = dmamux1::$chcr::Soie<Srt>;
        type CrDmareqId = dmamux1::$chcr::DmareqId<Srt>;
        type Csr = dmamux1::csr::Reg<Srt>;
        type CsrSof = dmamux1::csr::$sof_ty<Srt>;
        type Cfr = dmamux1::cfr::Reg<Srt>;
        type CfrCsof = dmamux1::cfr::$csof_ty<Srt>;

        res_impl!(Cr, cr, $dmamux_chcr);
        res_impl!(CrSyncId, cr_sync_id, $dmamux_chcr.sync_id);
        res_impl!(CrNbreq, cr_nbreq, $dmamux_chcr.nbreq);
        res_impl!(CrSpol, cr_spol, $dmamux_chcr.spol);
        res_impl!(CrSe, cr_se, $dmamux_chcr.se);
        res_impl!(CrEge, cr_ege, $dmamux_chcr.ege);
        res_impl!(CrSoie, cr_soie, $dmamux_chcr.soie);
        res_impl!(CrDmareqId, cr_dmareq_id, $dmamux_chcr.dmareq_id);
        res_impl!(CsrSof, csr_sof, $dmamux_csr_sof);
        res_impl!(CfrCsof, cfr_csof, $dmamux_cfr_csof);
      }
    )*

    $(
      #[doc = $doc_rg]
      pub type $name_rg = DmamuxRg<$name_rg_res>;

      #[doc = $doc_rg_res]
      #[allow(missing_docs)]
      #[derive(Resource)]
      pub struct $name_rg_res {
        pub $dmamux_rgcr: dmamux1::$rgcr::Reg<Srt>,
        pub $dmamux_rgsr_of: dmamux1::rgsr::$of_ty<Srt>,
        pub $dmamux_rgcfr_cof: dmamux1::rgcfr::$cof_ty<Srt>,
      }

      /// Creates a new `DmamuxRg`.
      #[macro_export]
      macro_rules! $name_rg_macro {
        ($reg: ident) => {
          <$crate::dmamux::DmamuxRg<_> as ::drone_core::drv::Driver>::new(
            $crate::dmamux::$name_rg_res {
              $dmamux_rgcr: $reg.$dmamux_rgcr,
              $dmamux_rgsr_of: $reg.dmamux1_rgsr.$of,
              $dmamux_rgcfr_cof: $reg.dmamux1_rgcfr.$cof,
            },
          )
        };
      }

      impl DmamuxRgRes for $name_rg_res {
        type RccRes = $name_res<Crt>;
        type CrVal = dmamux1::$rgcr::Val;
        type Cr = dmamux1::$rgcr::Reg<Srt>;
        type CrGnbreq = dmamux1::$rgcr::Gnbreq<Srt>;
        type CrGpol = dmamux1::$rgcr::Gpol<Srt>;
        type CrGe = dmamux1::$rgcr::Ge<Srt>;
        type CrOie = dmamux1::$rgcr::Oie<Srt>;
        type CrSigId = dmamux1::$rgcr::SigId<Srt>;
        type Rgsr = dmamux1::rgsr::Reg<Srt>;
        type RgsrOf = dmamux1::rgsr::$of_ty<Srt>;
        type Rgcfr = dmamux1::rgcfr::Reg<Srt>;
        type RgcfrCof = dmamux1::rgcfr::$cof_ty<Srt>;

        res_impl!(Cr, cr, $dmamux_rgcr);
        res_impl!(CrGnbreq, cr_gnbreq, $dmamux_rgcr.gnbreq);
        res_impl!(CrGpol, cr_gpol, $dmamux_rgcr.gpol);
        res_impl!(CrGe, cr_ge, $dmamux_rgcr.ge);
        res_impl!(CrOie, cr_oie, $dmamux_rgcr.oie);
        res_impl!(CrSigId, cr_sig_id, $dmamux_rgcr.sig_id);
        res_impl!(RgsrOf, rgsr_of, $dmamux_rgsr_of);
        res_impl!(RgcfrCof, rgcfr_cof, $dmamux_rgcfr_cof);
      }
    )*
  };
}

dmamux! {
  "DMAMUX1 reset and clock control driver.",
  Dmamux1Rcc,
  drv_dmamux1_rcc,
  "DMAMUX1 reset and clock control resource.",
  Dmamux1RccRes,
  "DMAMUX1 clock on guard resource.",
  Dmamux1On,
  Dmamux1En,
  dmamux1en,
  rcc_ahb1enr_dmamux1en,
  ((
    "DMAMUX1 Channel 0 driver.",
    Dmamux1Ch0,
    drv_dmamux1_ch0,
    "DMAMUX1 Channel 0 resource.",
    Dmamux1Ch0Res,
    Sof0,
    Csof0,
    dmamux1_c0cr,
    dmamux1_csr_sof0,
    dmamux1_cfr_csof0,
    c0cr,
    sof0,
    csof0,
  ), (
    "DMAMUX1 Channel 1 driver.",
    Dmamux1Ch1,
    drv_dmamux1_ch1,
    "DMAMUX1 Channel 1 resource.",
    Dmamux1Ch1Res,
    Sof1,
    Csof1,
    dmamux1_c1cr,
    dmamux1_csr_sof1,
    dmamux1_cfr_csof1,
    c1cr,
    sof1,
    csof1,
  ), (
    "DMAMUX1 Channel 2 driver.",
    Dmamux1Ch2,
    drv_dmamux1_ch2,
    "DMAMUX1 Channel 2 resource.",
    Dmamux1Ch2Res,
    Sof2,
    Csof2,
    dmamux1_c2cr,
    dmamux1_csr_sof2,
    dmamux1_cfr_csof2,
    c2cr,
    sof2,
    csof2,
  ), (
    "DMAMUX1 Channel 3 driver.",
    Dmamux1Ch3,
    drv_dmamux1_ch3,
    "DMAMUX1 Channel 3 resource.",
    Dmamux1Ch3Res,
    Sof3,
    Csof3,
    dmamux1_c3cr,
    dmamux1_csr_sof3,
    dmamux1_cfr_csof3,
    c3cr,
    sof3,
    csof3,
  ), (
    "DMAMUX1 Channel 4 driver.",
    Dmamux1Ch4,
    drv_dmamux1_ch4,
    "DMAMUX1 Channel 4 resource.",
    Dmamux1Ch4Res,
    Sof4,
    Csof4,
    dmamux1_c4cr,
    dmamux1_csr_sof4,
    dmamux1_cfr_csof4,
    c4cr,
    sof4,
    csof4,
  ), (
    "DMAMUX1 Channel 5 driver.",
    Dmamux1Ch5,
    drv_dmamux1_ch5,
    "DMAMUX1 Channel 5 resource.",
    Dmamux1Ch5Res,
    Sof5,
    Csof5,
    dmamux1_c5cr,
    dmamux1_csr_sof5,
    dmamux1_cfr_csof5,
    c5cr,
    sof5,
    csof5,
  ), (
    "DMAMUX1 Channel 6 driver.",
    Dmamux1Ch6,
    drv_dmamux1_ch6,
    "DMAMUX1 Channel 6 resource.",
    Dmamux1Ch6Res,
    Sof6,
    Csof6,
    dmamux1_c6cr,
    dmamux1_csr_sof6,
    dmamux1_cfr_csof6,
    c6cr,
    sof6,
    csof6,
  ), (
    "DMAMUX1 Channel 7 driver.",
    Dmamux1Ch7,
    drv_dmamux1_ch7,
    "DMAMUX1 Channel 7 resource.",
    Dmamux1Ch7Res,
    Sof7,
    Csof7,
    dmamux1_c7cr,
    dmamux1_csr_sof7,
    dmamux1_cfr_csof7,
    c7cr,
    sof7,
    csof7,
  ), (
    "DMAMUX1 Channel 8 driver.",
    Dmamux1Ch8,
    drv_dmamux1_ch8,
    "DMAMUX1 Channel 8 resource.",
    Dmamux1Ch8Res,
    Sof8,
    Csof8,
    dmamux1_c8cr,
    dmamux1_csr_sof8,
    dmamux1_cfr_csof8,
    c8cr,
    sof8,
    csof8,
  ), (
    "DMAMUX1 Channel 9 driver.",
    Dmamux1Ch9,
    drv_dmamux1_ch9,
    "DMAMUX1 Channel 9 resource.",
    Dmamux1Ch9Res,
    Sof9,
    Csof9,
    dmamux1_c9cr,
    dmamux1_csr_sof9,
    dmamux1_cfr_csof9,
    c9cr,
    sof9,
    csof9,
  ), (
    "DMAMUX1 Channel 10 driver.",
    Dmamux1Ch10,
    drv_dmamux1_ch10,
    "DMAMUX1 Channel 10 resource.",
    Dmamux1Ch10Res,
    Sof10,
    Csof10,
    dmamux1_c10cr,
    dmamux1_csr_sof10,
    dmamux1_cfr_csof10,
    c10cr,
    sof10,
    csof10,
  ), (
    "DMAMUX1 Channel 11 driver.",
    Dmamux1Ch11,
    drv_dmamux1_ch11,
    "DMAMUX1 Channel 11 resource.",
    Dmamux1Ch11Res,
    Sof11,
    Csof11,
    dmamux1_c11cr,
    dmamux1_csr_sof11,
    dmamux1_cfr_csof11,
    c11cr,
    sof11,
    csof11,
  ), (
    "DMAMUX1 Channel 12 driver.",
    Dmamux1Ch12,
    drv_dmamux1_ch12,
    "DMAMUX1 Channel 12 resource.",
    Dmamux1Ch12Res,
    Sof12,
    Csof12,
    dmamux1_c12cr,
    dmamux1_csr_sof12,
    dmamux1_cfr_csof12,
    c12cr,
    sof12,
    csof12,
  ), (
    "DMAMUX1 Channel 13 driver.",
    Dmamux1Ch13,
    drv_dmamux1_ch13,
    "DMAMUX1 Channel 13 resource.",
    Dmamux1Ch13Res,
    Sof13,
    Csof13,
    dmamux1_c13cr,
    dmamux1_csr_sof13,
    dmamux1_cfr_csof13,
    c13cr,
    sof13,
    csof13,
  )),
  ((
    "DMAMUX1 Request Generator 0 driver.",
    Dmamux1Rg0,
    drv_dmamux1_rg0,
    "DMAMUX1 Request Generator 0 resource.",
    Dmamux1Rg0Res,
    Of0,
    Cof0,
    dmamux1_rg0cr,
    dmamux1_rgsr_of0,
    dmamux1_rgcfr_of0,
    rg0cr,
    of0,
    cof0,
  ), (
    "DMAMUX1 Request Generator 1 driver.",
    Dmamux1Rg1,
    drv_dmamux1_rg1,
    "DMAMUX1 Request Generator 1 resource.",
    Dmamux1Rg1Res,
    Of1,
    Cof1,
    dmamux1_rg1cr,
    dmamux1_rgsr_of1,
    dmamux1_rgcfr_of1,
    rg1cr,
    of1,
    cof1,
  ), (
    "DMAMUX1 Request Generator 2 driver.",
    Dmamux1Rg2,
    drv_dmamux1_rg2,
    "DMAMUX1 Request Generator 2 resource.",
    Dmamux1Rg2Res,
    Of2,
    Cof2,
    dmamux1_rg2cr,
    dmamux1_rgsr_of2,
    dmamux1_rgcfr_of2,
    rg2cr,
    of2,
    cof2,
  ), (
    "DMAMUX1 Request Generator 3 driver.",
    Dmamux1Rg3,
    drv_dmamux1_rg3,
    "DMAMUX1 Request Generator 3 resource.",
    Dmamux1Rg3Res,
    Of3,
    Cof3,
    dmamux1_rg3cr,
    dmamux1_rgsr_of3,
    dmamux1_rgcfr_of3,
    rg3cr,
    of3,
    cof3,
  )),
}
