use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};
use thr::int::IntBundle;
use thr::prelude::*;

const NVIC_ISER: usize = 0xE000_E100;
const NVIC_ICER: usize = 0xE000_E180;
const NVIC_ISPR: usize = 0xE000_E200;
const NVIC_ICPR: usize = 0xE000_E280;
const NVIC_IABR: usize = 0xE000_E300;
const NVIC_IPR: usize = 0xE000_E400;

macro_rules! nvic_methods {
  (
    $bundle:ident,
    {$(
      $name_store_doc:expr,
      $name_store:ident,
      $name_write_one_doc:expr,
      $name_write_one:ident,
      $name_write_doc:expr,
      $name_write:ident,
    )*}
    {$(
      $name_load_doc:expr,
      $name_load:ident,
      $name_load_one_doc:expr,
      $name_load_one:ident,
      $name_read_doc:expr,
      $name_read:ident,
    )*}
  ) => {
    $(
      #[doc = $name_store_doc]
      #[inline(always)]
      fn $name_store<F>(&self, f: F)
      where
        F: FnOnce(&mut $bundle<Self::Bundle>),
      {
        $bundle::store(f);
      }

      #[doc = $name_write_doc]
      #[inline(always)]
      fn $name_write(&self, bundle: &mut $bundle<Self::Bundle>) {
        bundle.write::<Self>();
      }

      #[doc = $name_write_one_doc]
      #[inline(always)]
      fn $name_write_one(&self) {
        self.$name_store(|r| {
          self.$name_write(r);
        });
      }
    )*

    $(
      #[doc = $name_load_doc]
      #[inline(always)]
      fn $name_load(&self) -> $bundle<Self::Bundle> {
        $bundle::load()
      }

      #[doc = $name_read_doc]
      #[inline(always)]
      fn $name_read(&self, bundle: &$bundle<Self::Bundle>) -> bool {
        bundle.read::<Self>()
      }

      #[doc = $name_load_one_doc]
      #[inline(always)]
      fn $name_load_one(&self) -> bool {
        self.$name_read(&self.$name_load())
      }
    )*
  }
}

/// NVIC thread control.
pub trait ThrControl: IntToken<Ctt> {
  nvic_methods! {
    NvicIser,
    {
      "Enables multiple interrupts in a batch.",
      enable_batch,
      "Enables the interrupt.",
      enable_int,
      "Enables the interrupt.",
      enable,
    }
    {
      "Returns enabled state of multiple interrupts.",
      enabled,
      "Returns `true` if the interrupt is enabled.",
      is_int_enabled,
      "Returns `true` if the interrupt is enabled.",
      is_enabled,
    }
  }

  nvic_methods! {
    NvicIcer,
    {
      "Disables multiple interrupts in a batch.",
      disable_batch,
      "Disables the interrupt.",
      disable_int,
      "Disables the interrupt.",
      disable,
    }
    {}
  }

  nvic_methods! {
    NvicIspr,
    {
      "Sets multiple interrupts pending in a batch.",
      set_pending_batch,
      "Sets the interrupt pending.",
      set_pending_int,
      "Sets the interrupt pending.",
      set_pending,
    }
    {}
  }

  nvic_methods! {
    NvicIcpr,
    {
      "Clears multiple interrupts pending state in a batch.",
      clear_pending_batch,
      "Clears the interrupt pending state.",
      clear_pending_int,
      "Clears the interrupt pending state.",
      clear_pending,
    }
    {
      "Returns pending state of multiple interrupts.",
      pending,
      "Returns `true` if the interrupt is pending.",
      is_int_pending,
      "Returns `true` if the interrupt is pending.",
      is_pending,
    }
  }

  nvic_methods! {
    NvicIabr,
    {}
    {
      "Returns active state of multiple interrupts.",
      active,
      "Returns `true` if the interrupt is active.",
      is_int_active,
      "Returns `true` if the interrupt is active.",
      is_active,
    }
  }

  /// Returns the interrupt priority.
  #[inline(always)]
  fn priority(&self) -> u8 {
    unsafe { read_volatile((NVIC_IPR as *const u8).add(Self::INT_NUM)) }
  }

  /// Sets the interrupt priority.
  #[inline(always)]
  fn set_priority(&self, priority: u8) {
    unsafe {
      write_volatile((NVIC_IPR as *mut u8).add(Self::INT_NUM), priority);
    }
  }
}

impl<T: IntToken<Ctt>> ThrControl for T {}

trait NvicBundle<T: IntBundle>: Sized {
  const BASE: usize;

  fn new(inner: u32) -> Self;

  fn inner(&self) -> u32;

  fn inner_mut(&mut self) -> &mut u32;

  #[inline(always)]
  fn load() -> Self {
    Self::new(unsafe {
      read_volatile((Self::BASE as *const u32).add(T::BUNDLE_NUM))
    })
  }

  #[inline(always)]
  fn store<F>(f: F)
  where
    F: FnOnce(&mut Self),
  {
    let mut value = Self::new(0);
    f(&mut value);
    unsafe {
      write_volatile(
        (Self::BASE as *mut u32).add(T::BUNDLE_NUM),
        value.inner(),
      );
    }
  }

  #[inline(always)]
  fn read<U: IntToken<Ctt>>(&self) -> bool {
    self.inner() & 1 << bundle_offset::<U>() != 0
  }

  #[inline(always)]
  fn write<U: IntToken<Ctt>>(&mut self) {
    *self.inner_mut() |= 1 << bundle_offset::<U>();
  }
}

#[inline(always)]
const fn bundle_offset<T: IntToken<Ctt>>() -> usize {
  T::INT_NUM & 0b11_111
}

macro_rules! bundle {
  ($doc:expr, $name:ident, $base:expr,) => {
    #[doc = $doc]
    pub struct $name<T: IntBundle> {
      _bundle: PhantomData<T>,
      inner: u32,
    }

    impl<T: IntBundle> NvicBundle<T> for $name<T> {
      const BASE: usize = $base;

      #[inline(always)]
      fn new(inner: u32) -> Self {
        Self {
          _bundle: PhantomData,
          inner,
        }
      }

      #[inline(always)]
      fn inner(&self) -> u32 {
        self.inner
      }

      #[inline(always)]
      fn inner_mut(&mut self) -> &mut u32 {
        &mut self.inner
      }
    }
  };
}

bundle! {
  "Interrupt Set-Enable Register.",
  NvicIser,
  NVIC_ISER,
}

bundle! {
  "Interrupt Clear-Enable Register.",
  NvicIcer,
  NVIC_ICER,
}

bundle! {
  "Interrupt Set-Pending Register.",
  NvicIspr,
  NVIC_ISPR,
}

bundle! {
  "Interrupt Clear-Pending Register.",
  NvicIcpr,
  NVIC_ICPR,
}

bundle! {
  "Interrupt Active-Bit Register.",
  NvicIabr,
  NVIC_IABR,
}
