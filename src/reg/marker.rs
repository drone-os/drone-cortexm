//! Marker traits representing properties of memory-mapped registers.

#[doc(inline)]
pub use drone_core::reg::marker::*;

#[cfg(all(
    feature = "bit-band",
    any(
        cortex_m_core = "cortex_m3_r0p0",
        cortex_m_core = "cortex_m3_r1p0",
        cortex_m_core = "cortex_m3_r1p1",
        cortex_m_core = "cortex_m3_r2p0",
        cortex_m_core = "cortex_m3_r2p1",
        cortex_m_core = "cortex_m4_r0p0",
        cortex_m_core = "cortex_m4_r0p1",
        cortex_m_core = "cortex_m4f_r0p0",
        cortex_m_core = "cortex_m4f_r0p1",
    )
))]
use crate::reg::{
    field::{RRRegFieldBitBand, WWRegFieldBitBand},
    tag::{RegTag, Urt},
    RegBitBand,
};
use crate::reg::{
    field::{WRwRegFieldBitAtomic, WRwRegFieldBitsAtomic},
    tag::{Crt, Srt},
    RwRegAtomic,
};
use drone_core::reg::marker as core_marker;

/// Synchronized read-write register.
#[marker]
pub trait SRwReg
where
    Self: core_marker::SRwReg,
    Self: for<'a> RwRegAtomic<'a, Srt>,
{
}

impl<R> SRwReg for R
where
    R: core_marker::SRwReg,
    R: for<'a> RwRegAtomic<'a, Srt>,
{
}

/// Copyable read-write register.
#[marker]
pub trait CRwReg
where
    Self: core_marker::CRwReg,
    Self: for<'a> RwRegAtomic<'a, Crt>,
{
}

impl<R> CRwReg for R
where
    R: core_marker::CRwReg,
    R: for<'a> RwRegAtomic<'a, Crt>,
{
}

/// Synchronized single-bit read-write field of read-write register.
#[marker]
pub trait SRwRwRegFieldBit
where
    Self: core_marker::SRwRwRegFieldBit,
    Self: WRwRegFieldBitAtomic<Srt>,
    Self::Reg: SRwReg,
{
}

impl<R> SRwRwRegFieldBit for R
where
    R: core_marker::SRwRwRegFieldBit,
    R: WRwRegFieldBitAtomic<Srt>,
    R::Reg: SRwReg,
{
}

/// Synchronized multi-bit read-write field of read-write register.
#[marker]
pub trait SRwRwRegFieldBits
where
    Self: core_marker::SRwRwRegFieldBits,
    Self: WRwRegFieldBitsAtomic<Srt>,
    Self::Reg: SRwReg,
{
}

impl<R> SRwRwRegFieldBits for R
where
    R: core_marker::SRwRwRegFieldBits,
    R: WRwRegFieldBitsAtomic<Srt>,
    R::Reg: SRwReg,
{
}

/// Synchronized single-bit write-only field of read-write register.
#[marker]
pub trait SWoRwRegFieldBit
where
    Self: core_marker::SWoRwRegFieldBit,
    Self: WRwRegFieldBitAtomic<Srt>,
    Self::Reg: SRwReg,
{
}

impl<R> SWoRwRegFieldBit for R
where
    R: core_marker::SWoRwRegFieldBit,
    R: WRwRegFieldBitAtomic<Srt>,
    R::Reg: SRwReg,
{
}

/// Synchronized multi-bit write-only field of read-write register.
#[marker]
pub trait SWoRwRegFieldBits
where
    Self: core_marker::SWoRwRegFieldBits,
    Self: WRwRegFieldBitsAtomic<Srt>,
    Self::Reg: SRwReg,
{
}

impl<R> SWoRwRegFieldBits for R
where
    R: core_marker::SWoRwRegFieldBits,
    R: WRwRegFieldBitsAtomic<Srt>,
    R::Reg: SRwReg,
{
}

/// Copyable single-bit read-write field of read-write register.
#[marker]
pub trait CRwRwRegFieldBit
where
    Self: core_marker::CRwRwRegFieldBit,
    Self: WRwRegFieldBitAtomic<Crt>,
    Self::Reg: CRwReg,
{
}

impl<R> CRwRwRegFieldBit for R
where
    R: core_marker::CRwRwRegFieldBit,
    R: WRwRegFieldBitAtomic<Crt>,
    R::Reg: CRwReg,
{
}

/// Copyable multi-bit read-write field of read-write register.
#[marker]
pub trait CRwRwRegFieldBits
where
    Self: core_marker::CRwRwRegFieldBits,
    Self: WRwRegFieldBitsAtomic<Crt>,
    Self::Reg: CRwReg,
{
}

impl<R> CRwRwRegFieldBits for R
where
    R: core_marker::CRwRwRegFieldBits,
    R: WRwRegFieldBitsAtomic<Crt>,
    R::Reg: CRwReg,
{
}

/// Copyable single-bit write-only field of read-write register.
#[marker]
pub trait CWoRwRegFieldBit
where
    Self: core_marker::CWoRwRegFieldBit,
    Self: WRwRegFieldBitAtomic<Crt>,
    Self::Reg: CRwReg,
{
}

impl<R> CWoRwRegFieldBit for R
where
    R: core_marker::CWoRwRegFieldBit,
    R: WRwRegFieldBitAtomic<Crt>,
    R::Reg: CRwReg,
{
}

/// Copyable multi-bit write-only field of read-write register.
#[marker]
pub trait CWoRwRegFieldBits
where
    Self: core_marker::CWoRwRegFieldBits,
    Self: WRwRegFieldBitsAtomic<Crt>,
    Self::Reg: CRwReg,
{
}

impl<R> CWoRwRegFieldBits for R
where
    R: core_marker::CWoRwRegFieldBits,
    R: WRwRegFieldBitsAtomic<Crt>,
    R::Reg: CRwReg,
{
}

#[cfg(all(
    feature = "bit-band",
    any(
        cortex_m_core = "cortex_m3_r0p0",
        cortex_m_core = "cortex_m3_r1p0",
        cortex_m_core = "cortex_m3_r1p1",
        cortex_m_core = "cortex_m3_r2p0",
        cortex_m_core = "cortex_m3_r2p1",
        cortex_m_core = "cortex_m4_r0p0",
        cortex_m_core = "cortex_m4_r0p1",
        cortex_m_core = "cortex_m4f_r0p0",
        cortex_m_core = "cortex_m4f_r0p1",
    )
))]
mod bit_band {
    use super::*;

    /// Bit-band read-write register.
    #[marker]
    pub trait RwRegBitBand<T: RegTag>
    where
        Self: RwReg<T>,
        Self: RegBitBand<T>,
    {
    }

    impl<R, T: RegTag> RwRegBitBand<T> for R
    where
        R: RwReg<T>,
        R: RegBitBand<T>,
    {
    }

    /// Bit-band read-only register.
    #[marker]
    pub trait RoRegBitBand<T: RegTag>
    where
        Self: RoReg<T>,
        Self: RegBitBand<T>,
    {
    }

    impl<R, T: RegTag> RoRegBitBand<T> for R
    where
        R: RoReg<T>,
        R: RegBitBand<T>,
    {
    }

    /// Bit-band write-only register.
    #[marker]
    pub trait WoRegBitBand<T: RegTag>
    where
        Self: WoReg<T>,
        Self: RegBitBand<T>,
    {
    }

    impl<R, T: RegTag> WoRegBitBand<T> for R
    where
        R: WoReg<T>,
        R: RegBitBand<T>,
    {
    }

    /// Unsynchronized bit-band read-write register.
    #[marker]
    pub trait URwRegBitBand
    where
        Self: URwReg,
        Self: RegBitBand<Urt>,
    {
    }

    impl<R> URwRegBitBand for R
    where
        R: URwReg,
        R: RegBitBand<Urt>,
    {
    }

    /// Unsynchronized bit-band read-only register.
    #[marker]
    pub trait URoRegBitBand
    where
        Self: URoReg,
        Self: RegBitBand<Urt>,
    {
    }

    impl<R> URoRegBitBand for R
    where
        R: URoReg,
        R: RegBitBand<Urt>,
    {
    }

    /// Unsynchronized bit-band write-only register.
    #[marker]
    pub trait UWoRegBitBand
    where
        Self: UWoReg,
        Self: RegBitBand<Urt>,
    {
    }

    impl<R> UWoRegBitBand for R
    where
        R: UWoReg,
        R: RegBitBand<Urt>,
    {
    }

    /// Synchronized bit-band read-write register.
    #[marker]
    pub trait SRwRegBitBand
    where
        Self: SRwReg,
        Self: RegBitBand<Srt>,
    {
    }

    impl<R> SRwRegBitBand for R
    where
        R: SRwReg,
        R: RegBitBand<Srt>,
    {
    }

    /// Synchronized bit-band read-only register.
    #[marker]
    pub trait SRoRegBitBand
    where
        Self: SRoReg,
        Self: RegBitBand<Srt>,
    {
    }

    impl<R> SRoRegBitBand for R
    where
        R: SRoReg,
        R: RegBitBand<Srt>,
    {
    }

    /// Synchronized bit-band write-only register.
    #[marker]
    pub trait SWoRegBitBand
    where
        Self: SWoReg,
        Self: RegBitBand<Srt>,
    {
    }

    impl<R> SWoRegBitBand for R
    where
        R: SWoReg,
        R: RegBitBand<Srt>,
    {
    }

    /// Copyable bit-band read-write register.
    #[marker]
    pub trait CRwRegBitBand
    where
        Self: CRwReg,
        Self: RegBitBand<Crt>,
    {
    }

    impl<R> CRwRegBitBand for R
    where
        R: CRwReg,
        R: RegBitBand<Crt>,
    {
    }

    /// Copyable bit-band read-only register.
    #[marker]
    pub trait CRoRegBitBand
    where
        Self: CRoReg,
        Self: RegBitBand<Crt>,
    {
    }

    impl<R> CRoRegBitBand for R
    where
        R: CRoReg,
        R: RegBitBand<Crt>,
    {
    }

    /// Copyable bit-band write-only register.
    #[marker]
    pub trait CWoRegBitBand
    where
        Self: CWoReg,
        Self: RegBitBand<Crt>,
    {
    }

    impl<R> CWoRegBitBand for R
    where
        R: CWoReg,
        R: RegBitBand<Crt>,
    {
    }

    /// Single-bit read-write field of read-write register.
    #[marker]
    pub trait RwRwRegFieldBitBand<T: RegTag>
    where
        Self: RwRwRegFieldBit<T>,
        Self: RRRegFieldBitBand<T>,
        Self::Reg: RwRegBitBand<T>,
    {
    }

    impl<R, T: RegTag> RwRwRegFieldBitBand<T> for R
    where
        R: RwRwRegFieldBit<T>,
        R: RRRegFieldBitBand<T>,
        R::Reg: RwRegBitBand<T>,
    {
    }

    /// Single-bit write-only field of read-write register.
    #[marker]
    pub trait WoRwRegFieldBitBand<T: RegTag>
    where
        Self: WoRwRegFieldBit<T>,
        Self::Reg: RwRegBitBand<T>,
    {
    }

    impl<R, T: RegTag> WoRwRegFieldBitBand<T> for R
    where
        R: WoRwRegFieldBit<T>,
        R::Reg: RwRegBitBand<T>,
    {
    }

    /// Single-bit write-only field of write-only register.
    #[marker]
    pub trait WoWoRegFieldBitBand<T: RegTag>
    where
        Self: WoWoRegFieldBit<T>,
        Self: WWRegFieldBitBand<T>,
        Self::Reg: WoRegBitBand<T>,
    {
    }

    impl<R, T: RegTag> WoWoRegFieldBitBand<T> for R
    where
        R: WoWoRegFieldBit<T>,
        R: WWRegFieldBitBand<T>,
        R::Reg: WoRegBitBand<T>,
    {
    }

    /// Single-bit read-only field of read-write register.
    #[marker]
    pub trait RoRwRegFieldBitBand<T: RegTag>
    where
        Self: RoRwRegFieldBit<T>,
        Self: RRRegFieldBitBand<T>,
        Self::Reg: RwRegBitBand<T>,
    {
    }

    impl<R, T: RegTag> RoRwRegFieldBitBand<T> for R
    where
        R: RoRwRegFieldBit<T>,
        R: RRRegFieldBitBand<T>,
        R::Reg: RwRegBitBand<T>,
    {
    }

    /// Single-bit read-only field of read-only register.
    #[marker]
    pub trait RoRoRegFieldBitBand<T: RegTag>
    where
        Self: RoRoRegFieldBit<T>,
        Self: RRRegFieldBitBand<T>,
        Self::Reg: RoRegBitBand<T>,
    {
    }

    impl<R, T: RegTag> RoRoRegFieldBitBand<T> for R
    where
        R: RoRoRegFieldBit<T>,
        R: RRRegFieldBitBand<T>,
        R::Reg: RoRegBitBand<T>,
    {
    }

    /// Unsynchronized single-bit read-write field of read-write register.
    #[marker]
    pub trait URwRwRegFieldBitBand
    where
        Self: RwRwRegFieldBitBand<Urt>,
        Self: WWRegFieldBitBand<Urt>,
        Self::Reg: URwRegBitBand,
    {
    }

    impl<R> URwRwRegFieldBitBand for R
    where
        R: RwRwRegFieldBitBand<Urt>,
        R: WWRegFieldBitBand<Urt>,
        R::Reg: URwRegBitBand,
    {
    }

    /// Unsynchronized single-bit write-only field of read-write register.
    #[marker]
    pub trait UWoRwRegFieldBitBand
    where
        Self: WoRwRegFieldBitBand<Urt>,
        Self: WWRegFieldBitBand<Urt>,
        Self::Reg: URwRegBitBand,
    {
    }

    impl<R> UWoRwRegFieldBitBand for R
    where
        R: WoRwRegFieldBitBand<Urt>,
        R: WWRegFieldBitBand<Urt>,
        R::Reg: URwRegBitBand,
    {
    }

    /// Unsynchronized single-bit write-only field of write-only register.
    #[marker]
    pub trait UWoWoRegFieldBitBand
    where
        Self: WoWoRegFieldBitBand<Urt>,
        Self::Reg: UWoRegBitBand,
    {
    }

    impl<R> UWoWoRegFieldBitBand for R
    where
        R: WoWoRegFieldBitBand<Urt>,
        R::Reg: UWoRegBitBand,
    {
    }

    /// Unsynchronized single-bit read-only field of read-write register.
    #[marker]
    pub trait URoRwRegFieldBitBand
    where
        Self: RoRwRegFieldBitBand<Urt>,
        Self::Reg: URwRegBitBand,
    {
    }

    impl<R> URoRwRegFieldBitBand for R
    where
        R: RoRwRegFieldBitBand<Urt>,
        R::Reg: URwRegBitBand,
    {
    }

    /// Unsynchronized single-bit read-only field of read-only register.
    #[marker]
    pub trait URoRoRegFieldBitBand
    where
        Self: RoRoRegFieldBitBand<Urt>,
        Self::Reg: URoRegBitBand,
    {
    }

    impl<R> URoRoRegFieldBitBand for R
    where
        R: RoRoRegFieldBitBand<Urt>,
        R::Reg: URoRegBitBand,
    {
    }

    /// Synchronized single-bit read-write field of read-write register.
    #[marker]
    pub trait SRwRwRegFieldBitBand
    where
        Self: RwRwRegFieldBitBand<Srt>,
        Self: WRwRegFieldBitAtomic<Srt>,
        Self::Reg: SRwRegBitBand,
    {
    }

    impl<R> SRwRwRegFieldBitBand for R
    where
        R: RwRwRegFieldBitBand<Srt>,
        R: WRwRegFieldBitAtomic<Srt>,
        R::Reg: SRwRegBitBand,
    {
    }

    /// Synchronized single-bit write-only field of read-write register.
    #[marker]
    pub trait SWoRwRegFieldBitBand
    where
        Self: WoRwRegFieldBitBand<Srt>,
        Self: WRwRegFieldBitAtomic<Srt>,
        Self::Reg: SRwRegBitBand,
    {
    }

    impl<R> SWoRwRegFieldBitBand for R
    where
        R: WoRwRegFieldBitBand<Srt>,
        R: WRwRegFieldBitAtomic<Srt>,
        R::Reg: SRwRegBitBand,
    {
    }

    /// Synchronized single-bit write-only field of write-only register.
    #[marker]
    pub trait SWoWoRegFieldBitBand
    where
        Self: WoWoRegFieldBitBand<Srt>,
        Self::Reg: SWoRegBitBand,
    {
    }

    impl<R> SWoWoRegFieldBitBand for R
    where
        R: WoWoRegFieldBitBand<Srt>,
        R::Reg: SWoRegBitBand,
    {
    }

    /// Synchronized single-bit read-only field of read-write register.
    #[marker]
    pub trait SRoRwRegFieldBitBand
    where
        Self: RoRwRegFieldBitBand<Srt>,
        Self::Reg: SRwRegBitBand,
    {
    }

    impl<R> SRoRwRegFieldBitBand for R
    where
        R: RoRwRegFieldBitBand<Srt>,
        R::Reg: SRwRegBitBand,
    {
    }

    /// Synchronized single-bit read-only field of read-only register.
    #[marker]
    pub trait SRoRoRegFieldBitBand
    where
        Self: RoRoRegFieldBitBand<Srt>,
        Self::Reg: SRoRegBitBand,
    {
    }

    impl<R> SRoRoRegFieldBitBand for R
    where
        R: RoRoRegFieldBitBand<Srt>,
        R::Reg: SRoRegBitBand,
    {
    }

    /// Copyable single-bit read-write field of read-write register.
    #[marker]
    pub trait CRwRwRegFieldBitBand
    where
        Self: RwRwRegFieldBitBand<Crt>,
        Self: WRwRegFieldBitAtomic<Crt>,
        Self: Copy,
        Self::Reg: CRwRegBitBand,
    {
    }

    impl<R> CRwRwRegFieldBitBand for R
    where
        R: RwRwRegFieldBitBand<Crt>,
        R: WRwRegFieldBitAtomic<Crt>,
        R: Copy,
        R::Reg: CRwRegBitBand,
    {
    }

    /// Copyable single-bit write-only field of read-write register.
    #[marker]
    pub trait CWoRwRegFieldBitBand
    where
        Self: WoRwRegFieldBitBand<Crt>,
        Self: WRwRegFieldBitAtomic<Crt>,
        Self: Copy,
        Self::Reg: CRwRegBitBand,
    {
    }

    impl<R> CWoRwRegFieldBitBand for R
    where
        R: WoRwRegFieldBitBand<Crt>,
        R: WRwRegFieldBitAtomic<Crt>,
        R: Copy,
        R::Reg: CRwRegBitBand,
    {
    }

    /// Copyable single-bit write-only field of write-only register.
    #[marker]
    pub trait CWoWoRegFieldBitBand
    where
        Self: WoWoRegFieldBitBand<Crt>,
        Self: Copy,
        Self::Reg: CWoRegBitBand,
    {
    }

    impl<R> CWoWoRegFieldBitBand for R
    where
        R: WoWoRegFieldBitBand<Crt>,
        R: Copy,
        R::Reg: CWoRegBitBand,
    {
    }

    /// Copyable single-bit read-only field of read-write register.
    #[marker]
    pub trait CRoRwRegFieldBitBand
    where
        Self: RoRwRegFieldBitBand<Crt>,
        Self: Copy,
        Self::Reg: CRwRegBitBand,
    {
    }

    impl<R> CRoRwRegFieldBitBand for R
    where
        R: RoRwRegFieldBitBand<Crt>,
        R: Copy,
        R::Reg: CRwRegBitBand,
    {
    }

    /// Copyable single-bit read-only field of read-only register.
    #[marker]
    pub trait CRoRoRegFieldBitBand
    where
        Self: RoRoRegFieldBitBand<Crt>,
        Self: Copy,
        Self::Reg: CRoRegBitBand,
    {
    }

    impl<R> CRoRoRegFieldBitBand for R
    where
        R: RoRoRegFieldBitBand<Crt>,
        R: Copy,
        R::Reg: CRoRegBitBand,
    {
    }
}

#[cfg(all(
    feature = "bit-band",
    any(
        cortex_m_core = "cortex_m3_r0p0",
        cortex_m_core = "cortex_m3_r1p0",
        cortex_m_core = "cortex_m3_r1p1",
        cortex_m_core = "cortex_m3_r2p0",
        cortex_m_core = "cortex_m3_r2p1",
        cortex_m_core = "cortex_m4_r0p0",
        cortex_m_core = "cortex_m4_r0p1",
        cortex_m_core = "cortex_m4f_r0p0",
        cortex_m_core = "cortex_m4f_r0p1",
    )
))]
pub use self::bit_band::*;
