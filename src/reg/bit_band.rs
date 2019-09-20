use crate::reg::{
    field::{RRRegFieldBit, WWRegFieldBit, WoWoRegFieldBit},
    tag::{RegTag, Urt},
    RReg, Reg, WReg, WoReg,
};
use core::ptr::{read_volatile, write_volatile};

/// The peripheral bit-band alias start.
pub const BIT_BAND_BASE: usize = 0x4200_0000;

/// The peripheral bit-band region width.
pub const BIT_BAND_WIDTH: usize = 5;

/// Register located in the peripheral bit-band region.
pub trait RegBitBand<T: RegTag>: Reg<T> {}

/// Readable single-bit field of readable register located in the peripheral
/// bit-band region.
pub trait RRRegFieldBitBand<T: RegTag>
where
    Self: RRRegFieldBit<T>,
    Self::Reg: RegBitBand<T> + RReg<T>,
{
    /// Reads the value of this bit through the peripheral bit-band region
    /// alias.
    fn read_bit_band(&self) -> bool;

    /// Returns a raw pointer to the bit-band alias address of this field.
    ///
    /// See also [`WWRegFieldBitBand::to_bit_band_mut_ptr`].
    fn to_bit_band_ptr(&self) -> *const usize;
}

/// Writable single-bit field of writable register located in the peripheral
/// bit-band region.
pub trait WWRegFieldBitBand<T: RegTag>
where
    Self: WWRegFieldBitBandMarker<T>,
    Self::Reg: RegBitBand<T> + WReg<T>,
{
    /// Sets this bit through the peripheral bit-band region alias.
    fn set_bit_band(&self);

    /// Clears this bit through the peripheral bit-band region alias.
    fn clear_bit_band(&self);

    /// Returns a mutable raw pointer to the bit-band alias address of this
    /// field.
    ///
    /// See also [`RRRegFieldBitBand::to_bit_band_ptr`].
    fn to_bit_band_mut_ptr(&self) -> *mut usize;
}

#[marker]
pub trait WWRegFieldBitBandMarker<T: RegTag>
where
    Self: WWRegFieldBit<T>,
    Self::Reg: RegBitBand<T> + WReg<T>,
{
}

impl<T, R> RRRegFieldBitBand<T> for R
where
    T: RegTag,
    R: RRRegFieldBit<T>,
    R::Reg: RegBitBand<T> + RReg<T>,
{
    #[inline]
    fn read_bit_band(&self) -> bool {
        unsafe { read_volatile(self.to_bit_band_ptr()) != 0 }
    }

    #[inline]
    fn to_bit_band_ptr(&self) -> *const usize {
        bit_band_addr::<T, Self::Reg>(Self::OFFSET) as *const usize
    }
}

impl<T, R> WWRegFieldBitBand<T> for R
where
    T: RegTag,
    R: WWRegFieldBitBandMarker<T>,
    R::Reg: RegBitBand<T> + WReg<T>,
{
    #[inline]
    fn set_bit_band(&self) {
        unsafe { write_volatile(self.to_bit_band_mut_ptr(), 1) };
    }

    #[inline]
    fn clear_bit_band(&self) {
        unsafe { write_volatile(self.to_bit_band_mut_ptr(), 0) };
    }

    #[inline]
    fn to_bit_band_mut_ptr(&self) -> *mut usize {
        bit_band_addr::<T, Self::Reg>(Self::OFFSET) as *mut usize
    }
}

impl<T, R> WWRegFieldBitBandMarker<T> for R
where
    T: RegTag,
    R: WoWoRegFieldBit<T>,
    R::Reg: RegBitBand<T> + WoReg<T>,
{
}

impl<R> WWRegFieldBitBandMarker<Urt> for R
where
    R: WWRegFieldBit<Urt>,
    R::Reg: RegBitBand<Urt> + WReg<Urt>,
{
}

fn bit_band_addr<T: RegTag, R: RegBitBand<T>>(offset: usize) -> usize {
    BIT_BAND_BASE
        + (((R::ADDRESS + (offset >> 3)) & ((0b1 << (BIT_BAND_WIDTH << 2)) - 1)) << BIT_BAND_WIDTH)
        + ((offset & (8 - 1)) << 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use drone_core::reg;

    reg! {
        #[allow(dead_code)]
        mod R LOW;
        0x4000_0000 0x20 0x0000_0000 RegBitBand;
        TEST_BIT { 0 1 }
    }

    reg! {
        #[allow(dead_code)]
        mod R HIGH;
        0x400F_FFFC 0x20 0x0000_0000 RegBitBand;
        TEST_BIT { 0 1 }
    }

    #[test]
    fn reg_bit_band_addr() {
        assert_eq!(bit_band_addr::<Urt, r_low::Reg<Urt>>(0), 0x4200_0000);
        assert_eq!(bit_band_addr::<Urt, r_low::Reg<Urt>>(7), 0x4200_001C);
        assert_eq!(bit_band_addr::<Urt, r_low::Reg<Urt>>(31), 0x4200_007C);
        assert_eq!(bit_band_addr::<Urt, r_high::Reg<Urt>>(24), 0x43FF_FFE0);
        assert_eq!(bit_band_addr::<Urt, r_high::Reg<Urt>>(31), 0x43FF_FFFC);
    }
}
