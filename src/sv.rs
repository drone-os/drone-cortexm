//! Supervisor.

mod switch;

pub use self::switch::{Switch, SwitchBackService, SwitchContextService};

use core::mem::size_of;

/// A supervisor interface.
pub trait Supervisor: Sized + 'static {
    /// Returns a pointer to the first service.
    fn first() -> *const Self;
}

/// A supervisor call.
pub trait SvCall<T: SvService>: Supervisor {
    /// Call the system service.
    ///
    /// # Safety
    ///
    /// Directly calling supervisor services is unsafe in general. User code
    /// should use wrappers instead.
    unsafe fn call(service: &mut T);
}

/// A supervisor service.
pub trait SvService: Sized + Send + 'static {
    /// A system service handler.
    ///
    /// # Safety
    ///
    /// Must be called only by supervisor.
    unsafe extern "C" fn handler(&mut self);
}

/// Calls `SVC num` instruction.
///
/// # Safety
///
/// Directly calling supervisor services is unsafe in general. User code should
/// use wrappers instead.
#[cfg_attr(feature = "std", allow(unused_variables))]
#[inline(always)]
pub unsafe fn sv_call<T: SvService>(service: &mut T, num: u8) {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    {
        if size_of::<T>() == 0 {
            asm!("
                svc $0
            "   :
                : "i"(num)
                :
                : "volatile"
            );
        } else {
            asm!("
                svc $0
            "   :
                : "i"(num), "{r12}"(service)
                :
                : "volatile"
            );
        }
    }
}

/// Dispatches [`SvService::handler`](SvService::handler).
///
/// # Safety
///
/// Must be called only by [`sv_handler`](sv_handler).
pub unsafe extern "C" fn service_handler<T>(mut frame: *mut *mut u8)
where
    T: SvService,
{
    if size_of::<T>() == 0 {
        T::handler(&mut *(frame as *mut T));
    } else {
        frame = frame.add(4); // Stacked R12
        T::handler(&mut *(*frame as *mut T));
    }
}

/// An `SVC` handler.
///
/// # Safety
///
/// Must be called only by hardware.
#[naked]
pub unsafe extern "C" fn sv_handler<T: Supervisor>() {
    #[cfg(feature = "std")]
    unimplemented!();
    #[cfg(not(feature = "std"))]
    {
        use core::intrinsics::unreachable;
        asm!("
            tst lr, #4
            ite eq
            mrseq r0, msp
            mrsne r0, psp
            ldr r1, [r0, #24]
            ldrb r1, [r1, #-2]
            ldr pc, [r2, r1, lsl #2]
        "   :
            : "{r2}"(T::first())
            : "r0", "r1", "cc"
            : "volatile"
        );
        unreachable();
    }
}
