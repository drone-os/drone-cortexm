//! Safe API for memory-mapped registers.

pub mod stk;

/// Memory-mapped registers prelude.
pub mod prelude {
  pub use super::{RRegBitBand, RegBitBand, RwAtomicReg, WRegBitBand};
  pub use drone::reg::prelude::*;
}

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
  Self: RReg<Atomic> + WReg<Atomic>,
{
  /// Atomically modifies a register's value.
  unsafe fn modify<F>(&mut self, f: F)
  where
    F: Fn(&mut Self::Value) -> &Self::Value;
}

/// Register that falls into peripheral bit-band region.
pub trait RegBitBand<T>
where
  Self: Reg<T>,
  T: Flavor,
{
  /// Calculates bit-band address.
  ///
  /// # Panics
  ///
  /// If `offset` is greater than or equals to the platform's word size in bits.
  fn bit_band_addr(offset: usize) -> usize {
    assert!(offset < size_of::<usize>() * 8);
    BIT_BAND_BASE +
      (((Self::ADDRESS + (offset >> 3)) &
        ((0b1 << (BIT_BAND_LENGTH << 2)) - 1)) << BIT_BAND_LENGTH) +
      ((offset & (8 - 1)) << 2)
  }
}

/// Register that can read bits through peripheral bit-band region.
pub trait RRegBitBand<T>
where
  Self: RegBitBand<T> + RReg<T>,
  T: Flavor,
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
  T: Flavor,
{
  /// Atomically sets or clears the register's bit by `offset` through
  /// peripheral bit-band region.
  ///
  /// # Panics
  ///
  /// If `offset` is greater than or equals to the platform's word size in bits.
  fn set_bit_band(&mut self, offset: usize, value: bool);

  /// Returns an unsafe mutable pointer to the corresponding bit-band address.
  ///
  /// # Panics
  ///
  /// If `offset` is greater than or equals to the platform's word size in bits.
  fn bit_band_mut_ptr(&mut self, offset: usize) -> *mut usize;
}

impl<T> RwAtomicReg for T
where
  T: RReg<Atomic> + WReg<Atomic>,
{
  unsafe fn modify<F>(&mut self, f: F)
  where
    F: Fn(&mut Self::Value) -> &Self::Value,
  {
    let mut value: usize;
    let mut status: usize;
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
  U: Flavor,
{
  fn bit_band(&self, offset: usize) -> bool {
    unsafe { read_volatile(self.bit_band_ptr(offset)) != 0 }
  }

  fn bit_band_ptr(&self, offset: usize) -> *const usize {
    Self::bit_band_addr(offset) as *const usize
  }
}

impl<T, U> WRegBitBand<U> for T
where
  T: RegBitBand<U> + WReg<U>,
  U: Flavor,
{
  fn set_bit_band(&mut self, offset: usize, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
      write_volatile(self.bit_band_mut_ptr(offset), value);
    }
  }

  fn bit_band_mut_ptr(&mut self, offset: usize) -> *mut usize {
    Self::bit_band_addr(offset) as *mut usize
  }
}

include!(concat!(env!("OUT_DIR"), "/svd.rs"));

#[cfg(test)]
mod tests {
  use super::*;

  reg!([0x4000_0000] LowReg LowRegValue RegBitBand {});
  reg!([0x400F_FFFC] HighReg HighRegValue RegBitBand {});

  type LocalLowReg = LowReg<Local>;
  type LocalHighReg = HighReg<Local>;

  #[test]
  fn reg_bit_band_addr() {
    assert_eq!(LocalLowReg::bit_band_addr(0), 0x4200_0000);
    assert_eq!(LocalLowReg::bit_band_addr(7), 0x4200_001C);
    assert_eq!(LocalLowReg::bit_band_addr(31), 0x4200_007C);
    assert_eq!(LocalHighReg::bit_band_addr(24), 0x43FF_FFE0);
    assert_eq!(LocalHighReg::bit_band_addr(31), 0x43FF_FFFC);
  }
}
