use crate::reg::{
    field::{RRRegFieldBit, RegField, WWRegFieldBit, WoWoRegFieldBit},
    tag::{RegTag, Urt},
    RReg, Reg, WReg, WoReg,
};
use core::ptr::{read_volatile, write_volatile};

/// Peripheral bit-band alias start.
pub const BIT_BAND_BASE: usize = 0x4200_0000;

/// Peripheral bit-band region width.
pub const BIT_BAND_WIDTH: usize = 5;

/// Register that falls into peripheral bit-band region.
pub trait RegBitBand<T: RegTag>: Reg<T> {
    /// Calculates bit-band address.
    ///
    /// # Safety
    ///
    /// `offset` must be greater than or equals to the platform's word size in
    /// bits.
    #[inline]
    unsafe fn bit_band_addr(offset: usize) -> usize {
        BIT_BAND_BASE
            + (((Self::ADDRESS + (offset >> 3)) & ((0b1 << (BIT_BAND_WIDTH << 2)) - 1))
                << BIT_BAND_WIDTH)
            + ((offset & (8 - 1)) << 2)
    }
}

/// Register field that can read bits through peripheral bit-band region.
pub trait RRRegFieldBitBand<T: RegTag>
where
    Self: RRRegFieldBit<T>,
    Self::Reg: RegBitBand<T> + RReg<T>,
{
    /// Reads the state of the bit through peripheral bit-band region.
    fn read_bit_band(&self) -> bool;

    /// Returns an unsafe constant pointer to the corresponding bit-band
    /// address.
    fn bit_band_ptr(&self) -> *const usize;
}

/// Register field that can write bits through peripheral bit-band region.
pub trait WWRegFieldBitBand<T: RegTag>
where
    Self: SafeWWRegFieldBitBand<T>,
    Self::Reg: RegBitBand<T>,
{
    /// Sets the bit through peripheral bit-band region.
    fn set_bit_band(&self);

    /// Clears the bit through peripheral bit-band region.
    fn clear_bit_band(&self);

    /// Returns an unsafe mutable pointer to the corresponding bit-band address.
    fn bit_band_mut_ptr(&self) -> *mut usize;
}

#[doc(hidden)]
#[marker]
pub unsafe trait SafeWWRegFieldBitBand<T: RegTag>
where
    Self: RegField<T>,
    Self::Reg: RegBitBand<T>,
{
}

impl<T, U> RRRegFieldBitBand<T> for U
where
    T: RegTag,
    U: RRRegFieldBit<T>,
    U::Reg: RegBitBand<T> + RReg<T>,
{
    #[inline]
    fn read_bit_band(&self) -> bool {
        unsafe { read_volatile(self.bit_band_ptr()) != 0 }
    }

    #[inline]
    fn bit_band_ptr(&self) -> *const usize {
        unsafe { Self::Reg::bit_band_addr(Self::OFFSET) as *const usize }
    }
}

impl<T, U> WWRegFieldBitBand<T> for U
where
    T: RegTag,
    U: SafeWWRegFieldBitBand<T>,
    U::Reg: RegBitBand<T>,
{
    #[inline]
    fn set_bit_band(&self) {
        unsafe { write_volatile(self.bit_band_mut_ptr(), 1) };
    }

    #[inline]
    fn clear_bit_band(&self) {
        unsafe { write_volatile(self.bit_band_mut_ptr(), 0) };
    }

    #[inline]
    fn bit_band_mut_ptr(&self) -> *mut usize {
        unsafe { Self::Reg::bit_band_addr(Self::OFFSET) as *mut usize }
    }
}

unsafe impl<T, U> SafeWWRegFieldBitBand<T> for U
where
    T: RegTag,
    U: WoWoRegFieldBit<T>,
    U::Reg: RegBitBand<T> + WoReg<T>,
{
}

unsafe impl<T> SafeWWRegFieldBitBand<Urt> for T
where
    T: WWRegFieldBit<Urt>,
    T::Reg: RegBitBand<Urt> + WReg<Urt>,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use drone_core::reg;

    reg! {
        #[allow(dead_code)]
        mod TST LOW_REG;
        0x4000_0000 0x20 0x0000_0000 RegBitBand;
        TEST_BIT { 0 1 }
    }

    reg! {
        #[allow(dead_code)]
        mod TST HIGH_REG;
        0x400F_FFFC 0x20 0x0000_0000 RegBitBand;
        TEST_BIT { 0 1 }
    }

    type LocalLowReg = tst_low_reg::Reg<Urt>;
    type LocalHighReg = tst_high_reg::Reg<Urt>;

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
