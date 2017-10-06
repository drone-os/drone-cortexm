//! Memory-mapped registers support.

pub mod prelude;
pub mod scb;
pub mod stk;

pub use self::stk::Ctrl as StkCtrl;
pub use self::stk::Load as StkLoad;

use core::mem::size_of;
use core::ptr::{read_volatile, write_volatile};
use drone::reg::prelude::*;

/// Peripheral bit-band alias start.
pub const BIT_BAND_BASE: usize = 0x4200_0000;

/// Peripheral bit-band region length.
pub const BIT_BAND_LENGTH: usize = 5;

/// Register that can read and write its value in a multi-threaded context.
pub trait RwAtomicReg
where
  Self: RReg<Ar> + WReg<Ar>,
{
  /// Atomically modifies a register's value.
  unsafe fn modify<F>(&self, f: F)
  where
    F: Fn(&mut Self::Value) -> &Self::Value;
}

/// Register that falls into peripheral bit-band region.
pub trait RegBitBand<T>
where
  Self: Reg<T>,
  T: RegFlavor,
{
  /// Calculates bit-band address.
  ///
  /// # Panics
  ///
  /// If `offset` is greater than or equals to the platform's word size in bits.
  #[inline]
  fn bit_band_addr(offset: usize) -> usize {
    assert!(offset < size_of::<<Self::Value as RegValue>::Raw>() * 8);
    BIT_BAND_BASE
      + (((Self::ADDRESS + (offset >> 3))
        & ((0b1 << (BIT_BAND_LENGTH << 2)) - 1)) << BIT_BAND_LENGTH)
      + ((offset & (8 - 1)) << 2)
  }
}

/// Register that can read bits through peripheral bit-band region.
pub trait RRegBitBand<T>
where
  Self: RegBitBand<T> + RReg<T>,
  T: RegFlavor,
{
  /// Reads the register's bit by `offset` through peripheral bit-band region.
  ///
  /// # Panics
  ///
  /// If `offset` is greater than or equals to the platform's word size in bits.
  fn bit_band(&self, offset: usize) -> bool;

  /// Returns an unsafe constant pointer to the corresponding bit-band address.
  ///
  /// # Panics
  ///
  /// If `offset` is greater than or equals to the platform's word size in bits.
  fn bit_band_ptr(&self, offset: usize) -> *const usize;
}

/// Register that can write bits through peripheral bit-band region.
pub trait WRegBitBand<T>
where
  Self: RegBitBand<T> + WReg<T>,
  T: RegFlavor,
{
  /// Atomically sets or clears the register's bit by `offset` through
  /// peripheral bit-band region.
  ///
  /// # Panics
  ///
  /// If `offset` is greater than or equals to the platform's word size in bits.
  fn set_bit_band(&self, offset: usize, value: bool);

  /// Returns an unsafe mutable pointer to the corresponding bit-band address.
  ///
  /// # Panics
  ///
  /// If `offset` is greater than or equals to the platform's word size in bits.
  fn bit_band_mut_ptr(&self, offset: usize) -> *mut usize;
}

impl<T, U> RwAtomicReg for T
where
  T: RReg<Ar, Value = U> + WReg<Ar, Value = U>,
  U: RegValue<Raw = u32>,
{
  #[inline]
  unsafe fn modify<F>(&self, f: F)
  where
    F: Fn(&mut Self::Value) -> &Self::Value,
  {
    let mut value: u32;
    let mut status: u32;
    loop {
      asm!("
        ldrex $0, [$1]
      " : "=r"(value)
        : "r"(Self::ADDRESS)
        :
        : "volatile");
      value = f(&mut Self::Value::new(value)).raw();
      asm!("
        strex $0, $1, [$2]
      " : "=r"(status)
        : "r"(value), "r"(Self::ADDRESS)
        :
        : "volatile");
      if status == 0 {
        break;
      }
    }
  }
}

impl<T, U> RRegBitBand<U> for T
where
  T: RegBitBand<U> + RReg<U>,
  U: RegFlavor,
{
  #[inline]
  fn bit_band(&self, offset: usize) -> bool {
    unsafe { read_volatile(self.bit_band_ptr(offset)) != 0 }
  }

  #[inline]
  fn bit_band_ptr(&self, offset: usize) -> *const usize {
    Self::bit_band_addr(offset) as *const usize
  }
}

impl<T, U> WRegBitBand<U> for T
where
  T: RegBitBand<U> + WReg<U>,
  U: RegFlavor,
{
  #[inline]
  fn set_bit_band(&self, offset: usize, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
      write_volatile(self.bit_band_mut_ptr(offset), value);
    }
  }

  #[inline]
  fn bit_band_mut_ptr(&self, offset: usize) -> *mut usize {
    Self::bit_band_addr(offset) as *mut usize
  }
}

include!(concat!(env!("OUT_DIR"), "/svd.rs"));

#[cfg(test)]
mod tests {
  use super::*;
  use drone::reg;

  reg!(0x4000_0000 0x20 LowReg RegBitBand);
  reg!(0x400F_FFFC 0x20 HighReg RegBitBand);

  type LocalLowReg = LowReg<Lr>;
  type LocalHighReg = HighReg<Lr>;

  #[test]
  fn reg_bit_band_addr() {
    assert_eq!(LocalLowReg::bit_band_addr(0), 0x4200_0000);
    assert_eq!(LocalLowReg::bit_band_addr(7), 0x4200_001C);
    assert_eq!(LocalLowReg::bit_band_addr(31), 0x4200_007C);
    assert_eq!(LocalHighReg::bit_band_addr(24), 0x43FF_FFE0);
    assert_eq!(LocalHighReg::bit_band_addr(31), 0x43FF_FFFC);
  }
}
