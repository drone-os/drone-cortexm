//! DMA request multiplexer.

use drone_core::bitfield::Bitfield;
use drone_core::drv::Resource;
use drone_stm32::reg::dmamux1;
use drone_stm32::reg::marker::*;
use drone_stm32::reg::prelude::*;

/// DMAMUX channel driver.
#[derive(Driver)]
pub struct DmamuxCh<T: DmamuxChRes>(T);

/// DMAMUX channel resource.
#[allow(missing_docs)]
pub trait DmamuxChRes: Resource {
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

  res_reg_decl!(Cr, cr, cr_mut);
  res_reg_decl!(CrSyncId, cr_sync_id, cr_sync_id_mut);
  res_reg_decl!(CrNbreq, cr_nbreq, cr_nbreq_mut);
  res_reg_decl!(CrSpol, cr_spol, cr_spol_mut);
  res_reg_decl!(CrSe, cr_se, cr_se_mut);
  res_reg_decl!(CrEge, cr_ege, cr_ege_mut);
  res_reg_decl!(CrSoie, cr_soie, cr_soie_mut);
  res_reg_decl!(CrDmareqId, cr_dmareq_id, cr_dmareq_id_mut);
  res_reg_decl!(CsrSof, csr_sof, csr_sof_mut);
  res_reg_decl!(CfrCsof, cfr_csof, cfr_csof_mut);
}

/// DMAMUX request generator driver.
#[derive(Driver)]
pub struct DmamuxRg<T: DmamuxRgRes>(T);

/// DMAMUX request generator resource.
#[allow(missing_docs)]
pub trait DmamuxRgRes: Resource {
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

  res_reg_decl!(Cr, cr, cr_mut);
  res_reg_decl!(CrGnbreq, cr_gnbreq, cr_gnbreq_mut);
  res_reg_decl!(CrGpol, cr_gpol, cr_gpol_mut);
  res_reg_decl!(CrGe, cr_ge, cr_ge_mut);
  res_reg_decl!(CrOie, cr_oie, cr_oie_mut);
  res_reg_decl!(CrSigId, cr_sig_id, cr_sig_id_mut);
  res_reg_decl!(RgsrOf, rgsr_of, rgsr_of_mut);
  res_reg_decl!(RgcfrCof, rgcfr_cof, rgcfr_cof_mut);
}

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

macro_rules! dmamux_ch {
  (
    $doc:expr,
    $name:ident,
    $name_macro:ident,
    $doc_res:expr,
    $name_res:ident,
    $sof_ty:ident,
    $csof_ty:ident,
    $dmamux_cr:ident,
    $dmamux_csr_sof:ident,
    $dmamux_cfr_csof:ident,
    $cr:ident,
    $sof:ident,
    $csof:ident,
  ) => {
    #[doc = $doc]
    pub type $name = DmamuxCh<$name_res>;

    #[doc = $doc_res]
    #[allow(missing_docs)]
    #[derive(Resource)]
    pub struct $name_res {
      pub $dmamux_cr: dmamux1::$cr::Reg<Srt>,
      pub $dmamux_csr_sof: dmamux1::csr::$sof_ty<Srt>,
      pub $dmamux_cfr_csof: dmamux1::cfr::$csof_ty<Srt>,
    }

    /// Creates a new `DmamuxCh`.
    #[macro_export]
    macro_rules! $name_macro {
      ($reg: ident) => {
        <$crate::dmamux::DmamuxCh<_> as ::drone_core::drv::Driver>::new(
          $crate::dmamux::$name_res {
            $dmamux_cr: $reg.$dmamux_cr,
            $dmamux_csr_sof: $reg.dmamux1_csr.$sof,
            $dmamux_cfr_csof: $reg.dmamux1_cfr.$csof,
          },
        )
      };
    }

    impl DmamuxChRes for $name_res {
      type CrVal = dmamux1::$cr::Val;
      type Cr = dmamux1::$cr::Reg<Srt>;
      type CrSyncId = dmamux1::$cr::SyncId<Srt>;
      type CrNbreq = dmamux1::$cr::Nbreq<Srt>;
      type CrSpol = dmamux1::$cr::Spol<Srt>;
      type CrSe = dmamux1::$cr::Se<Srt>;
      type CrEge = dmamux1::$cr::Ege<Srt>;
      type CrSoie = dmamux1::$cr::Soie<Srt>;
      type CrDmareqId = dmamux1::$cr::DmareqId<Srt>;
      type Csr = dmamux1::csr::Reg<Srt>;
      type CsrSof = dmamux1::csr::$sof_ty<Srt>;
      type Cfr = dmamux1::cfr::Reg<Srt>;
      type CfrCsof = dmamux1::cfr::$csof_ty<Srt>;

      res_reg_impl!(Cr, cr, cr_mut, $dmamux_cr);
      res_reg_field_impl!(
        CrSyncId,
        cr_sync_id,
        cr_sync_id_mut,
        $dmamux_cr,
        sync_id
      );
      res_reg_field_impl!(CrNbreq, cr_nbreq, cr_nbreq_mut, $dmamux_cr, nbreq);
      res_reg_field_impl!(CrSpol, cr_spol, cr_spol_mut, $dmamux_cr, spol);
      res_reg_field_impl!(CrSe, cr_se, cr_se_mut, $dmamux_cr, se);
      res_reg_field_impl!(CrEge, cr_ege, cr_ege_mut, $dmamux_cr, ege);
      res_reg_field_impl!(CrSoie, cr_soie, cr_soie_mut, $dmamux_cr, soie);
      res_reg_field_impl!(
        CrDmareqId,
        cr_dmareq_id,
        cr_dmareq_id_mut,
        $dmamux_cr,
        dmareq_id
      );
      res_reg_impl!(CsrSof, csr_sof, csr_sof_mut, $dmamux_csr_sof);
      res_reg_impl!(CfrCsof, cfr_csof, cfr_csof_mut, $dmamux_cfr_csof);
    }
  };
}

macro_rules! dmamux_rg {
  (
    $doc:expr,
    $name:ident,
    $name_macro:ident,
    $doc_res:expr,
    $name_res:ident,
    $of_ty:ident,
    $cof_ty:ident,
    $dmamux_cr:ident,
    $dmamux_rgsr_of:ident,
    $dmamux_rgcfr_cof:ident,
    $cr:ident,
    $of:ident,
    $cof:ident,
  ) => {
    #[doc = $doc]
    pub type $name = DmamuxRg<$name_res>;

    #[doc = $doc_res]
    #[allow(missing_docs)]
    #[derive(Resource)]
    pub struct $name_res {
      pub $dmamux_cr: dmamux1::$cr::Reg<Srt>,
      pub $dmamux_rgsr_of: dmamux1::rgsr::$of_ty<Srt>,
      pub $dmamux_rgcfr_cof: dmamux1::rgcfr::$cof_ty<Srt>,
    }

    /// Creates a new `DmamuxRg`.
    #[macro_export]
    macro_rules! $name_macro {
      ($reg: ident) => {
        <$crate::dmamux::DmamuxRg<_> as ::drone_core::drv::Driver>::new(
          $crate::dmamux::$name_res {
            $dmamux_cr: $reg.$dmamux_cr,
            $dmamux_rgsr_of: $reg.dmamux1_rgsr.$of,
            $dmamux_rgcfr_cof: $reg.dmamux1_rgcfr.$cof,
          },
        )
      };
    }

    impl DmamuxRgRes for $name_res {
      type CrVal = dmamux1::$cr::Val;
      type Cr = dmamux1::$cr::Reg<Srt>;
      type CrGnbreq = dmamux1::$cr::Gnbreq<Srt>;
      type CrGpol = dmamux1::$cr::Gpol<Srt>;
      type CrGe = dmamux1::$cr::Ge<Srt>;
      type CrOie = dmamux1::$cr::Oie<Srt>;
      type CrSigId = dmamux1::$cr::SigId<Srt>;
      type Rgsr = dmamux1::rgsr::Reg<Srt>;
      type RgsrOf = dmamux1::rgsr::$of_ty<Srt>;
      type Rgcfr = dmamux1::rgcfr::Reg<Srt>;
      type RgcfrCof = dmamux1::rgcfr::$cof_ty<Srt>;

      res_reg_impl!(Cr, cr, cr_mut, $dmamux_cr);
      res_reg_field_impl!(
        CrGnbreq,
        cr_gnbreq,
        cr_gnbreq_mut,
        $dmamux_cr,
        gnbreq
      );
      res_reg_field_impl!(CrGpol, cr_gpol, cr_gpol_mut, $dmamux_cr, gpol);
      res_reg_field_impl!(CrGe, cr_ge, cr_ge_mut, $dmamux_cr, ge);
      res_reg_field_impl!(CrOie, cr_oie, cr_oie_mut, $dmamux_cr, oie);
      res_reg_field_impl!(
        CrSigId,
        cr_sig_id,
        cr_sig_id_mut,
        $dmamux_cr,
        sig_id
      );
      res_reg_impl!(RgsrOf, rgsr_of, rgsr_of_mut, $dmamux_rgsr_of);
      res_reg_impl!(RgcfrCof, rgcfr_cof, rgcfr_cof_mut, $dmamux_rgcfr_cof);
    }
  };
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_ch! {
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
}

dmamux_rg! {
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
}

dmamux_rg! {
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
}

dmamux_rg! {
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
}

dmamux_rg! {
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
}
