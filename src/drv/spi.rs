//! Serial Peripheral Interface.

#[allow(unused_imports)]
use core::ptr::{read_volatile, write_volatile};
use drone_core::bitfield::Bitfield;
use drone_core::drv::Resource;
use drv::dma::{Dma, DmaRes};
#[cfg(any(
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
))]
use drv::dma::{
  Dma1Ch2Res, Dma1Ch3Res, Dma1Ch4Res, Dma1Ch5Res, Dma2Ch1Res, Dma2Ch2Res,
};
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
use drv::dma::{Dma2Ch3Res, Dma2Ch4Res};
#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drv::dmamux::{DmamuxCh, DmamuxChRes};
use reg::marker::*;
use reg::prelude::*;
#[cfg(any(
  feature = "stm32f100",
  feature = "stm32f101",
  feature = "stm32f102",
  feature = "stm32f103",
  feature = "stm32f107",
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use reg::{spi1, spi2, spi3};
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6"
))]
use thr::int::{
  IntDma1Ch2, IntDma1Ch3, IntDma1Ch4, IntDma1Ch5, IntDma2Ch1, IntDma2Ch2,
  IntDma2Ch3, IntDma2Ch4,
};
#[cfg(any(
  feature = "stm32f100",
  feature = "stm32f101",
  feature = "stm32f102",
  feature = "stm32f103",
  feature = "stm32f107",
  feature = "stm32l4x3",
  feature = "stm32l4x5"
))]
use thr::int::{
  IntDma1Channel2 as IntDma1Ch2, IntDma1Channel3 as IntDma1Ch3,
  IntDma1Channel4 as IntDma1Ch4, IntDma1Channel5 as IntDma1Ch5,
  IntDma2Channel1 as IntDma2Ch1, IntDma2Channel2 as IntDma2Ch2,
};
#[cfg(any(feature = "stm32l4x3", feature = "stm32l4x5"))]
use thr::int::{IntDma2Channel3 as IntDma2Ch3, IntDma2Channel4 as IntDma2Ch4};
#[cfg(any(
  feature = "stm32f100",
  feature = "stm32f101",
  feature = "stm32f102",
  feature = "stm32f103",
  feature = "stm32f107",
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use thr::int::{IntSpi1, IntSpi2, IntSpi3};
use thr::prelude::*;

/// Motorola SPI mode error.
#[derive(Debug, Fail)]
pub enum SpiError {
  /// CRC value received does not match the `SPIx_RXCRCR` value.
  #[fail(display = "SPI CRC mismatch.")]
  Crcerr,
  /// Overrun occurred.
  #[fail(display = "SPI queue overrun.")]
  Ovr,
  /// Mode fault occurred.
  #[fail(display = "SPI mode fault.")]
  Modf,
}

/// SPI driver.
#[derive(Driver)]
pub struct Spi<T: SpiRes>(T);

/// DMA-driven SPI driver.
pub trait SpiDmaRx<T, Rx>
where
  T: SpiDmaRxRes<Rx>,
  Rx: DmaRes,
{
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  /// Initializes DMA for the SPI as peripheral.
  fn dma_rx_init(
    &self,
    dma_rx: &Dma<Rx>,
    dmamux_rx: &DmamuxCh<Rx::DmamuxChRes>,
  );

  #[cfg(not(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  )))]
  /// Initializes DMA for the SPI as peripheral.
  fn dma_rx_init(&self, dma_rx: &Dma<Rx>);

  /// Initializes DMA for the SPI as peripheral.
  fn dma_rx_paddr_init(&self, dma_rx: &Dma<Rx>);
}

/// DMA-driven SPI driver.
pub trait SpiDmaTx<T, Tx>
where
  T: SpiDmaTxRes<Tx>,
  Tx: DmaRes,
{
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  /// Initializes DMA for the SPI as peripheral.
  fn dma_tx_init(
    &self,
    dma_tx: &Dma<Tx>,
    dmamux_tx: &DmamuxCh<Tx::DmamuxChRes>,
  );

  #[cfg(not(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  )))]
  /// Initializes DMA for the SPI as peripheral.
  fn dma_tx_init(&self, dma_tx: &Dma<Tx>);

  /// Initializes DMA for the SPI as peripheral.
  fn dma_tx_paddr_init(&self, dma_tx: &Dma<Tx>);
}

/// DMA-driven SPI driver.
pub trait SpiDmaDx<T, Rx, Tx>
where
  T: SpiDmaRxRes<Rx> + SpiDmaTxRes<Tx>,
  Rx: DmaRes,
  Tx: DmaRes,
{
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  /// Initializes DMA for the SPI as peripheral.
  fn dma_dx_init(
    &self,
    dma_rx: &Dma<Rx>,
    dmamux_rx: &DmamuxCh<Rx::DmamuxChRes>,
    dma_tx: &Dma<Tx>,
    dmamux_tx: &DmamuxCh<Tx::DmamuxChRes>,
  );

  #[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  ))]
  /// Initializes DMA for the SPI as peripheral.
  fn dma_dx_init(&self, dma_rx: &Dma<Rx>, dma_tx: &Dma<Tx>)
  where
    Tx: DmaRes<Cselr = Rx::Cselr>;

  #[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  ))]
  /// Initializes DMA for the SPI as peripheral.
  fn dma_dx_init(&self, dma_rx: &Dma<Rx>, dma_tx: &Dma<Tx>);

  /// Initializes DMA for the SPI as peripheral.
  fn dma_dx_paddr_init(&self, dma_rx: &Dma<Rx>, dma_tx: &Dma<Tx>);
}

/// SPI resource.
#[allow(missing_docs)]
pub trait SpiRes: Resource {
  type Cr1Val: Bitfield<Bits = u32>;
  type Cr1: SRwRegBitBand<Val = Self::Cr1Val>;
  type Cr1Bidimode: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Bidioe: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Rxonly: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Lsbfirst: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Spe: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Mstr: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Cpol: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr1Cpha: SRwRwRegFieldBitBand<Reg = Self::Cr1>;
  type Cr2Val: Bitfield<Bits = u32>;
  type Cr2: SRwRegBitBand<Val = Self::Cr2Val>;
  type Cr2Txeie: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2Rxneie: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2Errie: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2Txdmaen: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Cr2Rxdmaen: SRwRwRegFieldBitBand<Reg = Self::Cr2>;
  type Crcpr: SRwRegBitBand;
  type Dr: FRwRegBitBand;
  type Rxcrcr: SRoRegBitBand;
  type SrVal: Bitfield<Bits = u32>;
  type Sr: SRwRegBitBand<Val = Self::SrVal>;
  type SrBsy: SRoRwRegFieldBitBand<Reg = Self::Sr>;
  type SrOvr: SRoRwRegFieldBitBand<Reg = Self::Sr>;
  type SrModf: SRoRwRegFieldBitBand<Reg = Self::Sr>;
  type SrCrcerr: SRwRwRegFieldBitBand<Reg = Self::Sr>;
  type SrRxne: SRoRwRegFieldBitBand<Reg = Self::Sr>;
  type Txcrcr: SRoRegBitBand;

  res_reg_decl!(Cr1, cr1, cr1_mut);
  res_reg_decl!(Cr1Bidimode, cr1_bidimode, cr1_bidimode_mut);
  res_reg_decl!(Cr1Bidioe, cr1_bidioe, cr1_bidioe_mut);
  res_reg_decl!(Cr1Rxonly, cr1_rxonly, cr1_rxonly_mut);
  res_reg_decl!(Cr1Lsbfirst, cr1_lsbfirst, cr1_lsbfirst_mut);
  res_reg_decl!(Cr1Spe, cr1_spe, cr1_spe_mut);
  res_reg_decl!(Cr1Mstr, cr1_mstr, cr1_mstr_mut);
  res_reg_decl!(Cr1Cpol, cr1_cpol, cr1_cpol_mut);
  res_reg_decl!(Cr1Cpha, cr1_cpha, cr1_cpha_mut);
  res_reg_decl!(Cr2, cr2, cr2_mut);
  res_reg_decl!(Cr2Txeie, cr2_txeie, cr2_txeie_mut);
  res_reg_decl!(Cr2Rxneie, cr2_rxneie, cr2_rxneie_mut);
  res_reg_decl!(Cr2Errie, cr2_errie, cr2_errie_mut);
  res_reg_decl!(Cr2Txdmaen, cr2_txdmaen, cr2_txdmaen_mut);
  res_reg_decl!(Cr2Rxdmaen, cr2_rxdmaen, cr2_rxdmaen_mut);
  res_reg_decl!(Crcpr, crcpr, crcpr_mut);
  res_reg_decl!(Dr, dr, dr_mut);
  res_reg_decl!(Rxcrcr, rxcrcr, rxcrcr_mut);
  res_reg_decl!(Sr, sr, sr_mut);
  res_reg_decl!(SrBsy, sr_bsy, sr_bsy_mut);
  res_reg_decl!(SrOvr, sr_ovr, sr_ovr_mut);
  res_reg_decl!(SrModf, sr_modf, sr_modf_mut);
  res_reg_decl!(SrCrcerr, sr_crcerr, sr_crcerr_mut);
  res_reg_decl!(SrRxne, sr_rxne, sr_rxne_mut);
  res_reg_decl!(Txcrcr, txcrcr, txcrcr_mut);

  #[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6",
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  fn set_frame_8(&self, cr2: &mut Self::Cr2Val);
}

/// Interrupt-driven SPI resource.
#[allow(missing_docs)]
pub trait SpiIntRes: SpiRes {
  type WithoutInt: SpiRes;
  type Int: IntToken<Ttt>;

  fn join_int(res: Self::WithoutInt, int: Self::Int) -> Self;
  fn split_int(self) -> (Self::WithoutInt, Self::Int);

  fn int(&self) -> Self::Int;
}

/// DMA-driven SPI resource.
#[allow(missing_docs)]
pub trait SpiDmaRxRes<T: DmaRes>: SpiRes {
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  fn dmamux_rx_init(
    &self,
    cr_val: &mut DmamuxCrVal<T::DmamuxChRes>,
    dmamux: &DmamuxCh<T::DmamuxChRes>,
  );

  #[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  ))]
  fn dma_rx_ch_init(&self, cs_val: &mut CselrVal<T>, dma: &Dma<T>);
}

/// DMA-driven SPI resource.
#[allow(missing_docs)]
pub trait SpiDmaTxRes<T: DmaRes>: SpiRes {
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  fn dmamux_tx_init(
    &self,
    cr_val: &mut DmamuxCrVal<T::DmamuxChRes>,
    dmamux: &DmamuxCh<T::DmamuxChRes>,
  );

  #[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  ))]
  fn dma_tx_ch_init(&self, cs_val: &mut CselrVal<T>, dma: &Dma<T>);
}

#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
type DmamuxCrVal<T> = <<T as DmamuxChRes>::Cr as Reg<Srt>>::Val;

#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6"
))]
type CselrVal<T> = <<T as DmaRes>::Cselr as Reg<Srt>>::Val;

#[allow(missing_docs)]
impl<T: SpiRes> Spi<T> {
  #[inline(always)]
  pub fn cr1(&self) -> &T::Cr1 {
    self.0.cr1()
  }

  #[inline(always)]
  pub fn cr1_bidimode(&self) -> &T::Cr1Bidimode {
    self.0.cr1_bidimode()
  }

  #[inline(always)]
  pub fn cr1_bidioe(&self) -> &T::Cr1Bidioe {
    self.0.cr1_bidioe()
  }

  #[inline(always)]
  pub fn cr1_rxonly(&self) -> &T::Cr1Rxonly {
    self.0.cr1_rxonly()
  }

  #[inline(always)]
  pub fn cr1_lsbfirst(&self) -> &T::Cr1Lsbfirst {
    self.0.cr1_lsbfirst()
  }

  #[inline(always)]
  pub fn cr1_spe(&self) -> &T::Cr1Spe {
    self.0.cr1_spe()
  }

  #[inline(always)]
  pub fn cr1_mstr(&self) -> &T::Cr1Mstr {
    self.0.cr1_mstr()
  }

  #[inline(always)]
  pub fn cr1_cpol(&self) -> &T::Cr1Cpol {
    self.0.cr1_cpol()
  }

  #[inline(always)]
  pub fn cr1_cpha(&self) -> &T::Cr1Cpha {
    self.0.cr1_cpha()
  }

  #[inline(always)]
  pub fn cr2(&self) -> &T::Cr2 {
    self.0.cr2()
  }

  #[inline(always)]
  pub fn cr2_txeie(&self) -> &T::Cr2Txeie {
    self.0.cr2_txeie()
  }

  #[inline(always)]
  pub fn cr2_rxneie(&self) -> &T::Cr2Rxneie {
    self.0.cr2_rxneie()
  }

  #[inline(always)]
  pub fn cr2_errie(&self) -> &T::Cr2Errie {
    self.0.cr2_errie()
  }

  #[inline(always)]
  pub fn cr2_txdmaen(&self) -> &T::Cr2Txdmaen {
    self.0.cr2_txdmaen()
  }

  #[inline(always)]
  pub fn cr2_rxdmaen(&self) -> &T::Cr2Rxdmaen {
    self.0.cr2_rxdmaen()
  }

  #[inline(always)]
  pub fn crcpr(&self) -> &T::Crcpr {
    self.0.crcpr()
  }

  #[inline(always)]
  pub fn dr(&self) -> &T::Dr {
    self.0.dr()
  }

  #[inline(always)]
  pub fn rxcrcr(&self) -> &T::Rxcrcr {
    self.0.rxcrcr()
  }

  #[inline(always)]
  pub fn sr(&self) -> &T::Sr {
    self.0.sr()
  }

  #[inline(always)]
  pub fn sr_bsy(&self) -> &T::SrBsy {
    self.0.sr_bsy()
  }

  #[inline(always)]
  pub fn sr_ovr(&self) -> &T::SrOvr {
    self.0.sr_ovr()
  }

  #[inline(always)]
  pub fn sr_modf(&self) -> &T::SrModf {
    self.0.sr_modf()
  }

  #[inline(always)]
  pub fn sr_crcerr(&self) -> &T::SrCrcerr {
    self.0.sr_crcerr()
  }

  #[inline(always)]
  pub fn sr_rxne(&self) -> &T::SrRxne {
    self.0.sr_rxne()
  }

  #[inline(always)]
  pub fn txcrcr(&self) -> &T::Txcrcr {
    self.0.txcrcr()
  }

  /// Sets the size of a data frame to 8 bits.
  #[inline(always)]
  pub fn set_frame_8(&self, _cr2: &mut T::Cr2Val) {
    #[cfg(any(
      feature = "stm32l4x1",
      feature = "stm32l4x2",
      feature = "stm32l4x3",
      feature = "stm32l4x5",
      feature = "stm32l4x6",
      feature = "stm32l4r5",
      feature = "stm32l4r7",
      feature = "stm32l4r9",
      feature = "stm32l4s5",
      feature = "stm32l4s7",
      feature = "stm32l4s9"
    ))]
    self.0.set_frame_8(_cr2);
  }

  /// Writes a byte to the data register.
  #[inline(always)]
  pub fn send_byte(&self, value: u8) {
    Self::dr_send_byte(self.0.dr(), value);
  }

  /// Returns a closure, which writes a byte to the data register.
  #[inline(always)]
  pub fn send_byte_fn(&mut self) -> impl Fn(u8) {
    let dr = self.0.dr_mut().fork();
    move |value| Self::dr_send_byte(&dr, value)
  }

  /// Writes a half word to the data register.
  #[inline(always)]
  pub fn send_hword(&self, value: u16) {
    Self::dr_send_hword(self.0.dr(), value);
  }

  /// Returns a closure, which writes a half word to the data register.
  #[inline(always)]
  pub fn send_hword_fn(&mut self) -> impl Fn(u16) {
    let dr = self.0.dr_mut().fork();
    move |value| Self::dr_send_hword(&dr, value)
  }

  /// Reads a byte from the data register.
  #[inline(always)]
  pub fn recv_byte(&self) -> u8 {
    unsafe { read_volatile(self.0.dr().to_ptr() as *const _) }
  }

  /// Reads a half word from the data register.
  #[inline(always)]
  pub fn recv_hword(&self) -> u16 {
    unsafe { read_volatile(self.0.dr().to_ptr() as *const _) }
  }

  /// Waits while SPI is busy in communication or Tx buffer is not empty.
  #[inline(always)]
  pub fn busy_wait(&self) {
    while self.0.sr_bsy().read_bit_band() {}
  }

  /// Checks for SPI mode errors.
  #[inline(always)]
  pub fn spi_errck(&self, sr: &T::SrVal) -> Result<(), SpiError> {
    if self.sr_ovr().read(sr) {
      Err(SpiError::Ovr)
    } else if self.sr_modf().read(sr) {
      Err(SpiError::Modf)
    } else if self.sr_crcerr().read(sr) {
      Err(SpiError::Crcerr)
    } else {
      Ok(())
    }
  }

  #[inline(always)]
  fn dr_send_byte(dr: &T::Dr, value: u8) {
    unsafe { write_volatile(dr.to_mut_ptr() as *mut _, value) };
  }

  #[inline(always)]
  fn dr_send_hword(dr: &T::Dr, value: u16) {
    unsafe { write_volatile(dr.to_mut_ptr() as *mut _, value) };
  }
}

#[allow(missing_docs)]
impl<T: SpiIntRes> Spi<T> {
  #[inline(always)]
  pub fn join_int(res: Spi<T::WithoutInt>, int: T::Int) -> Spi<T> {
    Spi(T::join_int(res.0, int))
  }

  #[inline(always)]
  pub fn split_int(self) -> (Spi<T::WithoutInt>, T::Int) {
    let (res, int) = self.0.split_int();
    (Spi(res), int)
  }

  #[inline(always)]
  pub fn int(&self) -> T::Int {
    self.0.int()
  }
}

#[allow(missing_docs)]
impl<T, Rx> SpiDmaRx<T, Rx> for Spi<T>
where
  T: SpiDmaRxRes<Rx>,
  Rx: DmaRes,
{
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  #[inline(always)]
  fn dma_rx_init(
    &self,
    dma_rx: &Dma<Rx>,
    dmamux_rx: &DmamuxCh<Rx::DmamuxChRes>,
  ) {
    self.dma_rx_paddr_init(dma_rx);
    dmamux_rx.cr_dmareq_id().modify(|r| {
      self.0.dmamux_rx_init(r, dmamux_rx);
    });
  }

  #[cfg(not(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  )))]
  #[inline(always)]
  fn dma_rx_init(&self, dma_rx: &Dma<Rx>) {
    self.dma_rx_paddr_init(dma_rx);
    #[cfg(any(
      feature = "stm32l4x1",
      feature = "stm32l4x2",
      feature = "stm32l4x3",
      feature = "stm32l4x5",
      feature = "stm32l4x6"
    ))]
    dma_rx.cselr_cs().modify(|r| {
      self.0.dma_rx_ch_init(r, dma_rx);
    });
  }

  #[inline(always)]
  fn dma_rx_paddr_init(&self, dma_rx: &Dma<Rx>) {
    unsafe { dma_rx.set_paddr(self.0.dr().to_ptr() as usize) };
  }
}

#[allow(missing_docs)]
impl<T, Tx> SpiDmaTx<T, Tx> for Spi<T>
where
  T: SpiDmaTxRes<Tx>,
  Tx: DmaRes,
{
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  #[inline(always)]
  fn dma_tx_init(
    &self,
    dma_tx: &Dma<Tx>,
    dmamux_tx: &DmamuxCh<Tx::DmamuxChRes>,
  ) {
    self.dma_tx_paddr_init(dma_tx);
    dmamux_tx.cr_dmareq_id().modify(|r| {
      self.0.dmamux_tx_init(r, dmamux_tx);
    });
  }

  #[cfg(not(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  )))]
  #[inline(always)]
  fn dma_tx_init(&self, dma_tx: &Dma<Tx>) {
    self.dma_tx_paddr_init(dma_tx);
    #[cfg(any(
      feature = "stm32l4x1",
      feature = "stm32l4x2",
      feature = "stm32l4x3",
      feature = "stm32l4x5",
      feature = "stm32l4x6"
    ))]
    dma_tx.cselr_cs().modify(|r| {
      self.0.dma_tx_ch_init(r, dma_tx);
    });
  }

  #[inline(always)]
  fn dma_tx_paddr_init(&self, dma_tx: &Dma<Tx>) {
    unsafe { dma_tx.set_paddr(self.0.dr().to_mut_ptr() as usize) };
  }
}

#[allow(missing_docs)]
impl<T, Rx, Tx> SpiDmaDx<T, Rx, Tx> for Spi<T>
where
  T: SpiDmaRxRes<Rx> + SpiDmaTxRes<Tx>,
  Rx: DmaRes,
  Tx: DmaRes,
{
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  fn dma_dx_init(
    &self,
    dma_rx: &Dma<Rx>,
    dmamux_rx: &DmamuxCh<Rx::DmamuxChRes>,
    dma_tx: &Dma<Tx>,
    dmamux_tx: &DmamuxCh<Tx::DmamuxChRes>,
  ) {
    self.dma_dx_paddr_init(dma_rx, dma_tx);
    dmamux_rx.cr_dmareq_id().modify(|r| {
      self.0.dmamux_rx_init(r, dmamux_rx);
    });
    dmamux_tx.cr_dmareq_id().modify(|r| {
      self.0.dmamux_tx_init(r, dmamux_tx);
    });
  }

  #[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  ))]
  #[inline(always)]
  fn dma_dx_init(&self, dma_rx: &Dma<Rx>, dma_tx: &Dma<Tx>)
  where
    Tx: DmaRes<Cselr = Rx::Cselr>,
  {
    self.dma_dx_paddr_init(dma_rx, dma_tx);
    dma_rx.cselr_cs().modify(|r| {
      self.0.dma_rx_ch_init(r, dma_rx);
      self.0.dma_tx_ch_init(r, dma_tx);
    });
  }

  #[cfg(any(
    feature = "stm32f100",
    feature = "stm32f101",
    feature = "stm32f102",
    feature = "stm32f103",
    feature = "stm32f107"
  ))]
  #[inline(always)]
  fn dma_dx_init(&self, dma_rx: &Dma<Rx>, dma_tx: &Dma<Tx>) {
    self.dma_dx_paddr_init(dma_rx, dma_tx);
  }

  #[inline(always)]
  fn dma_dx_paddr_init(&self, dma_rx: &Dma<Rx>, dma_tx: &Dma<Tx>) {
    self.dma_rx_paddr_init(dma_rx);
    self.dma_tx_paddr_init(dma_tx);
  }
}

#[allow(unused_macros)]
macro_rules! spi_shared {
  (
    $spi:ident,
    $spi_cr1:ident,
    $spi_cr2:ident,
    $spi_crcpr:ident,
    $spi_dr:ident,
    $spi_rxcrcr:ident,
    $spi_sr:ident,
    $spi_txcrcr:ident,
    $name_res:ident,
    ($($tp:ident: $bound:path),*),
    (
      $dma_rx_req_id:expr,
      $((
        [$($dma_rx_attr:meta,)*],
        $dma_rx_res:ident,
        $int_dma_rx:ident,
        $dma_rx_cs:expr,
        ($($dma_rx_tp:ident: $dma_rx_bound:path),*)
      ),)*
    ),
    (
      $dma_tx_req_id:expr,
      $((
        [$($dma_tx_attr:meta,)*],
        $dma_tx_res:ident,
        $int_dma_tx:ident,
        $dma_tx_cs:expr,
        ($($dma_tx_tp:ident: $dma_tx_bound:path),*)
      ),)*
    ),
  ) => {
    impl<$($tp: $bound,)*> SpiRes for $name_res<$($tp,)* Frt> {
      type Cr1Val = $spi::cr1::Val;
      type Cr1 = $spi::Cr1<Srt>;
      type Cr1Bidimode = $spi::cr1::Bidimode<Srt>;
      type Cr1Bidioe = $spi::cr1::Bidioe<Srt>;
      type Cr1Rxonly = $spi::cr1::Rxonly<Srt>;
      type Cr1Lsbfirst = $spi::cr1::Lsbfirst<Srt>;
      type Cr1Spe = $spi::cr1::Spe<Srt>;
      type Cr1Mstr = $spi::cr1::Mstr<Srt>;
      type Cr1Cpol = $spi::cr1::Cpol<Srt>;
      type Cr1Cpha = $spi::cr1::Cpha<Srt>;
      type Cr2Val = $spi::cr2::Val;
      type Cr2 = $spi::Cr2<Srt>;
      type Cr2Txeie = $spi::cr2::Txeie<Srt>;
      type Cr2Rxneie = $spi::cr2::Rxneie<Srt>;
      type Cr2Errie = $spi::cr2::Errie<Srt>;
      type Cr2Txdmaen = $spi::cr2::Txdmaen<Srt>;
      type Cr2Rxdmaen = $spi::cr2::Rxdmaen<Srt>;
      type Crcpr = $spi::Crcpr<Srt>;
      type Dr = $spi::Dr<Frt>;
      type Rxcrcr = $spi::Rxcrcr<Srt>;
      type SrVal = $spi::sr::Val;
      type Sr = $spi::Sr<Srt>;
      type SrBsy = $spi::sr::Bsy<Srt>;
      type SrOvr = $spi::sr::Ovr<Srt>;
      type SrModf = $spi::sr::Modf<Srt>;
      type SrCrcerr = $spi::sr::Crcerr<Srt>;
      type SrRxne = $spi::sr::Rxne<Srt>;
      type Txcrcr = $spi::Txcrcr<Srt>;

      res_reg_impl!(Cr1, cr1, cr1_mut, $spi_cr1);
      res_reg_field_impl!(Cr1Bidimode, cr1_bidimode, cr1_bidimode_mut,
                          $spi_cr1, bidimode);
      res_reg_field_impl!(Cr1Bidioe, cr1_bidioe, cr1_bidioe_mut, $spi_cr1,
                          bidioe);
      res_reg_field_impl!(Cr1Rxonly, cr1_rxonly, cr1_rxonly_mut, $spi_cr1,
                          rxonly);
      res_reg_field_impl!(Cr1Lsbfirst, cr1_lsbfirst, cr1_lsbfirst_mut, $spi_cr1,
                          lsbfirst);
      res_reg_field_impl!(Cr1Spe, cr1_spe, cr1_spe_mut, $spi_cr1, spe);
      res_reg_field_impl!(Cr1Mstr, cr1_mstr, cr1_mstr_mut, $spi_cr1, mstr);
      res_reg_field_impl!(Cr1Cpol, cr1_cpol, cr1_cpol_mut, $spi_cr1, cpol);
      res_reg_field_impl!(Cr1Cpha, cr1_cpha, cr1_cpha_mut, $spi_cr1, cpha);
      res_reg_impl!(Cr2, cr2, cr2_mut, $spi_cr2);
      res_reg_field_impl!(Cr2Txeie, cr2_txeie, cr2_txeie_mut, $spi_cr2, txeie);
      res_reg_field_impl!(Cr2Rxneie, cr2_rxneie, cr2_rxneie_mut, $spi_cr2,
                          rxneie);
      res_reg_field_impl!(Cr2Errie, cr2_errie, cr2_errie_mut, $spi_cr2, errie);
      res_reg_field_impl!(Cr2Txdmaen, cr2_txdmaen, cr2_txdmaen_mut, $spi_cr2,
                          txdmaen);
      res_reg_field_impl!(Cr2Rxdmaen, cr2_rxdmaen, cr2_rxdmaen_mut, $spi_cr2,
                          rxdmaen);
      res_reg_impl!(Crcpr, crcpr, crcpr_mut, $spi_crcpr);
      res_reg_impl!(Dr, dr, dr_mut, $spi_dr);
      res_reg_impl!(Rxcrcr, rxcrcr, rxcrcr_mut, $spi_rxcrcr);
      res_reg_impl!(Sr, sr, sr_mut, $spi_sr);
      res_reg_field_impl!(SrBsy, sr_bsy, sr_bsy_mut, $spi_sr, bsy);
      res_reg_field_impl!(SrOvr, sr_ovr, sr_ovr_mut, $spi_sr, ovr);
      res_reg_field_impl!(SrModf, sr_modf, sr_modf_mut, $spi_sr, modf);
      res_reg_field_impl!(SrCrcerr, sr_crcerr, sr_crcerr_mut, $spi_sr, crcerr);
      res_reg_field_impl!(SrRxne, sr_rxne, sr_rxne_mut, $spi_sr, rxne);
      res_reg_impl!(Txcrcr, txcrcr, txcrcr_mut, $spi_txcrcr);

      #[cfg(any(
        feature = "stm32l4x1",
        feature = "stm32l4x2",
        feature = "stm32l4x3",
        feature = "stm32l4x5",
        feature = "stm32l4x6",
        feature = "stm32l4r5",
        feature = "stm32l4r7",
        feature = "stm32l4r9",
        feature = "stm32l4s5",
        feature = "stm32l4s7",
        feature = "stm32l4s9"
      ))]
      #[inline(always)]
      fn set_frame_8(&self, cr2: &mut Self::Cr2Val) {
        self.$spi_cr2.frxth.set(cr2);
        self.$spi_cr2.ds.write(cr2, 0b0111);
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
    impl<$($tp,)* T> SpiDmaRxRes<T> for $name_res<$($tp,)* Frt>
    where
      T: DmaRes,
      $($tp: $bound,)*
    {
      #[inline(always)]
      fn dmamux_rx_init(
        &self,
        cr_val: &mut DmamuxCrVal<T::DmamuxChRes>,
        dmamux: &DmamuxCh<T::DmamuxChRes>,
      ) {
        dmamux.cr_dmareq_id().write(cr_val, $dma_rx_req_id);
      }
    }

    $(
      #[cfg(not(any(
        feature = "stm32l4r5",
        feature = "stm32l4r7",
        feature = "stm32l4r9",
        feature = "stm32l4s5",
        feature = "stm32l4s7",
        feature = "stm32l4s9"
      )))]
      $(#[$dma_rx_attr])*
      impl<$($dma_rx_tp,)* Rx> SpiDmaRxRes<$dma_rx_res<Rx, Frt>>
        for $name_res<$($dma_rx_tp,)* Frt>
      where
        Rx: $int_dma_rx<Ttt>,
        $($dma_rx_tp: $dma_rx_bound,)*
      {
        #[cfg(any(
          feature = "stm32l4x1",
          feature = "stm32l4x2",
          feature = "stm32l4x3",
          feature = "stm32l4x5",
          feature = "stm32l4x6"
        ))]
        #[inline(always)]
        fn dma_rx_ch_init(
          &self,
          cs_val: &mut CselrVal<$dma_rx_res<Rx, Frt>>,
          dma: &Dma<$dma_rx_res<Rx, Frt>>,
        ) {
          dma.cselr_cs().write(cs_val, $dma_rx_cs);
        }
      }
    )*

    #[cfg(any(
      feature = "stm32l4r5",
      feature = "stm32l4r7",
      feature = "stm32l4r9",
      feature = "stm32l4s5",
      feature = "stm32l4s7",
      feature = "stm32l4s9"
    ))]
    impl<$($tp,)* T> SpiDmaTxRes<T> for $name_res<$($tp,)* Frt>
    where
      T: DmaRes,
      $($tp: $bound,)*
    {
      #[inline(always)]
      fn dmamux_tx_init(
        &self,
        cr_val: &mut DmamuxCrVal<T::DmamuxChRes>,
        dmamux: &DmamuxCh<T::DmamuxChRes>,
      ) {
        dmamux.cr_dmareq_id().write(cr_val, $dma_tx_req_id);
      }
    }

    $(
      #[cfg(not(any(
        feature = "stm32l4r5",
        feature = "stm32l4r7",
        feature = "stm32l4r9",
        feature = "stm32l4s5",
        feature = "stm32l4s7",
        feature = "stm32l4s9"
      )))]
      $(#[$dma_tx_attr])*
      impl<$($dma_tx_tp,)* Tx> SpiDmaTxRes<$dma_tx_res<Tx, Frt>>
        for $name_res<$($dma_tx_tp,)* Frt>
      where
        Tx: $int_dma_tx<Ttt>,
        $($dma_tx_tp: $dma_tx_bound,)*
      {
        #[cfg(any(
          feature = "stm32l4x1",
          feature = "stm32l4x2",
          feature = "stm32l4x3",
          feature = "stm32l4x5",
          feature = "stm32l4x6"
        ))]
        #[inline(always)]
        fn dma_tx_ch_init(
          &self,
          cs_val: &mut CselrVal<$dma_tx_res<Tx, Frt>>,
          dma: &Dma<$dma_tx_res<Tx, Frt>>,
        ) {
          dma.cselr_cs().write(cs_val, $dma_tx_cs);
        }
      }
    )*
  }
}

#[allow(unused_macros)]
macro_rules! spi {
  (
    $doc:expr,
    $name:ident,
    $name_macro:ident,
    $doc_int:expr,
    $name_int:ident,
    $name_int_macro:ident,
    $doc_res:expr,
    $name_res:ident,
    $doc_int_res:expr,
    $name_int_res:ident,
    $int_ty:ident,
    $spi:ident,
    $spi_cr1:ident,
    $spi_cr2:ident,
    $spi_crcpr:ident,
    $spi_dr:ident,
    $spi_rxcrcr:ident,
    $spi_sr:ident,
    $spi_txcrcr:ident,
    (
      $dma_rx_req_id:expr,
      $((
        $(#[$dma_rx_attr:meta])*
        $dma_rx_res:ident,
        $int_dma_rx:ident,
        $dma_rx_cs:expr
      )),*
    ),
    (
      $dma_tx_req_id:expr,
      $((
        $(#[$dma_tx_attr:meta])*
        $dma_tx_res:ident,
        $int_dma_tx:ident,
        $dma_tx_cs:expr
      )),*
    ),
  ) => {
    #[doc = $doc]
    pub type $name = Spi<$name_res<Frt>>;

    #[doc = $doc_int]
    pub type $name_int<I> = Spi<$name_int_res<I, Frt>>;

    #[doc = $doc_res]
    #[allow(missing_docs)]
    pub struct $name_res<Rt: RegTag> {
      pub $spi_cr1: $spi::Cr1<Srt>,
      pub $spi_cr2: $spi::Cr2<Srt>,
      pub $spi_crcpr: $spi::Crcpr<Srt>,
      pub $spi_dr: $spi::Dr<Rt>,
      pub $spi_rxcrcr: $spi::Rxcrcr<Srt>,
      pub $spi_sr: $spi::Sr<Srt>,
      pub $spi_txcrcr: $spi::Txcrcr<Srt>,
    }

    #[doc = $doc_int_res]
    #[allow(missing_docs)]
    pub struct $name_int_res<I: $int_ty<Ttt>, Rt: RegTag> {
      pub $spi: I,
      pub $spi_cr1: $spi::Cr1<Srt>,
      pub $spi_cr2: $spi::Cr2<Srt>,
      pub $spi_crcpr: $spi::Crcpr<Srt>,
      pub $spi_dr: $spi::Dr<Rt>,
      pub $spi_rxcrcr: $spi::Rxcrcr<Srt>,
      pub $spi_sr: $spi::Sr<Srt>,
      pub $spi_txcrcr: $spi::Txcrcr<Srt>,
    }

    /// Creates a new `Spi`.
    #[macro_export]
    macro_rules! $name_macro {
      ($reg:ident) => {
        $crate::drv::spi::Spi::new(
          $crate::drv::spi::$name_res {
            $spi_cr1: $reg.$spi_cr1,
            $spi_cr2: $reg.$spi_cr2,
            $spi_crcpr: $reg.$spi_crcpr,
            $spi_dr: $reg.$spi_dr,
            $spi_rxcrcr: $reg.$spi_rxcrcr,
            $spi_sr: $reg.$spi_sr,
            $spi_txcrcr: $reg.$spi_txcrcr,
          }
        )
      }
    }

    /// Creates a new `SpiInt`.
    #[macro_export]
    macro_rules! $name_int_macro {
      ($reg:ident, $thr:ident) => {
        $crate::drv::spi::Spi::new(
          $crate::drv::spi::$name_int_res {
            $spi: $thr.$spi.into(),
            $spi_cr1: $reg.$spi_cr1,
            $spi_cr2: $reg.$spi_cr2,
            $spi_crcpr: $reg.$spi_crcpr,
            $spi_dr: $reg.$spi_dr,
            $spi_rxcrcr: $reg.$spi_rxcrcr,
            $spi_sr: $reg.$spi_sr,
            $spi_txcrcr: $reg.$spi_txcrcr,
          }
        )
      }
    }

    impl Resource for $name_res<Frt> {
      type Source = $name_res<Srt>;

      #[inline(always)]
      fn from_source(source: Self::Source) -> Self {
        Self {
          $spi_cr1: source.$spi_cr1,
          $spi_cr2: source.$spi_cr2,
          $spi_crcpr: source.$spi_crcpr,
          $spi_dr: source.$spi_dr.into(),
          $spi_rxcrcr: source.$spi_rxcrcr,
          $spi_sr: source.$spi_sr,
          $spi_txcrcr: source.$spi_txcrcr,
        }
      }
    }

    spi_shared! {
      $spi,
      $spi_cr1,
      $spi_cr2,
      $spi_crcpr,
      $spi_dr,
      $spi_rxcrcr,
      $spi_sr,
      $spi_txcrcr,
      $name_res,
      (),
      (
        $dma_rx_req_id,
        $(([$($dma_rx_attr,)*], $dma_rx_res, $int_dma_rx, $dma_rx_cs, ()),)*
      ),
      (
        $dma_tx_req_id,
        $(([$($dma_tx_attr,)*], $dma_tx_res, $int_dma_tx, $dma_tx_cs, ()),)*
      ),
    }

    impl<I: $int_ty<Ttt>> Resource for $name_int_res<I, Frt> {
      type Source = $name_int_res<I, Srt>;

      #[inline(always)]
      fn from_source(source: Self::Source) -> Self {
        Self {
          $spi: source.$spi,
          $spi_cr1: source.$spi_cr1,
          $spi_cr2: source.$spi_cr2,
          $spi_crcpr: source.$spi_crcpr,
          $spi_dr: source.$spi_dr.into(),
          $spi_rxcrcr: source.$spi_rxcrcr,
          $spi_sr: source.$spi_sr,
          $spi_txcrcr: source.$spi_txcrcr,
        }
      }
    }

    spi_shared! {
      $spi,
      $spi_cr1,
      $spi_cr2,
      $spi_crcpr,
      $spi_dr,
      $spi_rxcrcr,
      $spi_sr,
      $spi_txcrcr,
      $name_int_res,
      (I: $int_ty<Ttt>),
      (
        $dma_rx_req_id,
        $((
          [$($dma_rx_attr,)*], $dma_rx_res, $int_dma_rx, $dma_rx_cs,
          (I: $int_ty<Ttt>)
        ),)*
      ),
      (
        $dma_tx_req_id,
        $((
          [$($dma_tx_attr,)*], $dma_tx_res, $int_dma_tx, $dma_tx_cs,
          (I: $int_ty<Ttt>)
        ),)*
      ),
    }

    impl<I: $int_ty<Ttt>> SpiIntRes for $name_int_res<I, Frt> {
      type WithoutInt = $name_res<Frt>;
      type Int = I;

      #[inline(always)]
      fn join_int(res: Self::WithoutInt, int: Self::Int) -> Self {
        $name_int_res {
          $spi: int,
          $spi_cr1: res.$spi_cr1,
          $spi_cr2: res.$spi_cr2,
          $spi_crcpr: res.$spi_crcpr,
          $spi_dr: res.$spi_dr,
          $spi_rxcrcr: res.$spi_rxcrcr,
          $spi_sr: res.$spi_sr,
          $spi_txcrcr: res.$spi_txcrcr,
        }
      }

      #[inline(always)]
      fn split_int(self) -> (Self::WithoutInt, Self::Int) {
        (
          $name_res {
            $spi_cr1: self.$spi_cr1,
            $spi_cr2: self.$spi_cr2,
            $spi_crcpr: self.$spi_crcpr,
            $spi_dr: self.$spi_dr,
            $spi_rxcrcr: self.$spi_rxcrcr,
            $spi_sr: self.$spi_sr,
            $spi_txcrcr: self.$spi_txcrcr,
          },
          self.$spi,
        )
      }

      #[inline(always)]
      fn int(&self) -> Self::Int {
        self.$spi
      }
    }
  }
}

#[cfg(any(
  feature = "stm32f100",
  feature = "stm32f101",
  feature = "stm32f102",
  feature = "stm32f103",
  feature = "stm32f107",
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
spi! {
  "SPI1 driver.",
  Spi1,
  drv_spi1,
  "SPI1 driver with interrupt.",
  Spi1Int,
  drv_spi1_int,
  "SPI1 resource.",
  Spi1Res,
  "SPI1 resource with interrupt.",
  Spi1IntRes,
  IntSpi1,
  spi1,
  spi1_cr1,
  spi1_cr2,
  spi1_crcpr,
  spi1_dr,
  spi1_rxcrcr,
  spi1_sr,
  spi1_txcrcr,
  (
    10,
    (Dma1Ch2Res, IntDma1Ch2, 1),
    (
      #[cfg(any(
        feature = "stm32l4x1",
        feature = "stm32l4x2",
        feature = "stm32l4x3",
        feature = "stm32l4x5",
        feature = "stm32l4x6",
        feature = "stm32l4r5",
        feature = "stm32l4r7",
        feature = "stm32l4r9",
        feature = "stm32l4s5",
        feature = "stm32l4s7",
        feature = "stm32l4s9"
      ))]
      Dma2Ch3Res, IntDma2Ch3, 4
    )
  ),
  (
    11,
    (Dma1Ch3Res, IntDma1Ch3, 1),
    (
      #[cfg(any(
        feature = "stm32l4x1",
        feature = "stm32l4x2",
        feature = "stm32l4x3",
        feature = "stm32l4x5",
        feature = "stm32l4x6",
        feature = "stm32l4r5",
        feature = "stm32l4r7",
        feature = "stm32l4r9",
        feature = "stm32l4s5",
        feature = "stm32l4s7",
        feature = "stm32l4s9"
      ))]
      Dma2Ch4Res, IntDma2Ch4, 4
    )
  ),
}

#[cfg(any(
  feature = "stm32f100",
  feature = "stm32f101",
  feature = "stm32f102",
  feature = "stm32f103",
  feature = "stm32f107",
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
spi! {
  "SPI2 driver.",
  Spi2,
  drv_spi2,
  "SPI2 driver with interrupt.",
  Spi2Int,
  drv_spi2_int,
  "SPI2 resource.",
  Spi2Res,
  "SPI2 resource with interrupt.",
  Spi2IntRes,
  IntSpi2,
  spi2,
  spi2_cr1,
  spi2_cr2,
  spi2_crcpr,
  spi2_dr,
  spi2_rxcrcr,
  spi2_sr,
  spi2_txcrcr,
  (12, (Dma1Ch4Res, IntDma1Ch4, 1)),
  (13, (Dma1Ch5Res, IntDma1Ch5, 1)),
}

#[cfg(any(
  feature = "stm32f100",
  feature = "stm32f101",
  feature = "stm32f102",
  feature = "stm32f103",
  feature = "stm32f107",
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x3",
  feature = "stm32l4x5",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
spi! {
  "SPI3 driver.",
  Spi3,
  drv_spi3,
  "SPI3 driver with interrupt.",
  Spi3Int,
  drv_spi3_int,
  "SPI3 resource.",
  Spi3Res,
  "SPI3 resource with interrupt.",
  Spi3IntRes,
  IntSpi3,
  spi3,
  spi3_cr1,
  spi3_cr2,
  spi3_crcpr,
  spi3_dr,
  spi3_rxcrcr,
  spi3_sr,
  spi3_txcrcr,
  (14, (Dma2Ch1Res, IntDma2Ch1, 3)),
  (15, (Dma2Ch2Res, IntDma2Ch2, 3)),
}
