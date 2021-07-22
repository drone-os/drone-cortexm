#![cfg_attr(feature = "std", allow(unreachable_code, unused_variables))]

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
    ///
    /// See also [`modify_reg`](RwRegAtomic::modify_reg).
    fn modify<F>(&'a self, f: F)
    where
        F: for<'b> Fn(
            &'b mut <Self as RegRef<'a, T>>::Hold,
        ) -> &'b mut <Self as RegRef<'a, T>>::Hold;

    /// Reads the value from the register memory, then passes a reference to
    /// this register token and the value to the closure `f`, then writes the
    /// modified value into the register memory.
    ///
    /// See also [`modify`](RwRegAtomic::modify).
    fn modify_reg<F>(&'a self, f: F)
    where
        F: for<'b> Fn(&'b Self, &'b mut Self::Val);
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
            let mut val = unsafe { self.hold(load_excl::<T, Self>()) };
            f(&mut val);
            if unsafe { store_excl::<T, Self>(val.val()) } {
                break;
            }
        }
    }

    #[inline]
    fn modify_reg<F>(&'a self, f: F)
    where
        F: for<'b> Fn(&'b Self, &'b mut Self::Val),
    {
        loop {
            let mut val = unsafe { load_excl::<T, Self>() };
            f(self, &mut val);
            if unsafe { store_excl::<T, Self>(val) } {
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
            let mut val = unsafe { load_excl::<T, Self::Reg>() };
            f(&mut val);
            if unsafe { store_excl::<T, Self::Reg>(val) } {
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
    unsafe { R::val_from(<R::Val as Bitfield>::Bits::load_excl(R::ADDRESS)) }
}

unsafe fn store_excl<T, R>(val: R::Val) -> bool
where
    T: RegAtomic,
    R: Reg<T>,
    <R::Val as Bitfield>::Bits: AtomicBits,
{
    unsafe { val.bits().store_excl(R::ADDRESS) }
}

macro_rules! atomic_bits {
    ($type:ty, $ldrex:expr, $strex:expr) => {
        impl AtomicBits for $type {
            unsafe fn load_excl(address: usize) -> Self {
                #[cfg(feature = "std")]
                return unimplemented!();
                let output: Self;
                #[cfg(not(feature = "std"))]
                unsafe {
                    asm!(
                        $ldrex,
                        address = in(reg) address,
                        output = lateout(reg) output,
                        options(readonly, nostack, preserves_flags),
                    );
                }
                output
            }

            unsafe fn store_excl(self, address: usize) -> bool {
                #[cfg(feature = "std")]
                return unimplemented!();
                let status: Self;
                #[cfg(not(feature = "std"))]
                unsafe {
                    asm!(
                        $strex,
                        input = in(reg) self,
                        address = in(reg) address,
                        status = out(reg) status,
                        options(nostack, preserves_flags),
                    );
                }
                status == 0
            }
        }
    };
}

atomic_bits!(u32, "ldrex {output}, [{address}]", "strex {status}, {input}, [{address}]");
atomic_bits!(u16, "ldrexh {output}, [{address}]", "strexh {status}, {input}, [{address}]");
atomic_bits!(u8, "ldrexb {output}, [{address}]", "strexb {status}, {input}, [{address}]");
