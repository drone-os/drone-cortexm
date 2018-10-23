//! I2C DMA master session.

use drone_core::drv::Resource;
use drv::dma::{
  Dma, Dma1Ch2Res, Dma1Ch3Res, Dma1Ch4Res, Dma1Ch5Res, Dma1Ch6Res, Dma1Ch7Res,
  Dma2Ch6Res, Dma2Ch7Res, DmaRes, DmaTransferError,
};
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drv::dma::{Dma2Ch1Res, Dma2Ch2Res};
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use drv::i2c::I2C4IntRes;
use drv::i2c::{
  I2C1IntRes, I2C2IntRes, I2C3IntRes, I2CBreak, I2CDmaDx, I2CDmaRxRes,
  I2CDmaTxRes, I2CError, I2CIntRes, I2CRes, I2C,
};
use futures::future::Either;
use futures::prelude::*;
use reg::prelude::*;
#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
use thr::int::{
  IntDma1Ch2, IntDma1Ch3, IntDma1Ch4, IntDma1Ch5, IntDma1Ch6, IntDma1Ch7,
  IntDma2Ch1, IntDma2Ch2, IntDma2Ch6, IntDma2Ch7, IntI2C4Er, IntI2C4Ev,
};
#[cfg(any(feature = "stm32l4x3", feature = "stm32l4x5"))]
use thr::int::{
  IntDma1Channel2 as IntDma1Ch2, IntDma1Channel3 as IntDma1Ch3,
  IntDma1Channel4 as IntDma1Ch4, IntDma1Channel5 as IntDma1Ch5,
  IntDma1Channel6 as IntDma1Ch6, IntDma1Channel7 as IntDma1Ch7,
  IntDma2Channel6 as IntDma2Ch6, IntDma2Channel7 as IntDma2Ch7,
};
use thr::int::{
  IntI2C1Er, IntI2C1Ev, IntI2C2Er, IntI2C2Ev, IntI2C3Er, IntI2C3Ev,
};
use thr::prelude::*;

/// I2C DMA master session error.
#[derive(Debug, Fail)]
pub enum I2CDmaMasterSessError {
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

/// I2C DMA master session driver.
#[derive(Driver)]
pub struct I2CDmaMasterSess<T: I2CDmaMasterSessRes>(T);

/// I2C DMA master session resource.
#[allow(missing_docs)]
pub trait I2CDmaMasterSessRes: Resource {
  type I2CRes: I2CDmaRxRes<Self::DmaRxRes>
    + I2CDmaTxRes<Self::DmaTxRes>
    + I2CIntRes;
  type DmaRxRes: DmaRes;
  type DmaTxRes: DmaRes<Cselr = <Self::DmaRxRes as DmaRes>::Cselr>;

  fn i2c(&self) -> &I2C<Self::I2CRes>;
  fn i2c_mut(&mut self) -> &mut I2C<Self::I2CRes>;
  fn dma_rx(&self) -> &Dma<Self::DmaRxRes>;
  fn dma_rx_mut(&mut self) -> &mut Dma<Self::DmaRxRes>;
  fn dma_tx(&self) -> &Dma<Self::DmaTxRes>;
  fn dma_tx_mut(&mut self) -> &mut Dma<Self::DmaTxRes>;
}

#[allow(missing_docs)]
impl<T: I2CDmaMasterSessRes> I2CDmaMasterSess<T> {
  #[inline(always)]
  pub fn i2c(&self) -> &I2C<T::I2CRes> {
    self.0.i2c()
  }

  #[inline(always)]
  pub fn dma_tx(&self) -> &Dma<T::DmaTxRes> {
    self.0.dma_tx()
  }

  #[inline(always)]
  pub fn dma_rx(&self) -> &Dma<T::DmaRxRes> {
    self.0.dma_rx()
  }

  /// Initializes DMA for the I2C as peripheral.
  #[inline(always)]
  pub fn dma_init(&self) {
    self.0.i2c().dma_dx_init(self.0.dma_rx(), self.0.dma_tx());
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
    i2c_cr1_val: <T::I2CRes as I2CRes>::Cr1Val,
    i2c_cr2_val: <T::I2CRes as I2CRes>::Cr2Val,
  ) -> impl Future<Item = (), Error = I2CDmaMasterSessError> + 'sess {
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
    i2c_cr1_val: <T::I2CRes as I2CRes>::Cr1Val,
    i2c_cr2_val: <T::I2CRes as I2CRes>::Cr2Val,
  ) -> impl Future<Item = (), Error = I2CDmaMasterSessError> + 'sess {
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
    i2c_cr1_val: <T::I2CRes as I2CRes>::Cr1Val,
    i2c_cr2_val: <T::I2CRes as I2CRes>::Cr2Val,
  ) -> impl Future<Item = (), Error = I2CDmaMasterSessError> + 'sess {
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
    i2c_cr1_val: <T::I2CRes as I2CRes>::Cr1Val,
    i2c_cr2_val: <T::I2CRes as I2CRes>::Cr2Val,
  ) -> impl Future<Item = (), Error = I2CDmaMasterSessError> + 'sess {
    self.write_impl(buf, slave_addr, i2c_cr1_val, i2c_cr2_val, true)
  }

  fn read_impl<'sess>(
    &'sess mut self,
    buf: &'sess mut [u8],
    slave_addr: u8,
    mut i2c_cr1_val: <T::I2CRes as I2CRes>::Cr1Val,
    mut i2c_cr2_val: <T::I2CRes as I2CRes>::Cr2Val,
    autoend: bool,
  ) -> impl Future<Item = (), Error = I2CDmaMasterSessError> + 'sess {
    if buf.len() > 255 {
      panic!("I2C session overflow");
    }
    async(static move || {
      unsafe { self.0.dma_rx().set_maddr(buf.as_mut_ptr() as usize) };
      self.0.dma_rx().set_size(buf.len());
      self.0.dma_rx().ccr().store_val({
        let mut rx_ccr = self.init_dma_rx_ccr();
        self.0.dma_rx().ccr_en().set(&mut rx_ccr);
        rx_ccr
      });
      self.0.i2c().cr1().store_val({
        self.0.i2c().cr1_pe().set(&mut i2c_cr1_val);
        self.0.i2c().cr1_errie().set(&mut i2c_cr1_val);
        self.0.i2c().cr1_nackie().set(&mut i2c_cr1_val);
        self.0.i2c().cr1_rxdmaen().set(&mut i2c_cr1_val);
        i2c_cr1_val
      });
      let dma_rx = self.0.dma_rx_mut().transfer_complete();
      let i2c_break = self.0.i2c_mut().transfer_break();
      let i2c_error = self.0.i2c_mut().transfer_error();
      self.set_i2c_cr2(&mut i2c_cr2_val, slave_addr, autoend, buf.len(), false);
      self.0.i2c().cr2().store_val(i2c_cr2_val);
      match await!(dma_rx.select(i2c_break).select(i2c_error)) {
        Ok(Either::Left((Either::Left(((), i2c_break)), i2c_error))) => {
          drop(i2c_break);
          drop(i2c_error);
          self.0.dma_rx().ccr().store_val(self.init_dma_rx_ccr());
          self.0.i2c().int_ev().trigger();
          self.0.i2c().int_er().trigger();
          Ok(())
        }
        Err(Either::Left((Either::Left((dma_rx, i2c_break)), i2c_error))) => {
          drop(i2c_break);
          drop(i2c_error);
          self.0.dma_rx().ccr().store_val(self.init_dma_rx_ccr());
          self.0.i2c().int_ev().trigger();
          self.0.i2c().int_er().trigger();
          Err(dma_rx.into())
        }
        Err(Either::Left((Either::Right((i2c_break, dma_rx)), i2c_error))) => {
          drop(dma_rx);
          drop(i2c_error);
          self.0.dma_rx().ccr().store_val(self.init_dma_rx_ccr());
          self.0.dma_rx().int().trigger();
          self.0.i2c().int_er().trigger();
          Err(i2c_break.into())
        }
        Err(Either::Right((i2c_error, rest))) => {
          drop(rest);
          self.0.dma_rx().ccr().store_val(self.init_dma_rx_ccr());
          self.0.dma_rx().int().trigger();
          self.0.i2c().int_ev().trigger();
          Err(i2c_error.into())
        }
      }
    })
  }

  fn write_impl<'sess>(
    &'sess mut self,
    buf: &'sess [u8],
    slave_addr: u8,
    mut i2c_cr1_val: <T::I2CRes as I2CRes>::Cr1Val,
    mut i2c_cr2_val: <T::I2CRes as I2CRes>::Cr2Val,
    autoend: bool,
  ) -> impl Future<Item = (), Error = I2CDmaMasterSessError> + 'sess {
    if buf.len() > 255 {
      panic!("I2C session overflow");
    }
    async(static move || {
      unsafe { self.0.dma_tx().set_maddr(buf.as_ptr() as usize) };
      self.0.dma_tx().set_size(buf.len());
      self.0.dma_tx().ccr().store_val({
        let mut tx_ccr = self.init_dma_tx_ccr();
        self.0.dma_tx().ccr_en().set(&mut tx_ccr);
        tx_ccr
      });
      self.0.i2c().cr1().store_val({
        self.0.i2c().cr1_pe().set(&mut i2c_cr1_val);
        self.0.i2c().cr1_errie().set(&mut i2c_cr1_val);
        self.0.i2c().cr1_nackie().set(&mut i2c_cr1_val);
        self.0.i2c().cr1_txdmaen().set(&mut i2c_cr1_val);
        i2c_cr1_val
      });
      let dma_tx = self.0.dma_tx_mut().transfer_complete();
      let i2c_break = self.0.i2c_mut().transfer_break();
      let i2c_error = self.0.i2c_mut().transfer_error();
      self.set_i2c_cr2(&mut i2c_cr2_val, slave_addr, autoend, buf.len(), true);
      self.0.i2c().cr2().store_val(i2c_cr2_val);
      match await!(dma_tx.select(i2c_break).select(i2c_error)) {
        Ok(Either::Left((Either::Left(((), i2c_break)), i2c_error))) => {
          drop(i2c_break);
          drop(i2c_error);
          self.0.dma_tx().ccr().store_val(self.init_dma_tx_ccr());
          self.0.i2c().int_ev().trigger();
          self.0.i2c().int_er().trigger();
          Ok(())
        }
        Err(Either::Left((Either::Left((dma_tx, i2c_break)), i2c_error))) => {
          drop(i2c_break);
          drop(i2c_error);
          self.0.dma_tx().ccr().store_val(self.init_dma_tx_ccr());
          self.0.i2c().int_ev().trigger();
          self.0.i2c().int_er().trigger();
          Err(dma_tx.into())
        }
        Err(Either::Left((Either::Right((i2c_break, dma_tx)), i2c_error))) => {
          drop(dma_tx);
          drop(i2c_error);
          self.0.dma_tx().ccr().store_val(self.init_dma_tx_ccr());
          self.0.dma_tx().int().trigger();
          self.0.i2c().int_er().trigger();
          Err(i2c_break.into())
        }
        Err(Either::Right((i2c_error, rest))) => {
          drop(rest);
          self.0.dma_tx().ccr().store_val(self.init_dma_tx_ccr());
          self.0.dma_tx().int().trigger();
          self.0.i2c().int_ev().trigger();
          Err(i2c_error.into())
        }
      }
    })
  }

  fn set_i2c_cr2(
    &self,
    val: &mut <T::I2CRes as I2CRes>::Cr2Val,
    slave_addr: u8,
    autoend: bool,
    nbytes: usize,
    write: bool,
  ) {
    self.0.i2c().cr2_add10().clear(val);
    let slave_addr = u32::from(slave_addr << 1);
    self.0.i2c().cr2_sadd().write(val, slave_addr);
    if write {
      self.0.i2c().cr2_rd_wrn().clear(val);
    } else {
      self.0.i2c().cr2_rd_wrn().set(val);
    }
    self.0.i2c().cr2_nbytes().write(val, nbytes as u32);
    if autoend {
      self.0.i2c().cr2_autoend().set(val);
    } else {
      self.0.i2c().cr2_autoend().clear(val);
    }
    self.0.i2c().cr2_start().set(val);
  }

  fn init_dma_rx_ccr(&self) -> <T::DmaRxRes as DmaRes>::CcrVal {
    let mut val = self.0.dma_rx().ccr().default_val();
    self.0.dma_rx().ccr_mem2mem().clear(&mut val);
    self.0.dma_rx().ccr_msize().write(&mut val, 0b00);
    self.0.dma_rx().ccr_psize().write(&mut val, 0b00);
    self.0.dma_rx().ccr_minc().set(&mut val);
    self.0.dma_rx().ccr_pinc().clear(&mut val);
    self.0.dma_rx().ccr_circ().clear(&mut val);
    self.0.dma_rx().ccr_dir().clear(&mut val);
    self.0.dma_rx().ccr_teie().set(&mut val);
    self.0.dma_rx().ccr_htie().clear(&mut val);
    self.0.dma_rx().ccr_tcie().set(&mut val);
    self.0.dma_rx().ccr_en().clear(&mut val);
    val
  }

  fn init_dma_tx_ccr(&self) -> <T::DmaTxRes as DmaRes>::CcrVal {
    let mut val = self.0.dma_tx().ccr().default_val();
    self.0.dma_tx().ccr_mem2mem().clear(&mut val);
    self.0.dma_tx().ccr_msize().write(&mut val, 0b00);
    self.0.dma_tx().ccr_psize().write(&mut val, 0b00);
    self.0.dma_tx().ccr_minc().set(&mut val);
    self.0.dma_tx().ccr_pinc().clear(&mut val);
    self.0.dma_tx().ccr_circ().clear(&mut val);
    self.0.dma_tx().ccr_dir().set(&mut val);
    self.0.dma_tx().ccr_teie().set(&mut val);
    self.0.dma_tx().ccr_htie().clear(&mut val);
    self.0.dma_tx().ccr_tcie().set(&mut val);
    self.0.dma_tx().ccr_en().clear(&mut val);
    val
  }
}

impl From<DmaTransferError> for I2CDmaMasterSessError {
  fn from(err: DmaTransferError) -> Self {
    I2CDmaMasterSessError::Dma(err)
  }
}

impl From<I2CBreak> for I2CDmaMasterSessError {
  fn from(err: I2CBreak) -> Self {
    I2CDmaMasterSessError::I2CBreak(err)
  }
}

impl From<I2CError> for I2CDmaMasterSessError {
  fn from(err: I2CError) -> Self {
    I2CDmaMasterSessError::I2CError(err)
  }
}

#[allow(unused_macros)]
macro_rules! i2c_dma_master_sess {
  (
    $doc:expr,
    $name:ident,
    $name_macro:ident,
    $doc_res:expr,
    $name_res:ident,
    $i2c:ident,
    $i2c_res:ident,
    $i2c_macro:ident,
    $i2c_int_ev_ty:ident,
    $i2c_int_er_ty:ident,
    $dma_rx:ident,
    $dma_rx_res:ident,
    $dma_rx_macro:ident,
    $dma_rx_int_ty:ident,
    $dma_tx:ident,
    $dma_tx_res:ident,
    $dma_tx_macro:ident,
    $dma_tx_int_ty:ident,
  ) => {
    #[doc = $doc]
    pub type $name<I2CEv, I2CEr, DmaRx, DmaTx> =
      I2CDmaMasterSess<$name_res<I2CEv, I2CEr, DmaRx, DmaTx>>;

    #[doc = $doc_res]
    #[allow(missing_docs)]
    #[derive(Resource)]
    pub struct $name_res<I2CEv, I2CEr, DmaRx, DmaTx>
    where
      I2CEv: $i2c_int_ev_ty<Ttt>,
      I2CEr: $i2c_int_er_ty<Ttt>,
      DmaRx: $dma_rx_int_ty<Ttt>,
      DmaTx: $dma_tx_int_ty<Ttt>,
    {
      pub $i2c: I2C<$i2c_res<I2CEv, I2CEr, Frt>>,
      pub $dma_rx: Dma<$dma_rx_res<DmaRx, Frt>>,
      pub $dma_tx: Dma<$dma_tx_res<DmaTx, Frt>>,
    }

    /// Creates a new `I2CDmaMasterSess`.
    #[macro_export]
    macro_rules! $name_macro {
      ($reg: ident, $thr: ident) => {
        $crate::drv::i2c_dma_master_sess::I2CDmaMasterSess::new(
          $crate::drv::i2c_dma_master_sess::$name_res {
            $i2c: $i2c_macro!($reg, $thr),
            $dma_rx: $dma_rx_macro!($reg, $thr),
            $dma_tx: $dma_tx_macro!($reg, $thr),
          },
        )
      };
    }

    impl<I2CEv, I2CEr, DmaRx, DmaTx> I2CDmaMasterSessRes
      for $name_res<I2CEv, I2CEr, DmaRx, DmaTx>
    where
      I2CEv: $i2c_int_ev_ty<Ttt>,
      I2CEr: $i2c_int_er_ty<Ttt>,
      DmaRx: $dma_rx_int_ty<Ttt>,
      DmaTx: $dma_tx_int_ty<Ttt>,
    {
      type I2CRes = $i2c_res<I2CEv, I2CEr, Frt>;
      type DmaRxRes = $dma_rx_res<DmaRx, Frt>;
      type DmaTxRes = $dma_tx_res<DmaTx, Frt>;

      #[inline(always)]
      fn i2c(&self) -> &I2C<Self::I2CRes> {
        &self.$i2c
      }

      #[inline(always)]
      fn i2c_mut(&mut self) -> &mut I2C<Self::I2CRes> {
        &mut self.$i2c
      }

      #[inline(always)]
      fn dma_rx(&self) -> &Dma<Self::DmaRxRes> {
        &self.$dma_rx
      }

      #[inline(always)]
      fn dma_rx_mut(&mut self) -> &mut Dma<Self::DmaRxRes> {
        &mut self.$dma_rx
      }

      #[inline(always)]
      fn dma_tx(&self) -> &Dma<Self::DmaTxRes> {
        &self.$dma_tx
      }

      #[inline(always)]
      fn dma_tx_mut(&mut self) -> &mut Dma<Self::DmaTxRes> {
        &mut self.$dma_tx
      }
    }
  };
}

i2c_dma_master_sess! {
  "I2C1 DMA1 master session driver.",
  I2C1Dma1MasterSess,
  drv_i2c1_dma1_master_sess,
  "I2C1 DMA1 master session resource.",
  I2C1Dma1MasterSessRes,
  i2c1,
  I2C1IntRes,
  drv_i2c1_int,
  IntI2C1Ev,
  IntI2C1Er,
  dma1_ch7,
  Dma1Ch7Res,
  drv_dma1_ch7,
  IntDma1Ch7,
  dma1_ch6,
  Dma1Ch6Res,
  drv_dma1_ch6,
  IntDma1Ch6,
}

i2c_dma_master_sess! {
  "I2C1 DMA2 master session driver.",
  I2C1Dma2MasterSess,
  drv_i2c1_dma2_master_sess,
  "I2C1 DMA2 master session resource.",
  I2C1Dma2MasterSessRes,
  i2c1,
  I2C1IntRes,
  drv_i2c1_int,
  IntI2C1Ev,
  IntI2C1Er,
  dma2_ch6,
  Dma2Ch6Res,
  drv_dma2_ch6,
  IntDma2Ch6,
  dma2_ch7,
  Dma2Ch7Res,
  drv_dma2_ch7,
  IntDma2Ch7,
}

i2c_dma_master_sess! {
  "I2C2 DMA1 master session driver.",
  I2C2Dma1MasterSess,
  drv_i2c2_dma1_master_sess,
  "I2C2 DMA1 master session resource.",
  I2C2Dma1MasterSessRes,
  i2c2,
  I2C2IntRes,
  drv_i2c2_int,
  IntI2C2Ev,
  IntI2C2Er,
  dma1_ch5,
  Dma1Ch5Res,
  drv_dma1_ch5,
  IntDma1Ch5,
  dma1_ch4,
  Dma1Ch4Res,
  drv_dma1_ch4,
  IntDma1Ch4,
}

i2c_dma_master_sess! {
  "I2C3 DMA1 master session driver.",
  I2C3Dma1MasterSess,
  drv_i2c3_dma1_master_sess,
  "I2C3 DMA1 master session resource.",
  I2C3Dma1MasterSessRes,
  i2c3,
  I2C3IntRes,
  drv_i2c3_int,
  IntI2C3Ev,
  IntI2C3Er,
  dma1_ch3,
  Dma1Ch3Res,
  drv_dma1_ch3,
  IntDma1Ch3,
  dma1_ch2,
  Dma1Ch2Res,
  drv_dma1_ch2,
  IntDma1Ch2,
}

#[cfg(any(
  feature = "stm32l4x1",
  feature = "stm32l4x2",
  feature = "stm32l4x6",
  feature = "stm32l4r5",
  feature = "stm32l4r7",
  feature = "stm32l4r9",
  feature = "stm32l4s5",
  feature = "stm32l4s7",
  feature = "stm32l4s9"
))]
i2c_dma_master_sess! {
  "I2C4 DMA2 master session driver.",
  I2C4Dma2MasterSess,
  drv_i2c4_dma2_master_sess,
  "I2C4 DMA2 master session resource.",
  I2C4Dma2MasterSessRes,
  i2c4,
  I2C4IntRes,
  drv_i2c4_int,
  IntI2C4Ev,
  IntI2C4Er,
  dma2_ch1,
  Dma2Ch1Res,
  drv_dma2_ch1,
  IntDma2Ch1,
  dma2_ch2,
  Dma2Ch2Res,
  drv_dma2_ch2,
  IntDma2Ch2,
}
