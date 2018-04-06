use core::ops::{Deref, DerefMut};
use drone_core::bitfield::Bitfield;
use drone_core::reg::prelude::*;

/// A wrapper for a value loaded with `ldrex` instruction.
pub struct RegExcl<T> {
  inner: T,
}

/// `RegExl` for `RegHold`.
pub trait RegHoldExcl<T: RegAtomic, U: Reg<T>>: Sized {
  /// Downgrades to the underlying value.
  fn val(self) -> RegExcl<U::Val>;
}

/// Raw shared register value type.
pub trait AtomicBits: Sized {
  /// Loads the value with `ldrex` instruction.
  unsafe fn load_excl(address: usize) -> Self;

  /// Stores the value with `strex` instruction.
  unsafe fn store_excl(self, address: usize) -> bool;
}

/// Register that can read and write its value in a multi-threaded context.
pub trait RwRegAtomic<T: RegAtomic>: RReg<T> + WReg<T> {
  /// Loads the value with `ldrex` instruction.
  fn load_excl<'a>(&'a self) -> RegExcl<<Self as RegRef<'a, T>>::Hold>
  where
    Self: RegRef<'a, T>;

  /// Stores `val` with `strex` instruction.
  fn store_excl(&self, val: RegExcl<Self::Val>) -> bool;
}

/// Register that can update its value in a multi-threaded context.
// FIXME https://github.com/rust-lang/rust/issues/46397
pub trait RwRegAtomicRef<'a, T: RegAtomic>
where
  Self: RwRegAtomic<T> + WRegAtomic<'a, T> + RegRef<'a, T>,
{
  /// Atomically updates the register's value.
  fn modify<F>(&'a self, f: F)
  where
    F: for<'b> Fn(&'b mut <Self as RegRef<'a, T>>::Hold)
      -> &'b mut <Self as RegRef<'a, T>>::Hold;
}

/// Write field of shared read-write register.
pub trait WRwRegFieldAtomic<T: RegAtomic>
where
  Self: WWRegField<T>,
  Self::Reg: RwRegAtomic<T>,
{
  /// Loads the value with `ldrex` instruction.
  fn load_excl(&self) -> RegExcl<<Self::Reg as Reg<T>>::Val>;

  /// Stores `val` with `strex` instruction.
  fn store_excl(&self, val: RegExcl<<Self::Reg as Reg<T>>::Val>) -> bool;

  /// Atomically updates a register's value.
  fn modify<F>(&self, f: F)
  where
    F: Fn(&mut <Self::Reg as Reg<T>>::Val);
}

/// Single-bit write field of shared read-write register.
pub trait WRwRegFieldBitAtomic<T: RegAtomic>
where
  Self: WRwRegFieldAtomic<T> + RegFieldBit<T>,
  Self::Reg: RwRegAtomic<T>,
{
  /// Sets the bit in memory.
  fn set_bit(&self);

  /// Clears the bit in memory.
  fn clear_bit(&self);

  /// Toggles the bit in memory.
  fn toggle_bit(&self);
}

/// Multiple-bits write field of shared read-write register.
pub trait WRwRegFieldBitsAtomic<T: RegAtomic>
where
  Self: WRwRegFieldAtomic<T> + RegFieldBits<T>,
  Self::Reg: RwRegAtomic<T>,
{
  /// Sets the bit in memory.
  fn write_bits(&self, bits: <<Self::Reg as Reg<T>>::Val as Bitfield>::Bits);
}

impl<T, U> RwRegAtomic<T> for U
where
  T: RegAtomic,
  U: RReg<T> + WReg<T>,
  <U::Val as Bitfield>::Bits: AtomicBits,
{
  #[inline(always)]
  fn load_excl<'a>(&'a self) -> RegExcl<<Self as RegRef<'a, T>>::Hold>
  where
    Self: RegRef<'a, T>,
  {
    unsafe {
      RegExcl::new(self.hold(U::Val::from_bits(
        <U::Val as Bitfield>::Bits::load_excl(Self::ADDRESS),
      )))
    }
  }

  #[inline(always)]
  fn store_excl(&self, val: RegExcl<U::Val>) -> bool {
    unsafe {
      val
        .into_inner()
        .bits()
        .store_excl(Self::ADDRESS)
    }
  }
}

impl<'a, T, U> RwRegAtomicRef<'a, T> for U
where
  T: RegAtomic,
  U: RReg<T> + WRegAtomic<'a, T> + RegRef<'a, T>,
  <U::Val as Bitfield>::Bits: AtomicBits,
{
  #[inline(always)]
  fn modify<F>(&'a self, f: F)
  where
    F: for<'b> Fn(&'b mut <U as RegRef<'a, T>>::Hold)
      -> &'b mut <U as RegRef<'a, T>>::Hold,
  {
    loop {
      let mut val = self.load_excl();
      f(&mut val);
      if self.store_excl(val.val()) {
        break;
      }
    }
  }
}

impl<T, U> WRwRegFieldAtomic<T> for U
where
  T: RegAtomic,
  U: WWRegField<T>,
  U::Reg: RwRegAtomic<T>,
  <<U::Reg as Reg<T>>::Val as Bitfield>::Bits: AtomicBits,
{
  #[inline(always)]
  fn load_excl(&self) -> RegExcl<<U::Reg as Reg<T>>::Val> {
    unsafe {
      RegExcl::new(<U::Reg as Reg<T>>::Val::from_bits(
        <<U::Reg as Reg<T>>::Val as Bitfield>::Bits::load_excl(
          Self::Reg::ADDRESS,
        ),
      ))
    }
  }

  #[inline(always)]
  fn store_excl(&self, val: RegExcl<<U::Reg as Reg<T>>::Val>) -> bool {
    unsafe {
      val
        .into_inner()
        .bits()
        .store_excl(Self::Reg::ADDRESS)
    }
  }

  #[inline(always)]
  fn modify<F>(&self, f: F)
  where
    F: Fn(&mut <U::Reg as Reg<T>>::Val),
  {
    loop {
      let mut val = self.load_excl();
      f(&mut val);
      if self.store_excl(val) {
        break;
      }
    }
  }
}

impl<T, U> WRwRegFieldBitAtomic<T> for U
where
  T: RegAtomic,
  U: WRwRegFieldAtomic<T> + RegFieldBit<T>,
  U::Reg: RwRegAtomic<T>,
{
  #[inline(always)]
  fn set_bit(&self) {
    self.modify(|val| {
      self.set(val);
    });
  }

  #[inline(always)]
  fn clear_bit(&self) {
    self.modify(|val| {
      self.clear(val);
    });
  }

  #[inline(always)]
  fn toggle_bit(&self) {
    self.modify(|val| {
      self.toggle(val);
    });
  }
}

impl<T, U> WRwRegFieldBitsAtomic<T> for U
where
  T: RegAtomic,
  U: WRwRegFieldAtomic<T> + RegFieldBits<T>,
  U::Reg: RwRegAtomic<T>,
{
  #[inline(always)]
  fn write_bits(&self, bits: <<U::Reg as Reg<T>>::Val as Bitfield>::Bits) {
    self.modify(|val| {
      self.write(val, bits);
    });
  }
}

impl<T> Deref for RegExcl<T> {
  type Target = T;

  #[inline(always)]
  fn deref(&self) -> &T {
    &self.inner
  }
}

impl<T> DerefMut for RegExcl<T> {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut T {
    &mut self.inner
  }
}

impl<T> RegExcl<T> {
  #[inline(always)]
  fn new(inner: T) -> Self {
    Self { inner }
  }

  #[inline(always)]
  fn into_inner(self) -> T {
    self.inner
  }
}

impl<'a, T, U, V> RegHoldExcl<T, U> for RegExcl<V>
where
  T: RegAtomic,
  U: Reg<T>,
  V: RegHold<'a, T, U>,
{
  fn val(self) -> RegExcl<U::Val> {
    RegExcl::new(self.into_inner().val())
  }
}

macro_rules! atomic_bits {
  ($type:ty, $ldrex:expr, $strex:expr) => {
    impl AtomicBits for $type {
      #[inline(always)]
      unsafe fn load_excl(address: usize) -> Self {
        let raw: $type;
        asm!($ldrex
             : "=r"(raw)
             : "r"(address)
             :
             : "volatile");
        raw
      }

      #[inline(always)]
      unsafe fn store_excl(self, address: usize) -> bool {
        let status: $type;
        asm!($strex
             : "=r"(status)
             : "r"(self), "r"(address)
             :
             : "volatile");
        status == 0
      }
    }
  };
}

atomic_bits!(u32, "ldrex $0, [$1]", "strex $0, $1, [$2]");
atomic_bits!(u16, "ldrexh $0, [$1]", "strexh $0, $1, [$2]");
atomic_bits!(u8, "ldrexb $0, [$1]", "strexb $0, $1, [$2]");
