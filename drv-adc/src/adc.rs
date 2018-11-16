//! Analog-to-digital converters.

use core::marker::PhantomData;
use core::ptr::read_volatile;
use drone_core::bitfield::Bitfield;
use drone_core::drv::Resource;
use drone_stm32_core::fib;
#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drone_stm32_device::reg::adc_common;
use drone_stm32_device::reg::marker::*;
use drone_stm32_device::reg::prelude::*;
#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drone_stm32_device::reg::{adc, rcc};
use drone_stm32_device::reg::{RegGuard, RegGuardCnt, RegGuardRes};
#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drone_stm32_device::thr::int::IntAdc1;
use drone_stm32_device::thr::prelude::*;
use drone_stm32_drv_dma::dma::{DmaBond, DmaBondOnRgc};
#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drone_stm32_drv_dmamux::dmamux::{DmamuxCh, DmamuxChRes};
use futures::prelude::*;

/// ADC driver.
#[derive(Driver)]
pub struct Adc<T, C>(T, PhantomData<C>)
where
  T: AdcRes,
  C: RegGuardCnt<AdcOn<T>>;

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
/// ADC Common driver.
#[derive(Driver)]
pub struct AdcCom<I, C>(AdcComRes<Crt>, PhantomData<(I, C)>)
where
  I: IntAdc1<Att>,
  C: RegGuardCnt<Adc1On<I>>
    + RegGuardCnt<AdcCh18On<I, C>>
    + RegGuardCnt<AdcCh17On<I, C>>
    + RegGuardCnt<AdcVrefOn<I, C>>;

/// ADC resource.
#[allow(missing_docs)]
pub trait AdcRes: Resource + AdcResIsr {
  type Int: IntToken<Att>;
  type Ier: SRwReg;
  type Cr: SRwReg;
  type Cfgr: SRwReg;
  type Cfgr2: SRwReg;
  type Smpr1: SRwReg;
  type Smpr2: SRwReg;
  type Tr1: SRwReg;
  type Tr2: SRwReg;
  type Tr3: SRwReg;
  type Sqr1: SRwReg;
  type Sqr2: SRwReg;
  type Sqr3: SRwReg;
  type Sqr4: SRwReg;
  type Dr: SRoReg;
  type Jsqr: SRwReg;
  type Ofr1: SRwReg;
  type Ofr2: SRwReg;
  type Ofr3: SRwReg;
  type Ofr4: SRwReg;
  type Jdr1: SRoReg;
  type Jdr2: SRoReg;
  type Jdr3: SRoReg;
  type Jdr4: SRoReg;
  type Awd2Cr: SRwReg;
  type Awd3Cr: SRwReg;
  type Difsel: SRwReg;
  type Calfact: SRwReg;
  type RccAhbEnrVal: Bitfield<Bits = u32>;
  type RccAhbEnr: CRwRegBitBand<Val = Self::RccAhbEnrVal>;
  type RccAhbEnrAdcEn: CRwRwRegFieldBitBand<Reg = Self::RccAhbEnr>;

  fn int(&self) -> Self::Int;

  res_decl!(Ier, ier);
  res_decl!(Cr, cr);
  res_decl!(Cfgr, cfgr);
  res_decl!(Cfgr2, cfgr2);
  res_decl!(Smpr1, smpr1);
  res_decl!(Smpr2, smpr2);
  res_decl!(Tr1, tr1);
  res_decl!(Tr2, tr2);
  res_decl!(Tr3, tr3);
  res_decl!(Sqr1, sqr1);
  res_decl!(Sqr2, sqr2);
  res_decl!(Sqr3, sqr3);
  res_decl!(Sqr4, sqr4);
  res_decl!(Dr, dr);
  res_decl!(Jsqr, jsqr);
  res_decl!(Ofr1, ofr1);
  res_decl!(Ofr2, ofr2);
  res_decl!(Ofr3, ofr3);
  res_decl!(Ofr4, ofr4);
  res_decl!(Jdr1, jdr1);
  res_decl!(Jdr2, jdr2);
  res_decl!(Jdr3, jdr3);
  res_decl!(Jdr4, jdr4);
  res_decl!(Awd2Cr, awd2cr);
  res_decl!(Awd3Cr, awd3cr);
  res_decl!(Difsel, difsel);
  res_decl!(Calfact, calfact);
  res_decl!(RccAhbEnrAdcEn, rcc_en);
}

/// ADC resource.
#[allow(missing_docs)]
pub trait AdcDmaRes<T: DmaBond>: AdcRes {
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  fn dmamux_init(
    &self,
    cr_val: &mut <<T::DmamuxChRes as DmamuxChRes>::Cr as Reg<Srt>>::Val,
    dmamux: &DmamuxCh<T::DmamuxChRes>,
  );
}

/// ADC resource.
#[allow(missing_docs)]
pub trait AdcResIsr {
  type Isr: CRwReg;
  type IsrJqovf: CRwRwRegFieldBit<Reg = Self::Isr>;
  type IsrAwd3: CRwRwRegFieldBit<Reg = Self::Isr>;
  type IsrAwd2: CRwRwRegFieldBit<Reg = Self::Isr>;
  type IsrAwd1: CRwRwRegFieldBit<Reg = Self::Isr>;
  type IsrJeos: CRwRwRegFieldBit<Reg = Self::Isr>;
  type IsrJeoc: CRwRwRegFieldBit<Reg = Self::Isr>;
  type IsrOvr: CRwRwRegFieldBit<Reg = Self::Isr>;
  type IsrEos: CRwRwRegFieldBit<Reg = Self::Isr>;
  type IsrEoc: CRwRwRegFieldBit<Reg = Self::Isr>;
  type IsrEosmp: CRwRwRegFieldBit<Reg = Self::Isr>;
  type IsrAdrdy: CRwRwRegFieldBit<Reg = Self::Isr>;

  res_decl!(Isr, isr);
  res_decl!(IsrJqovf, isr_jqovf);
  res_decl!(IsrAwd3, isr_awd3);
  res_decl!(IsrAwd2, isr_awd2);
  res_decl!(IsrAwd1, isr_awd1);
  res_decl!(IsrJeos, isr_jeos);
  res_decl!(IsrJeoc, isr_jeoc);
  res_decl!(IsrOvr, isr_ovr);
  res_decl!(IsrEos, isr_eos);
  res_decl!(IsrEoc, isr_eoc);
  res_decl!(IsrEosmp, isr_eosmp);
  res_decl!(IsrAdrdy, isr_adrdy);
}

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
/// ADC Common resource.
#[allow(missing_docs)]
pub struct AdcComRes<Rt: RegTag> {
  pub adc_common_ccr: adc_common::Ccr<Rt>,
  pub adc_common_csr: adc_common::Csr<Srt>,
}

#[cfg(any(
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
/// Reads internal voltage reference (V<sub>REFINT</sub>).
pub fn read_vref_cal() -> u16 {
  unsafe { read_volatile(0x1FFF_75AA as *const u16) }
}

#[allow(missing_docs)]
impl<T, C> Adc<T, C>
where
  T: AdcRes,
  C: RegGuardCnt<AdcOn<T>>,
{
  #[inline(always)]
  pub fn int(&self) -> T::Int {
    self.0.int()
  }

  #[inline(always)]
  pub fn isr(&self) -> &<T::Isr as Reg<Crt>>::SReg {
    self.0.isr().as_sync()
  }

  #[inline(always)]
  pub fn ier(&self) -> &T::Ier {
    self.0.ier()
  }

  #[inline(always)]
  pub fn cr(&self) -> &T::Cr {
    self.0.cr()
  }

  #[inline(always)]
  pub fn cfgr(&self) -> &T::Cfgr {
    self.0.cfgr()
  }

  #[inline(always)]
  pub fn cfgr2(&self) -> &T::Cfgr2 {
    self.0.cfgr2()
  }

  #[inline(always)]
  pub fn smpr1(&self) -> &T::Smpr1 {
    self.0.smpr1()
  }

  #[inline(always)]
  pub fn smpr2(&self) -> &T::Smpr2 {
    self.0.smpr2()
  }

  #[inline(always)]
  pub fn tr1(&self) -> &T::Tr1 {
    self.0.tr1()
  }

  #[inline(always)]
  pub fn tr2(&self) -> &T::Tr2 {
    self.0.tr2()
  }

  #[inline(always)]
  pub fn tr3(&self) -> &T::Tr3 {
    self.0.tr3()
  }

  #[inline(always)]
  pub fn sqr1(&self) -> &T::Sqr1 {
    self.0.sqr1()
  }

  #[inline(always)]
  pub fn sqr2(&self) -> &T::Sqr2 {
    self.0.sqr2()
  }

  #[inline(always)]
  pub fn sqr3(&self) -> &T::Sqr3 {
    self.0.sqr3()
  }

  #[inline(always)]
  pub fn sqr4(&self) -> &T::Sqr4 {
    self.0.sqr4()
  }

  #[inline(always)]
  pub fn dr(&self) -> &T::Dr {
    self.0.dr()
  }

  #[inline(always)]
  pub fn jsqr(&self) -> &T::Jsqr {
    self.0.jsqr()
  }

  #[inline(always)]
  pub fn ofr1(&self) -> &T::Ofr1 {
    self.0.ofr1()
  }

  #[inline(always)]
  pub fn ofr2(&self) -> &T::Ofr2 {
    self.0.ofr2()
  }

  #[inline(always)]
  pub fn ofr3(&self) -> &T::Ofr3 {
    self.0.ofr3()
  }

  #[inline(always)]
  pub fn ofr4(&self) -> &T::Ofr4 {
    self.0.ofr4()
  }

  #[inline(always)]
  pub fn jdr1(&self) -> &T::Jdr1 {
    self.0.jdr1()
  }

  #[inline(always)]
  pub fn jdr2(&self) -> &T::Jdr2 {
    self.0.jdr2()
  }

  #[inline(always)]
  pub fn jdr3(&self) -> &T::Jdr3 {
    self.0.jdr3()
  }

  #[inline(always)]
  pub fn jdr4(&self) -> &T::Jdr4 {
    self.0.jdr4()
  }

  #[inline(always)]
  pub fn awd2cr(&self) -> &T::Awd2Cr {
    self.0.awd2cr()
  }

  #[inline(always)]
  pub fn awd3cr(&self) -> &T::Awd3Cr {
    self.0.awd3cr()
  }

  #[inline(always)]
  pub fn difsel(&self) -> &T::Difsel {
    self.0.difsel()
  }

  #[inline(always)]
  pub fn calfact(&self) -> &T::Calfact {
    self.0.calfact()
  }
}

impl<T, C> Adc<T, C>
where
  T: AdcRes,
  C: RegGuardCnt<AdcOn<T>>,
{
  /// Enables the clock.
  pub fn on(&self) -> RegGuard<AdcOn<T>, C> {
    RegGuard::new(AdcOn(*self.0.rcc_en()))
  }

  /// Initializes DMA for the ADC as peripheral.
  pub fn dma_init<U>(&self, bond: &U)
  where
    U: DmaBond,
    T: AdcDmaRes<U>,
    C: DmaBondOnRgc<U::DmaRes>,
  {
    unsafe { bond.dma_ch().set_paddr(self.0.dr().to_ptr() as usize) };
    #[cfg(any(
      feature = "stm32l4r5",
      feature = "stm32l4r7",
      feature = "stm32l4r9",
      feature = "stm32l4s5",
      feature = "stm32l4s7",
      feature = "stm32l4s9"
    ))]
    bond.dmamux_ch().cr_dmareq_id().modify(|r| {
      self.0.dmamux_init(r, bond.dmamux_ch());
    });
  }

  /// Returns a future, which resolves on ADC ready event.
  pub fn ready(&self) -> impl Future<Item = (), Error = !> {
    let adrdy = *self.0.isr_adrdy();
    fib::add_future(
      self.0.int(),
      fib::new(move || loop {
        if adrdy.read_bit() {
          adrdy.set_bit();
          break Ok(());
        }
        yield;
      }),
    )
  }
}

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
impl Resource for AdcComRes<Crt> {
  type Source = AdcComRes<Srt>;

  #[inline(always)]
  fn from_source(source: Self::Source) -> Self {
    Self {
      adc_common_ccr: source.adc_common_ccr.to_copy(),
      adc_common_csr: source.adc_common_csr,
    }
  }
}

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
#[allow(missing_docs)]
impl<I, C> AdcCom<I, C>
where
  I: IntAdc1<Att>,
  C: RegGuardCnt<Adc1On<I>>
    + RegGuardCnt<AdcCh18On<I, C>>
    + RegGuardCnt<AdcCh17On<I, C>>
    + RegGuardCnt<AdcVrefOn<I, C>>,
{
  #[inline(always)]
  pub fn ccr_presc(&self) -> &adc_common::ccr::Presc<Srt> {
    &self.0.adc_common_ccr.presc.as_sync()
  }

  #[inline(always)]
  pub fn ccr_ckmode(&self) -> &adc_common::ccr::Ckmode<Srt> {
    &self.0.adc_common_ccr.ckmode.as_sync()
  }

  #[inline(always)]
  pub fn csr(&self) -> &adc_common::Csr<Srt> {
    &self.0.adc_common_csr.as_sync()
  }
}

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
impl<I, C> AdcCom<I, C>
where
  I: IntAdc1<Att>,
  C: RegGuardCnt<Adc1On<I>>
    + RegGuardCnt<AdcCh18On<I, C>>
    + RegGuardCnt<AdcCh17On<I, C>>
    + RegGuardCnt<AdcVrefOn<I, C>>,
{
  /// Enables the V<sub>BAT</sub> channel.
  pub fn ch18_on(
    &self,
    adc_on: RegGuard<Adc1On<I>, C>,
  ) -> RegGuard<AdcCh18On<I, C>, C> {
    RegGuard::new(AdcCh18On(self.0.adc_common_ccr.ch18sel, adc_on))
  }

  /// Enables the temperature sensor channel.
  pub fn ch17_on(
    &self,
    adc_on: RegGuard<Adc1On<I>, C>,
  ) -> RegGuard<AdcCh17On<I, C>, C> {
    RegGuard::new(AdcCh17On(self.0.adc_common_ccr.ch17sel, adc_on))
  }

  /// Enables the V<sub>REFINT</sub> channel.
  pub fn vref_on(
    &self,
    adc_on: RegGuard<Adc1On<I>, C>,
  ) -> RegGuard<AdcVrefOn<I, C>, C> {
    RegGuard::new(AdcVrefOn(self.0.adc_common_ccr.vrefen, adc_on))
  }
}

/// ADC clock on guard resource.
pub struct AdcOn<T: AdcRes>(T::RccAhbEnrAdcEn);

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
/// ADC V<sub>BAT</sub> channel on guard resource.
pub struct AdcCh18On<I, C>(
  adc_common::ccr::Ch18Sel<Crt>,
  RegGuard<Adc1On<I>, C>,
)
where
  I: IntAdc1<Att>,
  C: RegGuardCnt<Adc1On<I>>;

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
/// ADC temperature sensor channel on guard resource.
pub struct AdcCh17On<I, C>(
  adc_common::ccr::Ch17Sel<Crt>,
  RegGuard<Adc1On<I>, C>,
)
where
  I: IntAdc1<Att>,
  C: RegGuardCnt<Adc1On<I>>;

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
/// ADC V<sub>REFINT</sub> channel on guard resource.
pub struct AdcVrefOn<I, C>(
  adc_common::ccr::Vrefen<Crt>,
  RegGuard<Adc1On<I>, C>,
)
where
  I: IntAdc1<Att>,
  C: RegGuardCnt<Adc1On<I>>;

impl<T: AdcRes> Clone for AdcOn<T> {
  #[inline(always)]
  fn clone(&self) -> Self {
    Self(self.0)
  }
}

impl<T: AdcRes> RegGuardRes for AdcOn<T> {
  type Reg = T::RccAhbEnr;
  type Field = T::RccAhbEnrAdcEn;

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

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
impl<I, C> Clone for AdcCh18On<I, C>
where
  I: IntAdc1<Att>,
  C: RegGuardCnt<Adc1On<I>>,
{
  #[inline(always)]
  fn clone(&self) -> Self {
    Self(self.0, self.1.clone())
  }
}

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
impl<I, C> RegGuardRes for AdcCh18On<I, C>
where
  I: IntAdc1<Att>,
  C: RegGuardCnt<Adc1On<I>>,
{
  type Reg = adc_common::ccr::Reg<Crt>;
  type Field = adc_common::ccr::Ch18Sel<Crt>;

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

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
impl<I, C> Clone for AdcCh17On<I, C>
where
  I: IntAdc1<Att>,
  C: RegGuardCnt<Adc1On<I>>,
{
  #[inline(always)]
  fn clone(&self) -> Self {
    Self(self.0, self.1.clone())
  }
}

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
impl<I, C> RegGuardRes for AdcCh17On<I, C>
where
  I: IntAdc1<Att>,
  C: RegGuardCnt<Adc1On<I>>,
{
  type Reg = adc_common::ccr::Reg<Crt>;
  type Field = adc_common::ccr::Ch17Sel<Crt>;

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

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
impl<I, C> Clone for AdcVrefOn<I, C>
where
  I: IntAdc1<Att>,
  C: RegGuardCnt<Adc1On<I>>,
{
  #[inline(always)]
  fn clone(&self) -> Self {
    Self(self.0, self.1.clone())
  }
}

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
impl<I, C> RegGuardRes for AdcVrefOn<I, C>
where
  I: IntAdc1<Att>,
  C: RegGuardCnt<Adc1On<I>>,
{
  type Reg = adc_common::ccr::Reg<Crt>;
  type Field = adc_common::ccr::Vrefen<Crt>;

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

#[allow(unused_macros)]
macro_rules! adc {
  (
    $doc:expr,
    $name:ident,
    $name_macro:ident,
    $doc_res:expr,
    $name_res:ident,
    $doc_on:expr,
    $name_on:ident,
    $int_ty:ident,
    $adcen_ty:ident,
    $ahb_enr:ident,
    $rcc_ahb_enr_adcen:ident,
    $rcc_ahb_enr:ident,
    $adcen:ident,
    $int:ident,
    $adc:ident,
    $adc_isr:ident,
    $adc_ier:ident,
    $adc_cr:ident,
    $adc_cfgr:ident,
    $adc_cfgr2:ident,
    $adc_smpr1:ident,
    $adc_smpr2:ident,
    $adc_tr1:ident,
    $adc_tr2:ident,
    $adc_tr3:ident,
    $adc_sqr1:ident,
    $adc_sqr2:ident,
    $adc_sqr3:ident,
    $adc_sqr4:ident,
    $adc_dr:ident,
    $adc_jsqr:ident,
    $adc_ofr1:ident,
    $adc_ofr2:ident,
    $adc_ofr3:ident,
    $adc_ofr4:ident,
    $adc_jdr1:ident,
    $adc_jdr2:ident,
    $adc_jdr3:ident,
    $adc_jdr4:ident,
    $adc_awd2cr:ident,
    $adc_awd3cr:ident,
    $adc_difsel:ident,
    $adc_calfact:ident,
    $dma_req_id:expr,
  ) => {
    #[doc = $doc]
    pub type $name<I, C> = Adc<$name_res<I, Crt>, C>;

    #[doc = $doc_res]
    #[allow(missing_docs)]
    pub struct $name_res<I: $int_ty<Att>, Rt: RegTag> {
      pub $int: I,
      pub $adc_isr: $adc::Isr<Rt>,
      pub $adc_ier: $adc::Ier<Srt>,
      pub $adc_cr: $adc::Cr<Srt>,
      pub $adc_cfgr: $adc::Cfgr<Srt>,
      pub $adc_cfgr2: $adc::Cfgr2<Srt>,
      pub $adc_smpr1: $adc::Smpr1<Srt>,
      pub $adc_smpr2: $adc::Smpr2<Srt>,
      pub $adc_tr1: $adc::Tr1<Srt>,
      pub $adc_tr2: $adc::Tr2<Srt>,
      pub $adc_tr3: $adc::Tr3<Srt>,
      pub $adc_sqr1: $adc::Sqr1<Srt>,
      pub $adc_sqr2: $adc::Sqr2<Srt>,
      pub $adc_sqr3: $adc::Sqr3<Srt>,
      pub $adc_sqr4: $adc::Sqr4<Srt>,
      pub $adc_dr: $adc::Dr<Srt>,
      pub $adc_jsqr: $adc::Jsqr<Srt>,
      pub $adc_ofr1: $adc::Ofr1<Srt>,
      pub $adc_ofr2: $adc::Ofr2<Srt>,
      pub $adc_ofr3: $adc::Ofr3<Srt>,
      pub $adc_ofr4: $adc::Ofr4<Srt>,
      pub $adc_jdr1: $adc::Jdr1<Srt>,
      pub $adc_jdr2: $adc::Jdr2<Srt>,
      pub $adc_jdr3: $adc::Jdr3<Srt>,
      pub $adc_jdr4: $adc::Jdr4<Srt>,
      pub $adc_awd2cr: $adc::Awd2Cr<Srt>,
      pub $adc_awd3cr: $adc::Awd3Cr<Srt>,
      pub $adc_difsel: $adc::Difsel<Srt>,
      pub $adc_calfact: $adc::Calfact<Srt>,
      pub $rcc_ahb_enr_adcen: rcc::$ahb_enr::$adcen_ty<Rt>,
    }

    #[doc = $doc_on]
    pub type $name_on<I> = AdcOn<$name_res<I, Crt>>;

    /// Creates a new `Adc`.
    #[macro_export]
    macro_rules! $name_macro {
      ($reg:ident, $thr:ident, $rgc:path) => {
        <$crate::adc::Adc<_, $rgc> as ::drone_core::drv::Driver>::new(
          $crate::adc::$name_res {
            $int: $thr.$int.to_attach(),
            $adc_isr: $reg.$adc_isr,
            $adc_ier: $reg.$adc_ier,
            $adc_cr: $reg.$adc_cr,
            $adc_cfgr: $reg.$adc_cfgr,
            $adc_cfgr2: $reg.$adc_cfgr2,
            $adc_smpr1: $reg.$adc_smpr1,
            $adc_smpr2: $reg.$adc_smpr2,
            $adc_tr1: $reg.$adc_tr1,
            $adc_tr2: $reg.$adc_tr2,
            $adc_tr3: $reg.$adc_tr3,
            $adc_sqr1: $reg.$adc_sqr1,
            $adc_sqr2: $reg.$adc_sqr2,
            $adc_sqr3: $reg.$adc_sqr3,
            $adc_sqr4: $reg.$adc_sqr4,
            $adc_dr: $reg.$adc_dr,
            $adc_jsqr: $reg.$adc_jsqr,
            $adc_ofr1: $reg.$adc_ofr1,
            $adc_ofr2: $reg.$adc_ofr2,
            $adc_ofr3: $reg.$adc_ofr3,
            $adc_ofr4: $reg.$adc_ofr4,
            $adc_jdr1: $reg.$adc_jdr1,
            $adc_jdr2: $reg.$adc_jdr2,
            $adc_jdr3: $reg.$adc_jdr3,
            $adc_jdr4: $reg.$adc_jdr4,
            $adc_awd2cr: $reg.$adc_awd2cr,
            $adc_awd3cr: $reg.$adc_awd3cr,
            $adc_difsel: $reg.$adc_difsel,
            $adc_calfact: $reg.$adc_calfact,
            $rcc_ahb_enr_adcen: $reg.$rcc_ahb_enr.$adcen,
          },
        )
      };
    }

    impl<I: $int_ty<Att>> Resource for $name_res<I, Crt> {
      type Source = $name_res<I, Srt>;

      #[inline(always)]
      fn from_source(source: Self::Source) -> Self {
        Self {
          $int: source.$int,
          $adc_isr: source.$adc_isr.to_copy(),
          $adc_ier: source.$adc_ier,
          $adc_cr: source.$adc_cr,
          $adc_cfgr: source.$adc_cfgr,
          $adc_cfgr2: source.$adc_cfgr2,
          $adc_smpr1: source.$adc_smpr1,
          $adc_smpr2: source.$adc_smpr2,
          $adc_tr1: source.$adc_tr1,
          $adc_tr2: source.$adc_tr2,
          $adc_tr3: source.$adc_tr3,
          $adc_sqr1: source.$adc_sqr1,
          $adc_sqr2: source.$adc_sqr2,
          $adc_sqr3: source.$adc_sqr3,
          $adc_sqr4: source.$adc_sqr4,
          $adc_dr: source.$adc_dr,
          $adc_jsqr: source.$adc_jsqr,
          $adc_ofr1: source.$adc_ofr1,
          $adc_ofr2: source.$adc_ofr2,
          $adc_ofr3: source.$adc_ofr3,
          $adc_ofr4: source.$adc_ofr4,
          $adc_jdr1: source.$adc_jdr1,
          $adc_jdr2: source.$adc_jdr2,
          $adc_jdr3: source.$adc_jdr3,
          $adc_jdr4: source.$adc_jdr4,
          $adc_awd2cr: source.$adc_awd2cr,
          $adc_awd3cr: source.$adc_awd3cr,
          $adc_difsel: source.$adc_difsel,
          $adc_calfact: source.$adc_calfact,
          $rcc_ahb_enr_adcen: source.$rcc_ahb_enr_adcen.to_copy(),
        }
      }
    }

    impl<I: $int_ty<Att>> AdcRes for $name_res<I, Crt> {
      type Int = I;
      type Ier = $adc::Ier<Srt>;
      type Cr = $adc::Cr<Srt>;
      type Cfgr = $adc::Cfgr<Srt>;
      type Cfgr2 = $adc::Cfgr2<Srt>;
      type Smpr1 = $adc::Smpr1<Srt>;
      type Smpr2 = $adc::Smpr2<Srt>;
      type Tr1 = $adc::Tr1<Srt>;
      type Tr2 = $adc::Tr2<Srt>;
      type Tr3 = $adc::Tr3<Srt>;
      type Sqr1 = $adc::Sqr1<Srt>;
      type Sqr2 = $adc::Sqr2<Srt>;
      type Sqr3 = $adc::Sqr3<Srt>;
      type Sqr4 = $adc::Sqr4<Srt>;
      type Dr = $adc::Dr<Srt>;
      type Jsqr = $adc::Jsqr<Srt>;
      type Ofr1 = $adc::Ofr1<Srt>;
      type Ofr2 = $adc::Ofr2<Srt>;
      type Ofr3 = $adc::Ofr3<Srt>;
      type Ofr4 = $adc::Ofr4<Srt>;
      type Jdr1 = $adc::Jdr1<Srt>;
      type Jdr2 = $adc::Jdr2<Srt>;
      type Jdr3 = $adc::Jdr3<Srt>;
      type Jdr4 = $adc::Jdr4<Srt>;
      type Awd2Cr = $adc::Awd2Cr<Srt>;
      type Awd3Cr = $adc::Awd3Cr<Srt>;
      type Difsel = $adc::Difsel<Srt>;
      type Calfact = $adc::Calfact<Srt>;
      type RccAhbEnrVal = rcc::$ahb_enr::Val;
      type RccAhbEnr = rcc::$ahb_enr::Reg<Crt>;
      type RccAhbEnrAdcEn = rcc::$ahb_enr::$adcen_ty<Crt>;

      #[inline(always)]
      fn int(&self) -> Self::Int {
        self.$int
      }

      res_impl!(Ier, ier, $adc_ier);
      res_impl!(Cr, cr, $adc_cr);
      res_impl!(Cfgr, cfgr, $adc_cfgr);
      res_impl!(Cfgr2, cfgr2, $adc_cfgr2);
      res_impl!(Smpr1, smpr1, $adc_smpr1);
      res_impl!(Smpr2, smpr2, $adc_smpr2);
      res_impl!(Tr1, tr1, $adc_tr1);
      res_impl!(Tr2, tr2, $adc_tr2);
      res_impl!(Tr3, tr3, $adc_tr3);
      res_impl!(Sqr1, sqr1, $adc_sqr1);
      res_impl!(Sqr2, sqr2, $adc_sqr2);
      res_impl!(Sqr3, sqr3, $adc_sqr3);
      res_impl!(Sqr4, sqr4, $adc_sqr4);
      res_impl!(Dr, dr, $adc_dr);
      res_impl!(Jsqr, jsqr, $adc_jsqr);
      res_impl!(Ofr1, ofr1, $adc_ofr1);
      res_impl!(Ofr2, ofr2, $adc_ofr2);
      res_impl!(Ofr3, ofr3, $adc_ofr3);
      res_impl!(Ofr4, ofr4, $adc_ofr4);
      res_impl!(Jdr1, jdr1, $adc_jdr1);
      res_impl!(Jdr2, jdr2, $adc_jdr2);
      res_impl!(Jdr3, jdr3, $adc_jdr3);
      res_impl!(Jdr4, jdr4, $adc_jdr4);
      res_impl!(Awd2Cr, awd2cr, $adc_awd2cr);
      res_impl!(Awd3Cr, awd3cr, $adc_awd3cr);
      res_impl!(Difsel, difsel, $adc_difsel);
      res_impl!(Calfact, calfact, $adc_calfact);
      res_impl!(RccAhbEnrAdcEn, rcc_en, $rcc_ahb_enr_adcen);
    }

    impl<I: $int_ty<Att>> AdcResIsr for $name_res<I, Crt> {
      type Isr = $adc::isr::Reg<Crt>;
      type IsrJqovf = $adc::isr::Jqovf<Crt>;
      type IsrAwd3 = $adc::isr::Awd3<Crt>;
      type IsrAwd2 = $adc::isr::Awd2<Crt>;
      type IsrAwd1 = $adc::isr::Awd1<Crt>;
      type IsrJeos = $adc::isr::Jeos<Crt>;
      type IsrJeoc = $adc::isr::Jeoc<Crt>;
      type IsrOvr = $adc::isr::Ovr<Crt>;
      type IsrEos = $adc::isr::Eos<Crt>;
      type IsrEoc = $adc::isr::Eoc<Crt>;
      type IsrEosmp = $adc::isr::Eosmp<Crt>;
      type IsrAdrdy = $adc::isr::Adrdy<Crt>;

      res_impl!(Isr, isr, $adc_isr);
      res_impl!(IsrJqovf, isr_jqovf, $adc_isr.jqovf);
      res_impl!(IsrAwd3, isr_awd3, $adc_isr.awd3);
      res_impl!(IsrAwd2, isr_awd2, $adc_isr.awd2);
      res_impl!(IsrAwd1, isr_awd1, $adc_isr.awd1);
      res_impl!(IsrJeos, isr_jeos, $adc_isr.jeos);
      res_impl!(IsrJeoc, isr_jeoc, $adc_isr.jeoc);
      res_impl!(IsrOvr, isr_ovr, $adc_isr.ovr);
      res_impl!(IsrEos, isr_eos, $adc_isr.eos);
      res_impl!(IsrEoc, isr_eoc, $adc_isr.eoc);
      res_impl!(IsrEosmp, isr_eosmp, $adc_isr.eosmp);
      res_impl!(IsrAdrdy, isr_adrdy, $adc_isr.adrdy);
    }

    #[cfg(any(
      feature = "stm32l4r5",
      feature = "stm32l4r7",
      feature = "stm32l4r9",
      feature = "stm32l4s5",
      feature = "stm32l4s7",
      feature = "stm32l4s9"
    ))]
    impl<I, T> AdcDmaRes<T> for $name_res<I, Crt>
    where
      I: $int_ty<Att>,
      T: DmaBond,
    {
      #[inline(always)]
      fn dmamux_init(
        &self,
        cr_val: &mut <<T::DmamuxChRes as DmamuxChRes>::Cr as Reg<Srt>>::Val,
        dmamux: &DmamuxCh<T::DmamuxChRes>,
      ) {
        dmamux.cr_dmareq_id().write(cr_val, $dma_req_id);
      }
    }
  };
}

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
/// Creates a new `AdcCom`.
#[macro_export]
macro_rules! drv_adc_com {
  ($reg:ident, $rgc:path) => {
    <$crate::adc::AdcCom<_, $rgc> as ::drone_core::drv::Driver>::new(
      $crate::adc::AdcComRes {
        adc_common_ccr: $reg.adc_common_ccr,
        adc_common_csr: $reg.adc_common_csr,
      },
    )
  };
}

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
adc! {
  "ADC driver.",
  Adc1,
  drv_adc1,
  "ADC resource.",
  Adc1Res,
  "ADC clock on guard resource.",
  Adc1On,
  IntAdc1,
  Adcen,
  ahb2enr,
  rcc_ahb2enr_adcen,
  rcc_ahb2enr,
  adcen,
  adc1,
  adc,
  adc_isr,
  adc_ier,
  adc_cr,
  adc_cfgr,
  adc_cfgr2,
  adc_smpr1,
  adc_smpr2,
  adc_tr1,
  adc_tr2,
  adc_tr3,
  adc_sqr1,
  adc_sqr2,
  adc_sqr3,
  adc_sqr4,
  adc_dr,
  adc_jsqr,
  adc_ofr1,
  adc_ofr2,
  adc_ofr3,
  adc_ofr4,
  adc_jdr1,
  adc_jdr2,
  adc_jdr3,
  adc_jdr4,
  adc_awd2cr,
  adc_awd3cr,
  adc_difsel,
  adc_calfact,
  5,
}
