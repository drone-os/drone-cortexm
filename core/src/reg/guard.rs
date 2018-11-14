use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::*;
use reg::prelude::*;

/// Defines [`RegGuardCnt`] structs.
#[macro_export]
macro_rules! reg_guard_cnt {
  (
    $(#[$rgc_attr:meta])* $rgc_vis:vis struct $rgc_ident:ident;
    $(
      $(#[$attr:meta])* $vis:vis static $ident:ident => $bit_guard_res:ty;
    )*
  ) => {
    $(#[$rgc_attr])*
    #[derive(Default)]
    $rgc_vis struct $rgc_ident;

    $(
      $vis static $ident: ::core::sync::atomic::AtomicUsize =
        ::core::sync::atomic::AtomicUsize::new(0);

      impl<T> $crate::reg::RegGuardCnt<$bit_guard_res, T> for $rgc_ident
      where
        $bit_guard_res: $crate::reg::RegGuardRes<T>,
        T: ::drone_core::reg::RegAtomic,
      {
        fn atomic() -> &'static ::core::sync::atomic::AtomicUsize {
          &$ident
        }
      }
    )*
  };
}

/// Register guard driver.
pub struct RegGuard<T, U, V>(T, U, V)
where
  T: RegGuardRes<V>,
  U: RegGuardCnt<T, V>,
  V: RegAtomic;

/// Register guard resource.
pub trait RegGuardRes<T: RegAtomic>: RegFork {
  /// Register.
  type Reg: RwRegAtomic<T>;

  /// Register field.
  type Field: WRwRegFieldAtomic<T, Reg = Self::Reg>;

  /// Returns a reference to the register field.
  fn field(&self) -> &Self::Field;

  /// Sets on value.
  fn up(&self, val: &mut <Self::Reg as Reg<T>>::Val);

  /// Sets off value.
  fn down(&self, val: &mut <Self::Reg as Reg<T>>::Val);
}

/// A static counter for [`RegGuard`].
pub trait RegGuardCnt<T, U>
where
  Self: Sized + Send + Sync + Default + 'static,
  T: RegGuardRes<U>,
  U: RegAtomic,
{
  /// Returns a reference to a static atomic counter.
  fn atomic() -> &'static AtomicUsize;
}

impl<T, U, V> RegGuard<T, U, V>
where
  T: RegGuardRes<V>,
  U: RegGuardCnt<T, V>,
  V: RegAtomic,
{
  /// Enables the resource and returns the new guard.
  pub fn new(res: T) -> Self {
    U::atomic().fetch_add(1, Relaxed);
    res.field().modify(|val| {
      res.up(val);
    });
    Self(res, U::default(), V::default())
  }
}

impl<T, U, V> RegFork for RegGuard<T, U, V>
where
  T: RegGuardRes<V>,
  U: RegGuardCnt<T, V>,
  V: RegAtomic,
{
  fn fork(&mut self) -> Self {
    U::atomic().fetch_add(1, Relaxed);
    Self(self.0.fork(), U::default(), V::default())
  }
}

impl<T, U, V> Drop for RegGuard<T, U, V>
where
  T: RegGuardRes<V>,
  U: RegGuardCnt<T, V>,
  V: RegAtomic,
{
  fn drop(&mut self) {
    U::atomic().fetch_sub(1, Relaxed);
    self.0.field().modify(|val| {
      if U::atomic().load(Relaxed) == 0 {
        self.0.down(val);
      }
    });
  }
}
