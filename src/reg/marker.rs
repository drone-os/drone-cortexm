//! Marker traits for memory-mapped registers.

pub use drone_core::reg::marker::{
  CRoReg, CRoRoRegFieldBit, CRoRoRegFieldBits, CRoRwRegFieldBit,
  CRoRwRegFieldBits, CWoReg, CWoWoRegFieldBit, CWoWoRegFieldBits,
  RoRoRegFieldBit, RoRoRegFieldBits, RoRwRegFieldBit, RoRwRegFieldBits, RwReg,
  RwRwRegFieldBit, RwRwRegFieldBits, SRoReg, SRoRoRegFieldBit,
  SRoRoRegFieldBits, SRoRwRegFieldBit, SRoRwRegFieldBits, SWoReg,
  SWoWoRegFieldBit, SWoWoRegFieldBits, URoReg, URoRoRegFieldBit,
  URoRoRegFieldBits, URoRwRegFieldBit, URoRwRegFieldBits, URwReg,
  URwRwRegFieldBit, URwRwRegFieldBits, UWoReg, UWoRwRegFieldBit,
  UWoRwRegFieldBits, UWoWoRegFieldBit, UWoWoRegFieldBits, WoRwRegFieldBit,
  WoRwRegFieldBits,
};

use drone_core::reg::marker;
use reg::prelude::*;

// {{{ SRwReg
/// Synchronized read-write register token.
#[marker]
pub trait SRwReg
where
  Self: marker::SRwReg,
  Self: for<'a> RwRegAtomicRef<'a, Srt>,
{
}

impl<R> SRwReg for R
where
  R: marker::SRwReg,
  R: for<'a> RwRegAtomicRef<'a, Srt>,
{
}

// }}}
// {{{ CRwReg
/// Copyable read-write register token.
#[marker]
pub trait CRwReg
where
  Self: marker::CRwReg,
  Self: for<'a> RwRegAtomicRef<'a, Crt>,
{
}

impl<R> CRwReg for R
where
  R: marker::CRwReg,
  R: for<'a> RwRegAtomicRef<'a, Crt>,
{
}

// }}}
// {{{ RwRegBitBand
/// Bit-band read-write register token.
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

// }}}
// {{{ RoRegBitBand
/// Bit-band read-only register token.
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

// }}}
// {{{ WoRegBitBand
/// Bit-band write-only register token.
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

// }}}
// {{{ URwRegBitBand
/// Unsynchronized bit-band read-write register token.
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

// }}}
// {{{ URoRegBitBand
/// Unsynchronized bit-band read-only register token.
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

// }}}
// {{{ UWoRegBitBand
/// Unsynchronized bit-band write-only register token.
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

// }}}
// {{{ SRwRegBitBand
/// Synchronized bit-band read-write register token.
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

// }}}
// {{{ SRoRegBitBand
/// Synchronized bit-band read-only register token.
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

// }}}
// {{{ SWoRegBitBand
/// Synchronized bit-band write-only register token.
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

// }}}
// {{{ CRwRegBitBand
/// Copyable bit-band read-write register token.
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

// }}}
// {{{ CRoRegBitBand
/// Copyable bit-band read-only register token.
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

// }}}
// {{{ CWoRegBitBand
/// Copyable bit-band write-only register token.
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

// }}}
// {{{ SRwRwRegFieldBit
/// Synchronized one-bit read-write field of read-write register token.
#[marker]
pub trait SRwRwRegFieldBit
where
  Self: marker::SRwRwRegFieldBit,
  Self: WRwRegFieldBitAtomic<Srt>,
  Self::Reg: SRwReg,
{
}

impl<R> SRwRwRegFieldBit for R
where
  R: marker::SRwRwRegFieldBit,
  R: WRwRegFieldBitAtomic<Srt>,
  R::Reg: SRwReg,
{
}

// }}}
// {{{ SRwRwRegFieldBits
/// Synchronized multi-bit read-write field of read-write register token.
#[marker]
pub trait SRwRwRegFieldBits
where
  Self: marker::SRwRwRegFieldBits,
  Self: WRwRegFieldBitsAtomic<Srt>,
  Self::Reg: SRwReg,
{
}

impl<R> SRwRwRegFieldBits for R
where
  R: marker::SRwRwRegFieldBits,
  R: WRwRegFieldBitsAtomic<Srt>,
  R::Reg: SRwReg,
{
}

// }}}
// {{{ SWoRwRegFieldBit
/// Synchronized one-bit write-only field of read-write register token.
#[marker]
pub trait SWoRwRegFieldBit
where
  Self: marker::SWoRwRegFieldBit,
  Self: WRwRegFieldBitAtomic<Srt>,
  Self::Reg: SRwReg,
{
}

impl<R> SWoRwRegFieldBit for R
where
  R: marker::SWoRwRegFieldBit,
  R: WRwRegFieldBitAtomic<Srt>,
  R::Reg: SRwReg,
{
}

// }}}
// {{{ SWoRwRegFieldBits
/// Synchronized multi-bit write-only field of read-write register token.
#[marker]
pub trait SWoRwRegFieldBits
where
  Self: marker::SWoRwRegFieldBits,
  Self: WRwRegFieldBitsAtomic<Srt>,
  Self::Reg: SRwReg,
{
}

impl<R> SWoRwRegFieldBits for R
where
  R: marker::SWoRwRegFieldBits,
  R: WRwRegFieldBitsAtomic<Srt>,
  R::Reg: SRwReg,
{
}

// }}}
// {{{ CRwRwRegFieldBit
/// Copyable one-bit read-write field of read-write register token.
#[marker]
pub trait CRwRwRegFieldBit
where
  Self: marker::CRwRwRegFieldBit,
  Self: WRwRegFieldBitAtomic<Crt>,
  Self::Reg: CRwReg,
{
}

impl<R> CRwRwRegFieldBit for R
where
  R: marker::CRwRwRegFieldBit,
  R: WRwRegFieldBitAtomic<Crt>,
  R::Reg: CRwReg,
{
}

// }}}
// {{{ CRwRwRegFieldBits
/// Copyable multi-bit read-write field of read-write register token.
#[marker]
pub trait CRwRwRegFieldBits
where
  Self: marker::CRwRwRegFieldBits,
  Self: WRwRegFieldBitsAtomic<Crt>,
  Self::Reg: CRwReg,
{
}

impl<R> CRwRwRegFieldBits for R
where
  R: marker::CRwRwRegFieldBits,
  R: WRwRegFieldBitsAtomic<Crt>,
  R::Reg: CRwReg,
{
}

// }}}
// {{{ CWoRwRegFieldBit
/// Copyable one-bit write-only field of read-write register token.
#[marker]
pub trait CWoRwRegFieldBit
where
  Self: marker::CWoRwRegFieldBit,
  Self: WRwRegFieldBitAtomic<Crt>,
  Self::Reg: CRwReg,
{
}

impl<R> CWoRwRegFieldBit for R
where
  R: marker::CWoRwRegFieldBit,
  R: WRwRegFieldBitAtomic<Crt>,
  R::Reg: CRwReg,
{
}

// }}}
// {{{ CWoRwRegFieldBits
/// Copyable multi-bit write-only field of read-write register token.
#[marker]
pub trait CWoRwRegFieldBits
where
  Self: marker::CWoRwRegFieldBits,
  Self: WRwRegFieldBitsAtomic<Crt>,
  Self::Reg: CRwReg,
{
}

impl<R> CWoRwRegFieldBits for R
where
  R: marker::CWoRwRegFieldBits,
  R: WRwRegFieldBitsAtomic<Crt>,
  R::Reg: CRwReg,
{
}

// }}}
// {{{ RwRwRegFieldBitBand
/// One-bit read-write field of read-write register token.
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

// }}}
// {{{ WoRwRegFieldBitBand
/// One-bit write-only field of read-write register token.
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

// }}}
// {{{ WoWoRegFieldBitBand
/// One-bit write-only field of write-only register token.
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

// }}}
// {{{ RoRwRegFieldBitBand
/// One-bit read-only field of read-write register token.
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

// }}}
// {{{ RoRoRegFieldBitBand
/// One-bit read-only field of read-only register token.
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

// }}}
// {{{ URwRwRegFieldBitBand
/// Unsynchronized one-bit read-write field of read-write register token.
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

// }}}
// {{{ UWoRwRegFieldBitBand
/// Unsynchronized one-bit write-only field of read-write register token.
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

// }}}
// {{{ UWoWoRegFieldBitBand
/// Unsynchronized one-bit write-only field of write-only register token.
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

// }}}
// {{{ URoRwRegFieldBitBand
/// Unsynchronized one-bit read-only field of read-write register token.
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

// }}}
// {{{ URoRoRegFieldBitBand
/// Unsynchronized one-bit read-only field of read-only register token.
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

// }}}
// {{{ SRwRwRegFieldBitBand
/// Synchronized one-bit read-write field of read-write register token.
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

// }}}
// {{{ SWoRwRegFieldBitBand
/// Synchronized one-bit write-only field of read-write register token.
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

// }}}
// {{{ SWoWoRegFieldBitBand
/// Synchronized one-bit write-only field of write-only register token.
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

// }}}
// {{{ SRoRwRegFieldBitBand
/// Synchronized one-bit read-only field of read-write register token.
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

// }}}
// {{{ SRoRoRegFieldBitBand
/// Synchronized one-bit read-only field of read-only register token.
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

// }}}
// {{{ CRwRwRegFieldBitBand
/// Copyable one-bit read-write field of read-write register token.
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

// }}}
// {{{ CWoRwRegFieldBitBand
/// Copyable one-bit write-only field of read-write register token.
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

// }}}
// {{{ CWoWoRegFieldBitBand
/// Copyable one-bit write-only field of write-only register token.
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

// }}}
// {{{ CRoRwRegFieldBitBand
/// Copyable one-bit read-only field of read-write register token.
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

// }}}
// {{{ CRoRoRegFieldBitBand
/// Copyable one-bit read-only field of read-only register token.
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

// }}}
// vim: set fdm=marker fmr={{{,}}} :
