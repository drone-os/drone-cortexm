use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};

use crate::thr::IntToken;

const NVIC_ISER: usize = 0xE000_E100;
const NVIC_ICER: usize = 0xE000_E180;
const NVIC_ISPR: usize = 0xE000_E200;
const NVIC_ICPR: usize = 0xE000_E280;
const NVIC_IABR: usize = 0xE000_E300;
const NVIC_IPR: usize = 0xE000_E400;

macro_rules! nvic_reg {
    ($doc:expr, $name:ident, $base:expr) => {
        #[doc = $doc]
        pub struct $name<T: NvicBlock> {
            _nvic_block: PhantomData<T>,
            inner: u32,
        }

        impl<T: NvicBlock> NvicReg<T> for $name<T> {
            const BASE: usize = $base;

            #[inline]
            fn new(inner: u32) -> Self {
                Self { _nvic_block: PhantomData, inner }
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

nvic_reg!("Interrupt Set-Enable Register.", NvicIser, NVIC_ISER);
nvic_reg!("Interrupt Clear-Enable Register.", NvicIcer, NVIC_ICER);
nvic_reg!("Interrupt Set-Pending Register.", NvicIspr, NVIC_ISPR);
nvic_reg!("Interrupt Clear-Pending Register.", NvicIcpr, NVIC_ICPR);
nvic_reg!("Interrupt Active-Bit Register.", NvicIabr, NVIC_IABR);

macro_rules! nvic_methods {
    (
        $nvic_reg:ident
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
            fn $write_batch<F>(self, f: F)
            where
                F: FnOnce(&mut $nvic_reg<Self::NvicBlock>),
            {
                $nvic_reg::store(f);
            }

            #[doc = $write_doc]
            #[inline]
            fn $write(self, nvic_reg: &mut $nvic_reg<Self::NvicBlock>) {
                nvic_reg.write::<Self>();
            }

            #[doc = $write_one_doc]
            #[inline]
            fn $write_one(self) {
                self.$write_batch(|r| {
                    self.$write(r);
                });
            }
        )*
        $(
            #[doc = $read_batch_doc]
            #[inline]
            fn $read_batch(self) -> $nvic_reg<Self::NvicBlock> {
                $nvic_reg::load()
            }

            #[doc = $read_doc]
            #[inline]
            fn $read(self, nvic_reg: &$nvic_reg<Self::NvicBlock>) -> bool {
                nvic_reg.read::<Self>()
            }

            #[doc = $read_one_doc]
            #[inline]
            fn $read_one(self) -> bool {
                self.$read(&self.$read_batch())
            }
        )*
    }
}

/// NVIC registers block.
pub trait NvicBlock {
    /// The number of NVIC block.
    const BLOCK_NUM: usize;
}

/// NVIC methods for interrupt tokens.
pub trait ThrNvic: IntToken {
    nvic_methods! {
        NvicIser
        {
            "Enables multiple interrupts within the NVIC register.",
            enable_batch,
            "Enables the interrupt.",
            enable,
            "Enables the interrupt within the `nvic_reg`.",
            enable_int,
        }
        {
            "Returns the NVIC register of enabled states.",
            enabled,
            "Returns `true` if the interrupt is enabled.",
            is_enabled,
            "Returns `true` if the interrupt is enabled within the `nvic_reg`.",
            is_int_enabled,
        }
    }
    nvic_methods! {
        NvicIcer
        {
            "Disables multiple interrupts within the NVIC register.",
            disable_batch,
            "Disables the interrupt.",
            disable,
            "Disables the interrupt within the `nvic_reg`.",
            disable_int,
        }
        {}
    }
    nvic_methods! {
        NvicIspr
        {
            "Sets multiple interrupts pending within the NVIC register.",
            set_pending_batch,
            "Sets the interrupt pending.",
            set_pending,
            "Sets the interrupt pending within the `nvic_reg`.",
            set_pending_int,
        }
        {}
    }
    nvic_methods! {
        NvicIcpr
        {
            "Clears multiple interrupts pending state within the NVIC register.",
            clear_pending_batch,
            "Clears the interrupt pending state.",
            clear_pending,
            "Clears the interrupt pending state within the `nvic_reg`.",
            clear_pending_int,
        }
        {
            "Returns the NVIC register of pending states.",
            pending,
            "Returns `true` if the interrupt is pending.",
            is_pending,
            "Returns `true` if the interrupt is pending within the `nvic_reg`.",
            is_int_pending,
        }
    }
    nvic_methods! {
        NvicIabr
        {}
        {
            "Returns the NVIC register of active states.",
            active,
            "Returns `true` if the interrupt is active.",
            is_active,
            "Returns `true` if the interrupt is active within the `nvic_reg`.",
            is_int_active,
        }
    }

    /// Reads the priority of the interrupt.
    #[inline]
    fn priority(self) -> u8 {
        unsafe { read_volatile((NVIC_IPR as *const u8).add(Self::INT_NUM as usize)) }
    }

    /// Writes the priority of the interrupt.
    #[inline]
    fn set_priority(self, priority: u8) {
        unsafe { write_volatile((NVIC_IPR as *mut u8).add(Self::INT_NUM as usize), priority) };
    }
}

trait NvicReg<T: NvicBlock>: Sized {
    const BASE: usize;

    fn new(inner: u32) -> Self;

    fn inner(&self) -> u32;

    fn inner_mut(&mut self) -> &mut u32;

    fn load() -> Self {
        Self::new(unsafe { read_volatile((Self::BASE as *const u32).add(T::BLOCK_NUM)) })
    }

    fn store<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::new(0);
        f(&mut value);
        unsafe { write_volatile((Self::BASE as *mut u32).add(T::BLOCK_NUM), value.inner()) };
    }

    fn read<U: IntToken>(&self) -> bool {
        self.inner() & 1 << block_offset::<U>() != 0
    }

    fn write<U: IntToken>(&mut self) {
        *self.inner_mut() |= 1 << block_offset::<U>();
    }
}

impl<T: IntToken> ThrNvic for T {}

const fn block_offset<T: IntToken>() -> usize {
    T::INT_NUM as usize & 0b1_1111
}
