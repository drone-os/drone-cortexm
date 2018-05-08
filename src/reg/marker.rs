//! Marker traits for memory-mapped registers.

pub use drone_core::reg::marker::{
  self, FRoReg, FRoRoRegFieldBit, FRoRoRegFieldBits, FRoRwRegFieldBit,
  FRoRwRegFieldBits, FWoReg, FWoWoRegFieldBit, FWoWoRegFieldBits, SRoReg,
  SRoRoRegFieldBit, SRoRoRegFieldBits, SRoRwRegFieldBit, SRoRwRegFieldBits,
  SWoReg, SWoWoRegFieldBit, SWoWoRegFieldBits, URoReg, URoRoRegFieldBit,
  URoRoRegFieldBits, URoRwRegFieldBit, URoRwRegFieldBits, URwReg,
  URwRwRegFieldBit, URwRwRegFieldBits, UWoReg, UWoRwRegFieldBit,
  UWoRwRegFieldBits, UWoWoRegFieldBit, UWoWoRegFieldBits,
};

use reg::prelude::*;

// {{{ SRwReg
/// Synchronized read-write register token.
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
// {{{ FRwReg
/// Forkable read-write register token.
pub trait FRwReg
where
  Self: marker::FRwReg,
  Self: for<'a> RwRegAtomicRef<'a, Frt>,
{
}

impl<R> FRwReg for R
where
  R: marker::FRwReg,
  R: for<'a> RwRegAtomicRef<'a, Frt>,
{
}

// }}}
// {{{ URwRegBitBand
/// Unsynchronized bit-band read-write register token.
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
// {{{ FRwRegBitBand
/// Forkable bit-band read-write register token.
pub trait FRwRegBitBand
where
  Self: FRwReg,
  Self: RegBitBand<Frt>,
{
}

impl<R> FRwRegBitBand for R
where
  R: FRwReg,
  R: RegBitBand<Frt>,
{
}

// }}}
// {{{ FRoRegBitBand
/// Forkable bit-band read-only register token.
pub trait FRoRegBitBand
where
  Self: FRoReg,
  Self: RegBitBand<Frt>,
{
}

impl<R> FRoRegBitBand for R
where
  R: FRoReg,
  R: RegBitBand<Frt>,
{
}

// }}}
// {{{ FWoRegBitBand
/// Forkable bit-band write-only register token.
pub trait FWoRegBitBand
where
  Self: FWoReg,
  Self: RegBitBand<Frt>,
{
}

impl<R> FWoRegBitBand for R
where
  R: FWoReg,
  R: RegBitBand<Frt>,
{
}

// }}}
// {{{ SRwRwRegFieldBit
/// Synchronized one-bit read-write field of read-write register token.
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
// {{{ FRwRwRegFieldBit
/// Forkable one-bit read-write field of read-write register token.
pub trait FRwRwRegFieldBit
where
  Self: marker::FRwRwRegFieldBit,
  Self: WRwRegFieldBitAtomic<Frt>,
  Self::Reg: FRwReg,
{
}

impl<R> FRwRwRegFieldBit for R
where
  R: marker::FRwRwRegFieldBit,
  R: WRwRegFieldBitAtomic<Frt>,
  R::Reg: FRwReg,
{
}

// }}}
// {{{ FRwRwRegFieldBits
/// Forkable multi-bit read-write field of read-write register token.
pub trait FRwRwRegFieldBits
where
  Self: marker::FRwRwRegFieldBits,
  Self: WRwRegFieldBitsAtomic<Frt>,
  Self::Reg: FRwReg,
{
}

impl<R> FRwRwRegFieldBits for R
where
  R: marker::FRwRwRegFieldBits,
  R: WRwRegFieldBitsAtomic<Frt>,
  R::Reg: FRwReg,
{
}

// }}}
// {{{ FWoRwRegFieldBit
/// Forkable one-bit write-only field of read-write register token.
pub trait FWoRwRegFieldBit
where
  Self: marker::FWoRwRegFieldBit,
  Self: WRwRegFieldBitAtomic<Frt>,
  Self::Reg: FRwReg,
{
}

impl<R> FWoRwRegFieldBit for R
where
  R: marker::FWoRwRegFieldBit,
  R: WRwRegFieldBitAtomic<Frt>,
  R::Reg: FRwReg,
{
}

// }}}
// {{{ FWoRwRegFieldBits
/// Forkable multi-bit write-only field of read-write register token.
pub trait FWoRwRegFieldBits
where
  Self: marker::FWoRwRegFieldBits,
  Self: WRwRegFieldBitsAtomic<Frt>,
  Self::Reg: FRwReg,
{
}

impl<R> FWoRwRegFieldBits for R
where
  R: marker::FWoRwRegFieldBits,
  R: WRwRegFieldBitsAtomic<Frt>,
  R::Reg: FRwReg,
{
}

// }}}
// {{{ URwRwRegFieldBitBand
/// Unsynchronized one-bit read-write field of read-write register token.
pub trait URwRwRegFieldBitBand
where
  Self: URwRwRegFieldBit,
  Self: RRRegFieldBitBand<Urt>,
  Self: WWRegFieldBitBand<Urt>,
  Self::Reg: URwRegBitBand,
{
}

impl<R> URwRwRegFieldBitBand for R
where
  R: URwRwRegFieldBit,
  R: RRRegFieldBitBand<Urt>,
  R: WWRegFieldBitBand<Urt>,
  R::Reg: URwRegBitBand,
{
}

// }}}
// {{{ UWoRwRegFieldBitBand
/// Unsynchronized one-bit write-only field of read-write register token.
pub trait UWoRwRegFieldBitBand
where
  Self: UWoRwRegFieldBit,
  Self: WWRegFieldBitBand<Urt>,
  Self::Reg: URwRegBitBand,
{
}

impl<R> UWoRwRegFieldBitBand for R
where
  R: UWoRwRegFieldBit,
  R: WWRegFieldBitBand<Urt>,
  R::Reg: URwRegBitBand,
{
}

// }}}
// {{{ UWoWoRegFieldBitBand
/// Unsynchronized one-bit write-only field of write-only register token.
pub trait UWoWoRegFieldBitBand
where
  Self: UWoWoRegFieldBit,
  Self: WWRegFieldBitBand<Urt>,
  Self::Reg: UWoRegBitBand,
{
}

impl<R> UWoWoRegFieldBitBand for R
where
  R: UWoWoRegFieldBit,
  R: WWRegFieldBitBand<Urt>,
  R::Reg: UWoRegBitBand,
{
}

// }}}
// {{{ URoRwRegFieldBitBand
/// Unsynchronized one-bit read-only field of read-write register token.
pub trait URoRwRegFieldBitBand
where
  Self: URoRwRegFieldBit,
  Self: RRRegFieldBitBand<Urt>,
  Self::Reg: URwRegBitBand,
{
}

impl<R> URoRwRegFieldBitBand for R
where
  R: URoRwRegFieldBit,
  R: RRRegFieldBitBand<Urt>,
  R::Reg: URwRegBitBand,
{
}

// }}}
// {{{ URoRoRegFieldBitBand
/// Unsynchronized one-bit read-only field of read-only register token.
pub trait URoRoRegFieldBitBand
where
  Self: URoRoRegFieldBit,
  Self: RRRegFieldBitBand<Urt>,
  Self::Reg: URoRegBitBand,
{
}

impl<R> URoRoRegFieldBitBand for R
where
  R: URoRoRegFieldBit,
  R: RRRegFieldBitBand<Urt>,
  R::Reg: URoRegBitBand,
{
}

// }}}
// {{{ SRwRwRegFieldBitBand
/// Synchronized one-bit read-write field of read-write register token.
pub trait SRwRwRegFieldBitBand
where
  Self: SRwRwRegFieldBit,
  Self: RRRegFieldBitBand<Srt>,
  Self: WWRegFieldBitBand<Srt>,
  Self::Reg: SRwRegBitBand,
{
}

impl<R> SRwRwRegFieldBitBand for R
where
  R: SRwRwRegFieldBit,
  R: RRRegFieldBitBand<Srt>,
  R: WWRegFieldBitBand<Srt>,
  R::Reg: SRwRegBitBand,
{
}

// }}}
// {{{ SWoRwRegFieldBitBand
/// Synchronized one-bit write-only field of read-write register token.
pub trait SWoRwRegFieldBitBand
where
  Self: SWoRwRegFieldBit,
  Self: WWRegFieldBitBand<Srt>,
  Self::Reg: SRwRegBitBand,
{
}

impl<R> SWoRwRegFieldBitBand for R
where
  R: SWoRwRegFieldBit,
  R: WWRegFieldBitBand<Srt>,
  R::Reg: SRwRegBitBand,
{
}

// }}}
// {{{ SWoWoRegFieldBitBand
/// Synchronized one-bit write-only field of write-only register token.
pub trait SWoWoRegFieldBitBand
where
  Self: SWoWoRegFieldBit,
  Self: WWRegFieldBitBand<Srt>,
  Self::Reg: SWoRegBitBand,
{
}

impl<R> SWoWoRegFieldBitBand for R
where
  R: SWoWoRegFieldBit,
  R: WWRegFieldBitBand<Srt>,
  R::Reg: SWoRegBitBand,
{
}

// }}}
// {{{ SRoRwRegFieldBitBand
/// Synchronized one-bit read-only field of read-write register token.
pub trait SRoRwRegFieldBitBand
where
  Self: SRoRwRegFieldBit,
  Self: RRRegFieldBitBand<Srt>,
  Self::Reg: SRwRegBitBand,
{
}

impl<R> SRoRwRegFieldBitBand for R
where
  R: SRoRwRegFieldBit,
  R: RRRegFieldBitBand<Srt>,
  R::Reg: SRwRegBitBand,
{
}

// }}}
// {{{ SRoRoRegFieldBitBand
/// Synchronized one-bit read-only field of read-only register token.
pub trait SRoRoRegFieldBitBand
where
  Self: SRoRoRegFieldBit,
  Self: RRRegFieldBitBand<Srt>,
  Self::Reg: SRoRegBitBand,
{
}

impl<R> SRoRoRegFieldBitBand for R
where
  R: SRoRoRegFieldBit,
  R: RRRegFieldBitBand<Srt>,
  R::Reg: SRoRegBitBand,
{
}

// }}}
// {{{ FRwRwRegFieldBitBand
/// Forkable one-bit read-write field of read-write register token.
pub trait FRwRwRegFieldBitBand
where
  Self: FRwRwRegFieldBit,
  Self: RRRegFieldBitBand<Frt>,
  Self: WWRegFieldBitBand<Frt>,
  Self::Reg: FRwRegBitBand,
{
}

impl<R> FRwRwRegFieldBitBand for R
where
  R: FRwRwRegFieldBit,
  R: RRRegFieldBitBand<Frt>,
  R: WWRegFieldBitBand<Frt>,
  R::Reg: FRwRegBitBand,
{
}

// }}}
// {{{ FWoRwRegFieldBitBand
/// Forkable one-bit write-only field of read-write register token.
pub trait FWoRwRegFieldBitBand
where
  Self: FWoRwRegFieldBit,
  Self: WWRegFieldBitBand<Frt>,
  Self::Reg: FRwRegBitBand,
{
}

impl<R> FWoRwRegFieldBitBand for R
where
  R: FWoRwRegFieldBit,
  R: WWRegFieldBitBand<Frt>,
  R::Reg: FRwRegBitBand,
{
}

// }}}
// {{{ FWoWoRegFieldBitBand
/// Forkable one-bit write-only field of write-only register token.
pub trait FWoWoRegFieldBitBand
where
  Self: FWoWoRegFieldBit,
  Self: WWRegFieldBitBand<Frt>,
  Self::Reg: FWoRegBitBand,
{
}

impl<R> FWoWoRegFieldBitBand for R
where
  R: FWoWoRegFieldBit,
  R: WWRegFieldBitBand<Frt>,
  R::Reg: FWoRegBitBand,
{
}

// }}}
// {{{ FRoRwRegFieldBitBand
/// Forkable one-bit read-only field of read-write register token.
pub trait FRoRwRegFieldBitBand
where
  Self: FRoRwRegFieldBit,
  Self: RRRegFieldBitBand<Frt>,
  Self::Reg: FRwRegBitBand,
{
}

impl<R> FRoRwRegFieldBitBand for R
where
  R: FRoRwRegFieldBit,
  R: RRRegFieldBitBand<Frt>,
  R::Reg: FRwRegBitBand,
{
}

// }}}
// {{{ FRoRoRegFieldBitBand
/// Forkable one-bit read-only field of read-only register token.
pub trait FRoRoRegFieldBitBand
where
  Self: FRoRoRegFieldBit,
  Self: RRRegFieldBitBand<Frt>,
  Self::Reg: FRoRegBitBand,
{
}

impl<R> FRoRoRegFieldBitBand for R
where
  R: FRoRoRegFieldBit,
  R: RRRegFieldBitBand<Frt>,
  R::Reg: FRoRegBitBand,
{
}

// }}}
// vim: set fdm=marker fmr={{{,}}} :
