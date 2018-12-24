use core::sync::atomic::{AtomicUsize, Ordering::*};
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
    #[derive(Clone, Default)]
    $rgc_vis struct $rgc_ident;

    $(
      $vis static $ident: ::core::sync::atomic::AtomicUsize =
        ::core::sync::atomic::AtomicUsize::new(0);

      impl $crate::reg::RegGuardCnt<$bit_guard_res> for $rgc_ident
      where
        $bit_guard_res: $crate::reg::RegGuardRes,
      {
        fn atomic() -> &'static ::core::sync::atomic::AtomicUsize {
          &$ident
        }
      }
    )*
  };
}

/// Register guard driver.
#[must_use]
pub struct RegGuard<T, U>(T, U)
where
  T: RegGuardRes,
  U: RegGuardCnt<T>;

/// Register guard resource.
pub trait RegGuardRes: Clone {
  /// Register.
  type Reg: RwRegAtomic<Crt>;

  /// Register field.
  type Field: WRwRegFieldAtomic<Crt, Reg = Self::Reg>;

  /// Returns a reference to the register field.
  fn field(&self) -> &Self::Field;

  /// Sets on value.
  fn up(&self, val: &mut <Self::Reg as Reg<Crt>>::Val);

  /// Sets off value.
  fn down(&self, val: &mut <Self::Reg as Reg<Crt>>::Val);
}

/// A static counter for [`RegGuard`].
pub trait RegGuardCnt<T>
where
  Self: Sized + Send + Sync + Default + Clone + 'static,
  T: RegGuardRes,
{
  /// Returns a reference to a static atomic counter.
  fn atomic() -> &'static AtomicUsize;
}

impl<T, U> RegGuard<T, U>
where
  T: RegGuardRes,
  U: RegGuardCnt<T>,
{
  /// Enables the resource and returns the new guard.
  pub fn new(res: T) -> Self {
    U::atomic().fetch_add(1, Relaxed);
    res.field().modify(|val| {
      res.up(val);
    });
    RegGuard(res, U::default())
  }
}

impl<T, U> Clone for RegGuard<T, U>
where
  T: RegGuardRes,
  U: RegGuardCnt<T>,
{
  fn clone(&self) -> Self {
    U::atomic().fetch_add(1, Relaxed);
    RegGuard(self.0.clone(), self.1.clone())
  }
}

impl<T, U> Drop for RegGuard<T, U>
where
  T: RegGuardRes,
  U: RegGuardCnt<T>,
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
