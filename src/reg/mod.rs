//! Memory-mapped registers.

pub mod prelude;

mod bindings;

pub use self::bindings::*;
pub use drone::reg::bind;

use core::mem::size_of;
use core::ptr::{read_volatile, write_volatile};
use drone::reg::{RegHoldVal, RegHoldValRaw};
use drone::reg::prelude::*;

/// Peripheral bit-band alias start.
pub const BIT_BAND_BASE: usize = 0x4200_0000;

/// Peripheral bit-band region length.
pub const BIT_BAND_LENGTH: usize = 5;

/// Register that can read and write its value in a multi-threaded context.
pub trait URegShared<'a, T>
where
  Self: RReg<'a, T> + WReg<'a, T>,
  T: RegShared + 'a,
{
  /// Atomically updates a register's value.
  fn update<F>(&'a self, f: F)
  where
    F: Fn(&mut Self::Hold) -> &mut Self::Hold;
}

/// Register that falls into peripheral bit-band region.
pub trait RegBitBand<'a, T>
where
  Self: Reg<'a, T>,
  T: RegFlavor + 'a,
{
  /// Calculates bit-band address.
  ///
  /// # Panics
  ///
  /// If `offset` is greater than or equals to the platform's word size in bits.
  #[inline]
  fn bit_band_addr(offset: usize) -> usize {
    assert!(offset < size_of::<RegHoldValRaw<'a, T, Self>>() * 8);
    BIT_BAND_BASE
      + (((Self::ADDRESS + (offset >> 3))
        & ((0b1 << (BIT_BAND_LENGTH << 2)) - 1)) << BIT_BAND_LENGTH)
      + ((offset & (8 - 1)) << 2)
  }
}

/// Register field that can read bits through peripheral bit-band region.
pub trait RRegFieldBitBand<'a, T>
where
  Self: RegFieldBit<'a, T>,
  Self::Reg: RegBitBand<'a, T> + RReg<'a, T>,
  T: RegFlavor + 'a,
{
  /// Reads the state of the bit through peripheral bit-band region.
  fn read_bit(&self) -> bool;

  /// Returns an unsafe constant pointer to the corresponding bit-band address.
  fn bit_band_ptr(&self) -> *const usize;
}

/// Register field that can write bits through peripheral bit-band region.
pub trait WRegFieldBitBand<'a, T>
where
  Self: RegFieldBit<'a, T>,
  Self::Reg: RegBitBand<'a, T> + WReg<'a, T>,
  T: RegFlavor + 'a,
{
  /// Sets the bit through peripheral bit-band region.
  fn set_bit(&self);

  /// Clears the bit through peripheral bit-band region.
  fn clear_bit(&self);

  /// Returns an unsafe mutable pointer to the corresponding bit-band address.
  fn bit_band_mut_ptr(&self) -> *mut usize;
}

impl<'a, T, U, V, W> URegShared<'a, T> for U
where
  T: RegShared + 'a,
  U: RReg<'a, T, Hold = V> + WReg<'a, T, Hold = V>,
  V: RegHold<'a, T, Self, Val = W>,
  W: RegVal<Raw = u32>,
  W: From<V>,
{
  #[inline]
  fn update<F>(&'a self, f: F)
  where
    F: Fn(&mut Self::Hold) -> &mut Self::Hold,
  {
    let mut raw: u32;
    let mut status: u32;
    unsafe {
      loop {
        asm!("
          ldrex $0, [$1]
        " : "=r"(raw)
          : "r"(Self::ADDRESS)
          :
          : "volatile");
        let val = RegHoldVal::<'a, T, Self>::from_raw(raw);
        raw = f(&mut self.hold(val)).val().raw();
        asm!("
          strex $0, $1, [$2]
        " : "=r"(status)
          : "r"(raw), "r"(Self::ADDRESS)
          :
          : "volatile");
        if status == 0 {
          break;
        }
      }
    }
  }
}

impl<'a, T, U> RRegFieldBitBand<'a, T> for U
where
  T: RegFlavor + 'a,
  U: RegFieldBit<'a, T>,
  U::Reg: RegBitBand<'a, T> + RReg<'a, T>,
{
  #[inline]
  fn read_bit(&self) -> bool {
    unsafe { read_volatile(self.bit_band_ptr()) != 0 }
  }

  #[inline]
  fn bit_band_ptr(&self) -> *const usize {
    Self::Reg::bit_band_addr(Self::OFFSET) as *const usize
  }
}

impl<'a, T, U> WRegFieldBitBand<'a, T> for U
where
  T: RegFlavor + 'a,
  U: RegFieldBit<'a, T>,
  U::Reg: RegBitBand<'a, T> + WReg<'a, T>,
{
  #[inline]
  fn set_bit(&self) {
    unsafe { write_volatile(self.bit_band_mut_ptr(), 1) };
  }

  #[inline]
  fn clear_bit(&self) {
    unsafe { write_volatile(self.bit_band_mut_ptr(), 0) };
  }

  #[inline]
  fn bit_band_mut_ptr(&self) -> *mut usize {
    Self::Reg::bit_band_addr(Self::OFFSET) as *mut usize
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use drone::reg;

  reg! {
    #![allow(dead_code)]
    LOW_REG 0x4000_0000 0x20 0x0000_0000 RegBitBand
  }

  reg! {
    #![allow(dead_code)]
    HIGH_REG 0x400F_FFFC 0x20 0x0000_0000 RegBitBand
  }

  type LocalLowReg = LowReg<Ur>;
  type LocalHighReg = HighReg<Ur>;

  #[test]
  fn reg_bit_band_addr() {
    assert_eq!(LocalLowReg::bit_band_addr(0), 0x4200_0000);
    assert_eq!(LocalLowReg::bit_band_addr(7), 0x4200_001C);
    assert_eq!(LocalLowReg::bit_band_addr(31), 0x4200_007C);
    assert_eq!(LocalHighReg::bit_band_addr(24), 0x43FF_FFE0);
    assert_eq!(LocalHighReg::bit_band_addr(31), 0x43FF_FFFC);
  }
}
