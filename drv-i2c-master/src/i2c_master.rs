//! I2C master session.

use drone_stm32_core::reg::prelude::*;
use drone_stm32_core::thr::prelude::*;
use drone_stm32_drv_dma::dma::{Dma, DmaRes, DmaTransferError};
#[cfg(any(
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drone_stm32_drv_dmamux::dmamux::DmamuxCh;
use drone_stm32_drv_i2c::i2c::{
  I2CBreak, I2CDmaDx, I2CDmaRxRes, I2CDmaTxRes, I2CError, I2CIntRes, I2C,
};
use futures::future::Either;
use futures::prelude::*;

/// I2C master session error.
#[derive(Debug, Fail)]
pub enum I2CMasterError {
  /// DMA error.
  #[fail(display = "DMA error: {}", _0)]
  Dma(DmaTransferError),
  /// I2C transfer failure.
  #[fail(display = "I2C failure: {}", _0)]
  I2CBreak(I2CBreak),
  /// I2C error.
  #[fail(display = "I2C error: {}", _0)]
  I2CError(I2CError),
}

/// I2C master session driver.
#[derive(Driver)]
pub struct I2CMaster<I2CRes, DmaRxRes, DmaTxRes>(
  I2CMasterRes<I2CRes, DmaRxRes, DmaTxRes>,
)
where
  I2CRes: I2CIntRes + I2CDmaRxRes<DmaRxRes> + I2CDmaTxRes<DmaTxRes>,
  DmaRxRes: DmaRes,
  DmaTxRes: DmaRes;

/// I2C master session resource.
#[allow(missing_docs)]
#[derive(Resource)]
pub struct I2CMasterRes<I2CRes, DmaRxRes, DmaTxRes>
where
  I2CRes: I2CIntRes + I2CDmaRxRes<DmaRxRes> + I2CDmaTxRes<DmaTxRes>,
  DmaRxRes: DmaRes,
  DmaTxRes: DmaRes,
{
  pub i2c: I2C<I2CRes>,
  pub dma_rx: Dma<DmaRxRes>,
  pub dma_tx: Dma<DmaTxRes>,
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  pub dmamux_rx: DmamuxCh<DmaRxRes::DmamuxChRes>,
  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  pub dmamux_tx: DmamuxCh<DmaTxRes::DmamuxChRes>,
}

#[allow(missing_docs)]
impl<I2CRes, DmaRxRes, DmaTxRes> I2CMaster<I2CRes, DmaRxRes, DmaTxRes>
where
  I2CRes: I2CIntRes + I2CDmaRxRes<DmaRxRes> + I2CDmaTxRes<DmaTxRes>,
  DmaRxRes: DmaRes,
  DmaTxRes: DmaRes,
{
  #[inline(always)]
  pub fn i2c(&self) -> &I2C<I2CRes> {
    &self.0.i2c
  }

  #[inline(always)]
  pub fn dma_rx(&self) -> &Dma<DmaRxRes> {
    &self.0.dma_rx
  }

  #[inline(always)]
  pub fn dma_tx(&self) -> &Dma<DmaTxRes> {
    &self.0.dma_tx
  }

  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  #[inline(always)]
  pub fn dmamux_rx(&self) -> &DmamuxCh<DmaRxRes::DmamuxChRes> {
    &self.0.dmamux_rx
  }

  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  pub fn dmamux_tx(&self) -> &DmamuxCh<DmaTxRes::DmamuxChRes> {
    &self.0.dmamux_tx
  }

  #[cfg(any(
    feature = "stm32l4r5",
    feature = "stm32l4r7",
    feature = "stm32l4r9",
    feature = "stm32l4s5",
    feature = "stm32l4s7",
    feature = "stm32l4s9"
  ))]
  /// Initializes DMA for the I2C as peripheral.
  #[inline(always)]
  pub fn dma_init(&self) {
    self.0.i2c.dma_dx_init(
      &self.0.dma_rx,
      &self.0.dmamux_rx,
      &self.0.dma_tx,
      &self.0.dmamux_tx,
    );
  }

  #[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
  ))]
  /// Initializes DMA for the I2C as peripheral.
  #[inline(always)]
  pub fn dma_init(&self) {
    self.0.i2c.dma_dx_init(&self.0.dma_rx, &self.0.dma_tx);
  }

  /// Reads bytes to `buf` from `slave_addr`. Leaves the session open.
  ///
  /// # Panics
  ///
  /// If length of `buf` is greater than 255.
  pub fn read<'sess>(
    &'sess mut self,
    buf: &'sess mut [u8],
    slave_addr: u8,
    i2c_cr1_val: I2CRes::Cr1Val,
    i2c_cr2_val: I2CRes::Cr2Val,
  ) -> impl Future<Item = (), Error = I2CMasterError> + 'sess {
    self.read_impl(buf, slave_addr, i2c_cr1_val, i2c_cr2_val, false)
  }

  /// Reads bytes to `buf` from `slave_addr`. Closes the session afterwards.
  ///
  /// # Panics
  ///
  /// If length of `buf` is greater than 255.
  pub fn read_and_stop<'sess>(
    &'sess mut self,
    buf: &'sess mut [u8],
    slave_addr: u8,
    i2c_cr1_val: I2CRes::Cr1Val,
    i2c_cr2_val: I2CRes::Cr2Val,
  ) -> impl Future<Item = (), Error = I2CMasterError> + 'sess {
    self.read_impl(buf, slave_addr, i2c_cr1_val, i2c_cr2_val, true)
  }

  /// Writes bytes from `buf` to `slave_addr`. Leaves the session open.
  ///
  /// # Panics
  ///
  /// If length of `buf` is greater than 255.
  pub fn write<'sess>(
    &'sess mut self,
    buf: &'sess [u8],
    slave_addr: u8,
    i2c_cr1_val: I2CRes::Cr1Val,
    i2c_cr2_val: I2CRes::Cr2Val,
  ) -> impl Future<Item = (), Error = I2CMasterError> + 'sess {
    self.write_impl(buf, slave_addr, i2c_cr1_val, i2c_cr2_val, false)
  }

  /// Writes bytes from `buf` to `slave_addr`. Closes the session afterwards.
  ///
  /// # Panics
  ///
  /// If length of `buf` is greater than 255.
  pub fn write_and_stop<'sess>(
    &'sess mut self,
    buf: &'sess [u8],
    slave_addr: u8,
    i2c_cr1_val: I2CRes::Cr1Val,
    i2c_cr2_val: I2CRes::Cr2Val,
  ) -> impl Future<Item = (), Error = I2CMasterError> + 'sess {
    self.write_impl(buf, slave_addr, i2c_cr1_val, i2c_cr2_val, true)
  }

  fn read_impl<'sess>(
    &'sess mut self,
    buf: &'sess mut [u8],
    slave_addr: u8,
    mut i2c_cr1_val: I2CRes::Cr1Val,
    mut i2c_cr2_val: I2CRes::Cr2Val,
    autoend: bool,
  ) -> impl Future<Item = (), Error = I2CMasterError> + 'sess {
    if buf.len() > 255 {
      panic!("I2C session overflow");
    }
    async(static move || {
      unsafe { self.0.dma_rx.set_maddr(buf.as_mut_ptr() as usize) };
      self.0.dma_rx.set_size(buf.len());
      self.0.dma_rx.ccr().store_val({
        let mut rx_ccr = self.init_dma_rx_ccr();
        self.0.dma_rx.ccr_en().set(&mut rx_ccr);
        rx_ccr
      });
      self.0.i2c.cr1().store_val({
        self.0.i2c.cr1_pe().set(&mut i2c_cr1_val);
        self.0.i2c.cr1_errie().set(&mut i2c_cr1_val);
        self.0.i2c.cr1_nackie().set(&mut i2c_cr1_val);
        self.0.i2c.cr1_rxdmaen().set(&mut i2c_cr1_val);
        i2c_cr1_val
      });
      let dma_rx = self.0.dma_rx.transfer_complete();
      let i2c_break = self.0.i2c.transfer_break();
      let i2c_error = self.0.i2c.transfer_error();
      self.set_i2c_cr2(&mut i2c_cr2_val, slave_addr, autoend, buf.len(), false);
      self.0.i2c.cr2().store_val(i2c_cr2_val);
      match await!(dma_rx.select(i2c_break).select(i2c_error)) {
        Ok(Either::Left((Either::Left(((), i2c_break)), i2c_error))) => {
          drop(i2c_break);
          drop(i2c_error);
          self.0.dma_rx.ccr().store_val(self.init_dma_rx_ccr());
          self.0.i2c.int_ev().trigger();
          self.0.i2c.int_er().trigger();
          Ok(())
        }
        Err(Either::Left((Either::Left((dma_rx, i2c_break)), i2c_error))) => {
          drop(i2c_break);
          drop(i2c_error);
          self.0.dma_rx.ccr().store_val(self.init_dma_rx_ccr());
          self.0.i2c.int_ev().trigger();
          self.0.i2c.int_er().trigger();
          Err(dma_rx.into())
        }
        Err(Either::Left((Either::Right((i2c_break, dma_rx)), i2c_error))) => {
          drop(dma_rx);
          drop(i2c_error);
          self.0.dma_rx.ccr().store_val(self.init_dma_rx_ccr());
          self.0.dma_rx.int().trigger();
          self.0.i2c.int_er().trigger();
          Err(i2c_break.into())
        }
        Err(Either::Right((i2c_error, rest))) => {
          drop(rest);
          self.0.dma_rx.ccr().store_val(self.init_dma_rx_ccr());
          self.0.dma_rx.int().trigger();
          self.0.i2c.int_ev().trigger();
          Err(i2c_error.into())
        }
      }
    })
  }

  fn write_impl<'sess>(
    &'sess mut self,
    buf: &'sess [u8],
    slave_addr: u8,
    mut i2c_cr1_val: I2CRes::Cr1Val,
    mut i2c_cr2_val: I2CRes::Cr2Val,
    autoend: bool,
  ) -> impl Future<Item = (), Error = I2CMasterError> + 'sess {
    if buf.len() > 255 {
      panic!("I2C session overflow");
    }
    async(static move || {
      unsafe { self.0.dma_tx.set_maddr(buf.as_ptr() as usize) };
      self.0.dma_tx.set_size(buf.len());
      self.0.dma_tx.ccr().store_val({
        let mut tx_ccr = self.init_dma_tx_ccr();
        self.0.dma_tx.ccr_en().set(&mut tx_ccr);
        tx_ccr
      });
      self.0.i2c.cr1().store_val({
        self.0.i2c.cr1_pe().set(&mut i2c_cr1_val);
        self.0.i2c.cr1_errie().set(&mut i2c_cr1_val);
        self.0.i2c.cr1_nackie().set(&mut i2c_cr1_val);
        self.0.i2c.cr1_txdmaen().set(&mut i2c_cr1_val);
        i2c_cr1_val
      });
      let dma_tx = self.0.dma_tx.transfer_complete();
      let i2c_break = self.0.i2c.transfer_break();
      let i2c_error = self.0.i2c.transfer_error();
      self.set_i2c_cr2(&mut i2c_cr2_val, slave_addr, autoend, buf.len(), true);
      self.0.i2c.cr2().store_val(i2c_cr2_val);
      match await!(dma_tx.select(i2c_break).select(i2c_error)) {
        Ok(Either::Left((Either::Left(((), i2c_break)), i2c_error))) => {
          drop(i2c_break);
          drop(i2c_error);
          self.0.dma_tx.ccr().store_val(self.init_dma_tx_ccr());
          self.0.i2c.int_ev().trigger();
          self.0.i2c.int_er().trigger();
          Ok(())
        }
        Err(Either::Left((Either::Left((dma_tx, i2c_break)), i2c_error))) => {
          drop(i2c_break);
          drop(i2c_error);
          self.0.dma_tx.ccr().store_val(self.init_dma_tx_ccr());
          self.0.i2c.int_ev().trigger();
          self.0.i2c.int_er().trigger();
          Err(dma_tx.into())
        }
        Err(Either::Left((Either::Right((i2c_break, dma_tx)), i2c_error))) => {
          drop(dma_tx);
          drop(i2c_error);
          self.0.dma_tx.ccr().store_val(self.init_dma_tx_ccr());
          self.0.dma_tx.int().trigger();
          self.0.i2c.int_er().trigger();
          Err(i2c_break.into())
        }
        Err(Either::Right((i2c_error, rest))) => {
          drop(rest);
          self.0.dma_tx.ccr().store_val(self.init_dma_tx_ccr());
          self.0.dma_tx.int().trigger();
          self.0.i2c.int_ev().trigger();
          Err(i2c_error.into())
        }
      }
    })
  }

  fn set_i2c_cr2(
    &self,
    val: &mut I2CRes::Cr2Val,
    slave_addr: u8,
    autoend: bool,
    nbytes: usize,
    write: bool,
  ) {
    self.0.i2c.cr2_add10().clear(val);
    let slave_addr = u32::from(slave_addr << 1);
    self.0.i2c.cr2_sadd().write(val, slave_addr);
    if write {
      self.0.i2c.cr2_rd_wrn().clear(val);
    } else {
      self.0.i2c.cr2_rd_wrn().set(val);
    }
    self.0.i2c.cr2_nbytes().write(val, nbytes as u32);
    if autoend {
      self.0.i2c.cr2_autoend().set(val);
    } else {
      self.0.i2c.cr2_autoend().clear(val);
    }
    self.0.i2c.cr2_start().set(val);
  }

  fn init_dma_rx_ccr(&self) -> DmaRxRes::CcrVal {
    let mut val = self.0.dma_rx.ccr().default_val();
    self.0.dma_rx.ccr_mem2mem().clear(&mut val);
    self.0.dma_rx.ccr_msize().write(&mut val, 0b00);
    self.0.dma_rx.ccr_psize().write(&mut val, 0b00);
    self.0.dma_rx.ccr_minc().set(&mut val);
    self.0.dma_rx.ccr_pinc().clear(&mut val);
    self.0.dma_rx.ccr_circ().clear(&mut val);
    self.0.dma_rx.ccr_dir().clear(&mut val);
    self.0.dma_rx.ccr_teie().set(&mut val);
    self.0.dma_rx.ccr_htie().clear(&mut val);
    self.0.dma_rx.ccr_tcie().set(&mut val);
    self.0.dma_rx.ccr_en().clear(&mut val);
    val
  }

  fn init_dma_tx_ccr(&self) -> DmaTxRes::CcrVal {
    let mut val = self.0.dma_tx.ccr().default_val();
    self.0.dma_tx.ccr_mem2mem().clear(&mut val);
    self.0.dma_tx.ccr_msize().write(&mut val, 0b00);
    self.0.dma_tx.ccr_psize().write(&mut val, 0b00);
    self.0.dma_tx.ccr_minc().set(&mut val);
    self.0.dma_tx.ccr_pinc().clear(&mut val);
    self.0.dma_tx.ccr_circ().clear(&mut val);
    self.0.dma_tx.ccr_dir().set(&mut val);
    self.0.dma_tx.ccr_teie().set(&mut val);
    self.0.dma_tx.ccr_htie().clear(&mut val);
    self.0.dma_tx.ccr_tcie().set(&mut val);
    self.0.dma_tx.ccr_en().clear(&mut val);
    val
  }
}

impl From<DmaTransferError> for I2CMasterError {
  fn from(err: DmaTransferError) -> Self {
    I2CMasterError::Dma(err)
  }
}

impl From<I2CBreak> for I2CMasterError {
  fn from(err: I2CBreak) -> Self {
    I2CMasterError::I2CBreak(err)
  }
}

impl From<I2CError> for I2CMasterError {
  fn from(err: I2CError) -> Self {
    I2CMasterError::I2CError(err)
  }
}
