use core::ptr::{read_volatile, write_volatile};
use drone::reg::prelude::*;

/// Peripheral bit-band alias start.
pub const BIT_BAND_BASE: usize = 0x4200_0000;

/// Peripheral bit-band region width.
pub const BIT_BAND_WIDTH: usize = 5;

/// Register that falls into peripheral bit-band region.
pub trait RegBitBand<'a, T>
where
  Self: Reg<'a, T>,
  T: RegTag + 'a,
{
  /// Calculates bit-band address.
  ///
  /// # Safety
  ///
  /// `offset` must be greater than or equals to the platform's word size in
  /// bits.
  #[inline(always)]
  unsafe fn bit_band_addr(offset: usize) -> usize {
    BIT_BAND_BASE
      + (((Self::ADDRESS + (offset >> 3))
        & ((0b1 << (BIT_BAND_WIDTH << 2)) - 1)) << BIT_BAND_WIDTH)
      + ((offset & (8 - 1)) << 2)
  }
}

/// Register field that can read bits through peripheral bit-band region.
pub trait RRegFieldBitBand<'a, T>
where
  Self: RegFieldBit<'a, T>,
  Self::Reg: RegBitBand<'a, T> + RReg<'a, T>,
  T: RegTag + 'a,
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
  T: RegTag + 'a,
{
  /// Sets the bit through peripheral bit-band region.
  fn set_bit(&self);

  /// Clears the bit through peripheral bit-band region.
  fn clear_bit(&self);

  /// Returns an unsafe mutable pointer to the corresponding bit-band address.
  fn bit_band_mut_ptr(&self) -> *mut usize;
}

impl<'a, T, U> RRegFieldBitBand<'a, T> for U
where
  T: RegTag + 'a,
  U: RegFieldBit<'a, T>,
  U::Reg: RegBitBand<'a, T> + RReg<'a, T>,
{
  #[inline(always)]
  fn read_bit(&self) -> bool {
    unsafe { read_volatile(self.bit_band_ptr()) != 0 }
  }

  #[inline(always)]
  fn bit_band_ptr(&self) -> *const usize {
    unsafe { Self::Reg::bit_band_addr(Self::OFFSET) as *const usize }
  }
}

impl<'a, T, U> WRegFieldBitBand<'a, T> for U
where
  T: RegTag + 'a,
  U: RegFieldBit<'a, T>,
  U::Reg: RegBitBand<'a, T> + WReg<'a, T>,
{
  #[inline(always)]
  fn set_bit(&self) {
    unsafe { write_volatile(self.bit_band_mut_ptr(), 1) };
  }

  #[inline(always)]
  fn clear_bit(&self) {
    unsafe { write_volatile(self.bit_band_mut_ptr(), 0) };
  }

  #[inline(always)]
  fn bit_band_mut_ptr(&self) -> *mut usize {
    unsafe { Self::Reg::bit_band_addr(Self::OFFSET) as *mut usize }
  }
}

#[cfg(test)]
mod tests {
  use self::test_block::*;
  use super::*;
  use drone::reg;

  reg! {
    TEST_BLOCK

    #[allow(dead_code)]
    LOW_REG {
      0x4000_0000 0x20 0x0000_0000 RegBitBand
      TEST_BIT { 0 1 }
    }

    #[allow(dead_code)]
    HIGH_REG {
      0x400F_FFFC 0x20 0x0000_0000 RegBitBand
      TEST_BIT { 0 1 }
    }
  }

  type LocalLowReg = LowReg<Ur>;
  type LocalHighReg = HighReg<Ur>;

  #[test]
  fn reg_bit_band_addr() {
    unsafe {
      assert_eq!(LocalLowReg::bit_band_addr(0), 0x4200_0000);
      assert_eq!(LocalLowReg::bit_band_addr(7), 0x4200_001C);
      assert_eq!(LocalLowReg::bit_band_addr(31), 0x4200_007C);
      assert_eq!(LocalHighReg::bit_band_addr(24), 0x43FF_FFE0);
      assert_eq!(LocalHighReg::bit_band_addr(31), 0x43FF_FFFC);
    }
  }
}
