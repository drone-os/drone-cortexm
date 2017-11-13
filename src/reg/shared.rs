use core::ops::{Deref, DerefMut};
use drone::reg::{RegFieldRegHoldVal, RegHoldVal, RegRaw};
use drone::reg::prelude::*;

/// A wrapper for a value loaded with `ldrex` instruction.
pub struct RegExcl<T> {
  inner: T,
}

/// Raw shared register value type.
pub trait RegRawShared<T, U>
where
  Self: Sized,
{
  /// Loads the value with `ldrex` instruction.
  unsafe fn load<F>(address: usize, f: F) -> Self
  where
    F: FnOnce(U) -> T;

  /// Stores the value with `strex` instruction.
  unsafe fn store<F>(self, address: usize, f: F) -> bool
  where
    F: FnOnce(Self) -> U;
}

/// Register that can read and write its value in a multi-threaded context.
pub trait RwRegShared<'a, T>
where
  Self: RReg<'a, T> + WReg<'a, T>,
  T: RegShared + 'a,
{
  /// Loads the value with `ldrex` instruction.
  fn load_excl(&'a self) -> RegExcl<Self::Hold>;

  /// Stores `val` with `strex` instruction.
  fn store_excl(&self, val: RegExcl<Self::Hold>) -> bool;

  /// Atomically updates a register's value.
  fn update<F>(&'a self, f: F)
  where
    F: Fn(&mut Self::Hold) -> &mut Self::Hold;
}

/// Register field that can write to read-write register in multi-threaded
/// context.
pub trait RwRegFieldShared<'a, T>
where
  Self: WRegField<'a, T>,
  Self::Reg: RwRegShared<'a, T>,
  T: RegShared + 'a,
{
  /// Loads the value with `ldrex` instruction.
  fn load_excl(&'a self) -> RegExcl<RegFieldRegHoldVal<'a, T, Self>>;

  /// Stores `val` with `strex` instruction.
  fn store_excl(&self, val: RegExcl<RegFieldRegHoldVal<'a, T, Self>>) -> bool;

  /// Atomically updates a register's value.
  fn update<F>(&'a self, f: F)
  where
    F: Fn(&mut RegFieldRegHoldVal<'a, T, Self>);
}

impl<'a, T, U, V, W, X> RwRegShared<'a, T> for U
where
  T: RegShared + 'a,
  U: RReg<'a, T, Hold = V> + WReg<'a, T, Hold = V>,
  V: RegHold<'a, T, Self, Val = W>,
  W: RegVal<Raw = X>,
  X: RegRaw,
  RegExcl<V>: RegRawShared<V, X>,
{
  #[inline]
  fn load_excl(&'a self) -> RegExcl<Self::Hold> {
    unsafe {
      RegExcl::load(Self::ADDRESS, |raw| {
        self.hold(RegHoldVal::<'a, T, Self>::from_raw(raw))
      })
    }
  }

  #[inline]
  fn store_excl(&self, val: RegExcl<Self::Hold>) -> bool {
    unsafe { val.store(Self::ADDRESS, |val| val.into_inner().val().raw()) }
  }

  #[inline]
  fn update<F>(&'a self, f: F)
  where
    F: Fn(&mut Self::Hold) -> &mut Self::Hold,
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

impl<'a, T, U, V, W, X> RwRegFieldShared<'a, T> for U
where
  T: RegShared + 'a,
  U: WRegField<'a, T>,
  U::Reg: RwRegShared<'a, T, Hold = V>,
  V: RegHold<'a, T, U::Reg, Val = W>,
  W: RegVal<Raw = X>,
  X: RegRaw,
  RegExcl<W>: RegRawShared<W, X>,
{
  #[inline]
  fn load_excl(&'a self) -> RegExcl<RegFieldRegHoldVal<'a, T, Self>> {
    unsafe {
      RegExcl::load(Self::Reg::ADDRESS, |raw| {
        RegFieldRegHoldVal::<'a, T, Self>::from_raw(raw)
      })
    }
  }

  #[inline]
  fn store_excl(&self, val: RegExcl<RegFieldRegHoldVal<'a, T, Self>>) -> bool {
    unsafe { val.store(Self::Reg::ADDRESS, |val| val.into_inner().raw()) }
  }

  #[inline]
  fn update<F>(&'a self, f: F)
  where
    F: Fn(&mut RegFieldRegHoldVal<'a, T, Self>),
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

impl<T> Deref for RegExcl<T> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &T {
    &self.inner
  }
}

impl<T> DerefMut for RegExcl<T> {
  #[inline]
  fn deref_mut(&mut self) -> &mut T {
    &mut self.inner
  }
}

impl<T> RegExcl<T> {
  #[inline]
  fn into_inner(self) -> T {
    self.inner
  }
}

pub macro impl_reg_raw_shared($type:ty, $ldrex:expr, $strex:expr) {
  impl<T> RegRawShared<T, $type> for RegExcl<T> {
    #[inline]
    unsafe fn load<F>(address: usize, f: F) -> Self
    where
      F: FnOnce($type) -> T,
    {
      let raw: $type;
      asm!($ldrex
        : "=r"(raw)
        : "r"(address)
        :
        : "volatile");
      Self { inner: f(raw) }
    }

    #[inline]
    unsafe fn store<F>(self, address: usize, f: F) -> bool
    where
      F: FnOnce(Self) -> $type,
    {
      let status: $type;
      asm!($strex
        : "=r"(status)
        : "r"(f(self)), "r"(address)
        :
        : "volatile");
      status == 0
    }
  }
}

impl_reg_raw_shared!(u32, "ldrex $0, [$1]", "strex $0, $1, [$2]");
impl_reg_raw_shared!(u16, "ldrexh $0, [$1]", "strexh $0, $1, [$2]");
impl_reg_raw_shared!(u8, "ldrexb $0, [$1]", "strexb $0, $1, [$2]");
