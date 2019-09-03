use crate::thr::{IntBundle, IntToken};
use core::{
    marker::PhantomData,
    ptr::{read_volatile, write_volatile},
};

const NVIC_ISER: usize = 0xE000_E100;
const NVIC_ICER: usize = 0xE000_E180;
const NVIC_ISPR: usize = 0xE000_E200;
const NVIC_ICPR: usize = 0xE000_E280;
const NVIC_IABR: usize = 0xE000_E300;
const NVIC_IPR: usize = 0xE000_E400;

macro_rules! bundle {
    ($doc:expr, $name:ident, $base:expr) => {
        #[doc = $doc]
        pub struct $name<T: IntBundle> {
            _bundle: PhantomData<T>,
            inner: u32,
        }

        impl<T: IntBundle> NvicBundle<T> for $name<T> {
            const BASE: usize = $base;

            #[inline]
            fn new(inner: u32) -> Self {
                Self {
                    _bundle: PhantomData,
                    inner,
                }
            }

            #[inline]
            fn inner(&self) -> u32 {
                self.inner
            }

            #[inline]
            fn inner_mut(&mut self) -> &mut u32 {
                &mut self.inner
            }
        }
    };
}

bundle!("Interrupt Set-Enable Register.", NvicIser, NVIC_ISER);
bundle!("Interrupt Clear-Enable Register.", NvicIcer, NVIC_ICER);
bundle!("Interrupt Set-Pending Register.", NvicIspr, NVIC_ISPR);
bundle!("Interrupt Clear-Pending Register.", NvicIcpr, NVIC_ICPR);
bundle!("Interrupt Active-Bit Register.", NvicIabr, NVIC_IABR);

macro_rules! nvic_methods {
    (
        $bundle:ident
        {$(
            $write_batch_doc:expr,
            $write_batch:ident,
            $write_doc:expr,
            $write:ident,
            $write_one_doc:expr,
            $write_one:ident,
        )?}
        {$(
            $read_batch_doc:expr,
            $read_batch:ident,
            $read_doc:expr,
            $read:ident,
            $read_one_doc:expr,
            $read_one:ident,
        )?}
    ) => {
        $(
            #[doc = $write_batch_doc]
            #[inline]
            fn $write_batch<F>(&self, f: F)
            where
                F: FnOnce(&mut $bundle<Self::Bundle>),
            {
                $bundle::store(f);
            }

            #[doc = $write_doc]
            #[inline]
            fn $write(&self, bundle: &mut $bundle<Self::Bundle>) {
                bundle.write::<Self>();
            }

            #[doc = $write_one_doc]
            #[inline]
            fn $write_one(&self) {
                self.$write_batch(|r| {
                    self.$write(r);
                });
            }
        )*
        $(
            #[doc = $read_batch_doc]
            #[inline]
            fn $read_batch(&self) -> $bundle<Self::Bundle> {
                $bundle::load()
            }

            #[doc = $read_doc]
            #[inline]
            fn $read(&self, bundle: &$bundle<Self::Bundle>) -> bool {
                bundle.read::<Self>()
            }

            #[doc = $read_one_doc]
            #[inline]
            fn $read_one(&self) -> bool {
                self.$read(&self.$read_batch())
            }
        )*
    }
}

/// NVIC methods for interrupt tokens.
pub trait ThrNvic: IntToken {
    nvic_methods! {
        NvicIser
        {
            "Enables multiple interrupts within the NVIC bundle.",
            enable_batch,
            "Enables the interrupt.",
            enable,
            "Enables the interrupt within the NVIC `bundle`.",
            enable_int,
        }
        {
            "Returns the NVIC bundle of enabled states.",
            enabled,
            "Returns `true` if the interrupt is enabled.",
            is_enabled,
            "Returns `true` if the interrupt is enabled within the NVIC `bundle`.",
            is_int_enabled,
        }
    }
    nvic_methods! {
        NvicIcer
        {
            "Disables multiple interrupts within the NVIC bundle.",
            disable_batch,
            "Disables the interrupt.",
            disable,
            "Disables the interrupt within the NVIC `bundle`.",
            disable_int,
        }
        {}
    }
    nvic_methods! {
        NvicIspr
        {
            "Sets multiple interrupts pending within the NVIC bundle.",
            set_pending_batch,
            "Sets the interrupt pending.",
            set_pending,
            "Sets the interrupt pending within the NVIC `bundle`.",
            set_pending_int,
        }
        {}
    }
    nvic_methods! {
        NvicIcpr
        {
            "Clears multiple interrupts pending state within the NVIC bundle.",
            clear_pending_batch,
            "Clears the interrupt pending state.",
            clear_pending,
            "Clears the interrupt pending state within the NVIC `bundle`.",
            clear_pending_int,
        }
        {
            "Returns the NVIC bundle of pending states.",
            pending,
            "Returns `true` if the interrupt is pending.",
            is_pending,
            "Returns `true` if the interrupt is pending within the NVIC `bundle`.",
            is_int_pending,
        }
    }
    nvic_methods! {
        NvicIabr
        {}
        {
            "Returns the NVIC bundle of active states.",
            active,
            "Returns `true` if the interrupt is active.",
            is_active,
            "Returns `true` if the interrupt is active within the NVIC `bundle`.",
            is_int_active,
        }
    }

    /// Reads the priority of the interrupt.
    #[inline]
    fn priority(&self) -> u8 {
        unsafe { read_volatile((NVIC_IPR as *const u8).add(Self::INT_NUM)) }
    }

    /// Writes the priority of the interrupt.
    #[inline]
    fn set_priority(&self, priority: u8) {
        unsafe { write_volatile((NVIC_IPR as *mut u8).add(Self::INT_NUM), priority) };
    }
}

trait NvicBundle<T: IntBundle>: Sized {
    const BASE: usize;

    fn new(inner: u32) -> Self;

    fn inner(&self) -> u32;

    fn inner_mut(&mut self) -> &mut u32;

    fn load() -> Self {
        Self::new(unsafe { read_volatile((Self::BASE as *const u32).add(T::BUNDLE_NUM)) })
    }

    fn store<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::new(0);
        f(&mut value);
        unsafe {
            write_volatile((Self::BASE as *mut u32).add(T::BUNDLE_NUM), value.inner());
        }
    }

    fn read<U: IntToken>(&self) -> bool {
        self.inner() & 1 << bundle_offset::<U>() != 0
    }

    fn write<U: IntToken>(&mut self) {
        *self.inner_mut() |= 1 << bundle_offset::<U>();
    }
}

impl<T: IntToken> ThrNvic for T {}

const fn bundle_offset<T: IntToken>() -> usize {
    T::INT_NUM & 0b11_111
}
