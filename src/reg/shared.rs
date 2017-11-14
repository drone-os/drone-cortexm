use core::ops::{Deref, DerefMut};
use drone::reg::{RegFieldRegHoldVal, RegFieldRegHoldValRaw, RegHoldVal,
                 RegHoldValRaw, RegRaw};
use drone::reg::prelude::*;

/// A wrapper for a value loaded with `ldrex` instruction.
pub struct RegExcl<T> {
  inner: T,
}

/// Raw shared register value type.
pub trait RegRawShared
where
  Self: Sized,
{
  /// Loads the value with `ldrex` instruction.
  unsafe fn load_excl(address: usize) -> Self;

  /// Stores the value with `strex` instruction.
  unsafe fn store_excl(self, address: usize) -> bool;
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

/// Write-only field of write-only register.
pub trait WRwRegFieldShared<'a, T>
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
  U: Reg<'a, T, Hold = V> + RReg<'a, T> + WReg<'a, T>,
  V: RegHold<'a, T, Self, Val = W>,
  W: RegVal<Raw = X>,
  X: RegRaw + RegRawShared,
{
  #[inline(always)]
  fn load_excl(&'a self) -> RegExcl<Self::Hold> {
    unsafe {
      RegExcl::new(self.hold(RegHoldVal::<'a, T, Self>::from_raw(
        RegHoldValRaw::<'a, T, Self>::load_excl(Self::ADDRESS),
      )))
    }
  }

  #[inline(always)]
  fn store_excl(&self, val: RegExcl<Self::Hold>) -> bool {
    unsafe { val.into_inner().val().raw().store_excl(Self::ADDRESS) }
  }

  #[inline(always)]
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

impl<'a, T, U, V, W, X> WRwRegFieldShared<'a, T> for U
where
  T: RegShared + 'a,
  U: WRegField<'a, T>,
  U::Reg: Reg<'a, T, Hold = V> + RwRegShared<'a, T>,
  V: RegHold<'a, T, U::Reg, Val = W>,
  W: RegVal<Raw = X>,
  X: RegRaw + RegRawShared,
{
  #[inline(always)]
  fn load_excl(&'a self) -> RegExcl<RegFieldRegHoldVal<'a, T, Self>> {
    unsafe {
      RegExcl::new(RegFieldRegHoldVal::<'a, T, Self>::from_raw(
        RegFieldRegHoldValRaw::<'a, T, Self>::load_excl(Self::Reg::ADDRESS),
      ))
    }
  }

  #[inline(always)]
  fn store_excl(&self, val: RegExcl<RegFieldRegHoldVal<'a, T, Self>>) -> bool {
    unsafe { val.into_inner().raw().store_excl(Self::Reg::ADDRESS) }
  }

  #[inline(always)]
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

macro impl_reg_raw_shared($type:ty, $ldrex:expr, $strex:expr) {
  impl RegRawShared for $type {
    #[inline(always)]
    unsafe fn load_excl(address: usize) -> Self
    {
      let raw: $type;
      asm!($ldrex
        : "=r"(raw)
        : "r"(address)
        :
        : "volatile");
      raw
    }

    #[inline(always)]
    unsafe fn store_excl(self, address: usize) -> bool
    {
      let status: $type;
      asm!($strex
        : "=r"(status)
        : "r"(self), "r"(address)
        :
        : "volatile");
      status == 0
    }
  }
}

impl_reg_raw_shared!(u32, "ldrex $0, [$1]", "strex $0, $1, [$2]");
impl_reg_raw_shared!(u16, "ldrexh $0, [$1]", "strexh $0, $1, [$2]");
impl_reg_raw_shared!(u8, "ldrexb $0, [$1]", "strexb $0, $1, [$2]");
