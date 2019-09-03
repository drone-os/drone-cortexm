#![cfg_attr(feature = "std", allow(unused_variables))]

use crate::reg::{
    field::{RegFieldBit, RegFieldBits, WWRegField, WWRegFieldBit, WWRegFieldBits},
    tag::RegAtomic,
    RReg, Reg, RegHold, RegRef, WReg, WRegAtomic,
};
use drone_core::bitfield::Bitfield;

/// Atomic operations for read-write register.
// FIXME https://github.com/rust-lang/rust/issues/46397
pub trait RwRegAtomic<'a, T: RegAtomic>: RReg<T> + WRegAtomic<'a, T> + RegRef<'a, T> {
    /// Reads the value from the register memory, then passes the value to the
    /// closure `f`, then writes the result of the closure back to the register
    /// memory.
    ///
    /// This operation is atomic, it repeats itself in case it was interrupted
    /// in the middle. Thus the closure `f` may be called multiple times.
    fn modify<F>(&'a self, f: F)
    where
        F: for<'b> Fn(
            &'b mut <Self as RegRef<'a, T>>::Hold,
        ) -> &'b mut <Self as RegRef<'a, T>>::Hold;
}

/// Atomic operations for writable field of read-write register.
pub trait WRwRegFieldAtomic<T: RegAtomic>
where
    Self: WWRegField<T>,
    Self::Reg: RReg<T> + WReg<T>,
{
    /// Reads the value from the register memory, then passes the value to the
    /// closure `f`, then writes the modified value back to the register memory.
    ///
    /// This operation is atomic, it repeats itself in case it was interrupted
    /// in the middle. Thus the closure `f` may be called multiple times.
    fn modify<F>(&self, f: F)
    where
        F: Fn(&mut <Self::Reg as Reg<T>>::Val);
}

/// Atomic operations for writable single-bit field of read-write register.
pub trait WRwRegFieldBitAtomic<T: RegAtomic>
where
    Self: WRwRegFieldAtomic<T> + RegFieldBit<T>,
    Self::Reg: RReg<T> + WReg<T>,
{
    /// Reads the value from the register memory, sets the bit, writes the value
    /// back to the register memory, repeat if interrupted.
    fn set_bit(&self);

    /// Reads the value from the register memory, clears the bit, writes the
    /// value back to the register memory, repeat if interrupted.
    fn clear_bit(&self);

    /// Reads the value from the register memory, toggles the bit, writes the
    /// value back to the register memory, repeat if interrupted.
    fn toggle_bit(&self);
}

/// Atomic operations for writable multiple-bit field of read-write register.
pub trait WRwRegFieldBitsAtomic<T: RegAtomic>
where
    Self: WRwRegFieldAtomic<T> + RegFieldBits<T>,
    Self::Reg: RReg<T> + WReg<T>,
{
    /// Reads the value from the register memory, replaces the field bits by
    /// `bits`, writes the value back to the register memory, repeat if
    /// interrupted.
    fn write_bits(&self, bits: <<Self::Reg as Reg<T>>::Val as Bitfield>::Bits);
}

pub trait AtomicBits: Sized {
    unsafe fn load_excl(address: usize) -> Self;

    unsafe fn store_excl(self, address: usize) -> bool;
}

impl<'a, T, R> RwRegAtomic<'a, T> for R
where
    T: RegAtomic,
    R: RReg<T> + WRegAtomic<'a, T> + RegRef<'a, T>,
    <R::Val as Bitfield>::Bits: AtomicBits,
{
    #[inline]
    fn modify<F>(&'a self, f: F)
    where
        F: for<'b> Fn(
            &'b mut <Self as RegRef<'a, T>>::Hold,
        ) -> &'b mut <Self as RegRef<'a, T>>::Hold,
    {
        loop {
            let mut val = unsafe { self.hold(load_excl::<T, R>()) };
            f(&mut val);
            if unsafe { store_excl::<T, R>(val.val()) } {
                break;
            }
        }
    }
}

impl<T, R> WRwRegFieldAtomic<T> for R
where
    T: RegAtomic,
    R: WWRegField<T>,
    R::Reg: RReg<T> + WReg<T>,
    <<R::Reg as Reg<T>>::Val as Bitfield>::Bits: AtomicBits,
{
    #[inline]
    fn modify<F>(&self, f: F)
    where
        F: Fn(&mut <Self::Reg as Reg<T>>::Val),
    {
        loop {
            let mut val = unsafe { load_excl::<T, R::Reg>() };
            f(&mut val);
            if unsafe { store_excl::<T, R::Reg>(val) } {
                break;
            }
        }
    }
}

impl<T, R> WRwRegFieldBitAtomic<T> for R
where
    T: RegAtomic,
    R: WRwRegFieldAtomic<T> + RegFieldBit<T>,
    R::Reg: RReg<T> + WReg<T>,
{
    #[inline]
    fn set_bit(&self) {
        self.modify(|val| {
            self.set(val);
        });
    }

    #[inline]
    fn clear_bit(&self) {
        self.modify(|val| {
            self.clear(val);
        });
    }

    #[inline]
    fn toggle_bit(&self) {
        self.modify(|val| {
            self.toggle(val);
        });
    }
}

impl<T, R> WRwRegFieldBitsAtomic<T> for R
where
    T: RegAtomic,
    R: WRwRegFieldAtomic<T> + RegFieldBits<T>,
    R::Reg: RReg<T> + WReg<T>,
{
    #[inline]
    fn write_bits(&self, bits: <<Self::Reg as Reg<T>>::Val as Bitfield>::Bits) {
        self.modify(|val| {
            self.write(val, bits);
        });
    }
}

unsafe fn load_excl<T, R>() -> R::Val
where
    T: RegAtomic,
    R: Reg<T>,
    <R::Val as Bitfield>::Bits: AtomicBits,
{
    R::val_from(<R::Val as Bitfield>::Bits::load_excl(R::ADDRESS))
}

unsafe fn store_excl<T, R>(val: R::Val) -> bool
where
    T: RegAtomic,
    R: Reg<T>,
    <R::Val as Bitfield>::Bits: AtomicBits,
{
    val.bits().store_excl(R::ADDRESS)
}

macro_rules! atomic_bits {
    ($type:ty, $ldrex:expr, $strex:expr) => {
        impl AtomicBits for $type {
            unsafe fn load_excl(address: usize) -> Self {
                #[cfg(feature = "std")]
                unimplemented!();
                #[cfg(not(feature = "std"))]
                {
                    let raw: Self;
                    asm!($ldrex
                        : "=r"(raw)
                        : "r"(address)
                        :
                        : "volatile"
                    );
                    raw
                }
            }

            unsafe fn store_excl(self, address: usize) -> bool {
                #[cfg(feature = "std")]
                unimplemented!();
                #[cfg(not(feature = "std"))]
                {
                    let status: Self;
                    asm!($strex
                        : "=r"(status)
                        : "r"(self), "r"(address)
                        :
                        : "volatile"
                    );
                    status == 0
                }
            }
        }
    };
}

atomic_bits!(u32, "ldrex $0, [$1]", "strex $0, $1, [$2]");
atomic_bits!(u16, "ldrexh $0, [$1]", "strexh $0, $1, [$2]");
atomic_bits!(u8, "ldrexb $0, [$1]", "strexb $0, $1, [$2]");
