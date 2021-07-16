#![cfg_attr(feature = "std", allow(dead_code, unreachable_code))]

use crate::{map::reg::scb, reg::prelude::*};
use drone_core::token::Token;

/// Threads initialization token.
///
/// # Safety
///
/// * Must be defined only once for a particular set of threads.
/// * `ThrTokens` type must contain only thread tokens.
pub unsafe trait ThrsInitToken: Token {
    /// The set of thread tokens.
    type ThrTokens: Token;
}

/// A set of register tokens returned by [`init_extended`].
#[allow(missing_docs)]
pub struct ThrInitExtended {
    pub scb_ccr_bfhfnmign: scb::ccr::Bfhfnmign<Srt>,
    pub scb_ccr_div_0_trp: scb::ccr::Div0Trp<Srt>,
    pub scb_ccr_unalign_trp: scb::ccr::UnalignTrp<Srt>,
    pub scb_ccr_usersetmpend: scb::ccr::Usersetmpend<Srt>,
}

/// Initializes the thread system and returns a set of thread tokens.
///
/// # Examples
///
/// ```no_run
/// # #![feature(const_fn_fn_ptr_basics)]
/// # #![feature(proc_macro_hygiene)]
/// # use drone_core::token::Token;
/// # thr::nvic! {
/// #     thread => pub Thr {};
/// #     local => pub ThrLocal {};
/// #     index => Thrs;
/// #     vtable => Vtable;
/// #     init => ThrsInit;
/// #     threads => {};
/// # }
/// use drone_cortexm::{cortexm_reg_tokens, reg::prelude::*, thr};
///
/// cortexm_reg_tokens! {
///     index => Regs;
///     exclude => {
///         scb_ccr,
///         mpu_type, mpu_ctrl, mpu_rnr, mpu_rbar, mpu_rasr,
///     }
/// }
///
/// fn handler(reg: Regs, thr_init: ThrsInit) {
///     let (thr, extended) = thr::init_extended(thr_init);
///     extended.scb_ccr_div_0_trp.set_bit();
/// }
///
/// # fn main() {
/// #     handler(unsafe { Regs::take() }, unsafe { ThrsInit::take() })
/// # }
/// ```
#[allow(clippy::needless_pass_by_value)]
#[inline]
pub fn init_extended<T: ThrsInitToken>(_token: T) -> (T::ThrTokens, ThrInitExtended) {
    let scb_ccr = unsafe { scb::Ccr::<Srt>::take() };
    #[cfg(not(cortexm_core = "cortexm7_r0p1"))]
    scb_ccr.store(|r| r.set_stkalign().set_nonbasethrdena());
    #[cfg(cortexm_core = "cortexm7_r0p1")]
    scb_ccr.store(|r| r.set_nonbasethrdena());
    let scb::Ccr {
        stkalign,
        bfhfnmign: scb_ccr_bfhfnmign,
        div_0_trp: scb_ccr_div_0_trp,
        unalign_trp: scb_ccr_unalign_trp,
        usersetmpend: scb_ccr_usersetmpend,
        nonbasethrdena,
    } = scb_ccr;
    #[cfg(feature = "memory-protection-unit")]
    unsafe {
        mpu::reset();
    }
    drop(stkalign);
    drop(nonbasethrdena);
    (unsafe { T::ThrTokens::take() }, ThrInitExtended {
        scb_ccr_bfhfnmign,
        scb_ccr_div_0_trp,
        scb_ccr_unalign_trp,
        scb_ccr_usersetmpend,
    })
}

/// Initializes the thread system and returns a set of thread tokens.
///
/// # Examples
///
/// ```no_run
/// # #![feature(const_fn_fn_ptr_basics)]
/// # #![feature(proc_macro_hygiene)]
/// # use drone_core::token::Token;
/// # thr::nvic! {
/// #     thread => pub Thr {};
/// #     local => pub ThrLocal {};
/// #     index => Thrs;
/// #     vtable => Vtable;
/// #     init => ThrsInit;
/// #     threads => {};
/// # }
/// use drone_cortexm::{cortexm_reg_tokens, thr};
///
/// cortexm_reg_tokens! {
///     index => Regs;
///     exclude => {
///         scb_ccr,
///         mpu_type, mpu_ctrl, mpu_rnr, mpu_rbar, mpu_rasr,
///     }
/// }
///
/// fn handler(reg: Regs, thr_init: ThrsInit) {
///     let thr = thr::init(thr_init);
/// }
///
/// # fn main() {
/// #     handler(unsafe { Regs::take() }, unsafe { ThrsInit::take() })
/// # }
/// ```
#[allow(clippy::needless_pass_by_value)]
#[inline]
pub fn init<T: ThrsInitToken>(token: T) -> T::ThrTokens {
    let (thr, _) = init_extended(token);
    thr
}

#[cfg(feature = "memory-protection-unit")]
mod mpu {
    use crate::{map::reg::mpu, reg::prelude::*};
    use drone_core::token::Token;

    static MPU_RESET_TABLE: [u32; 16] = [
        rbar_reset(0),
        0,
        rbar_reset(1),
        0,
        rbar_reset(2),
        0,
        rbar_reset(3),
        0,
        rbar_reset(4),
        0,
        rbar_reset(5),
        0,
        rbar_reset(6),
        0,
        rbar_reset(7),
        0,
    ];

    pub(super) unsafe fn reset() {
        #[cfg(feature = "std")]
        return unimplemented!();
        let mpu_type = unsafe { mpu::Type::<Srt>::take() };
        let mpu_ctrl = unsafe { mpu::Ctrl::<Srt>::take() };
        if mpu_type.load().dregion() == 0 {
            return;
        }
        mpu_ctrl.reset();
        #[cfg(not(feature = "std"))]
        unsafe {
            asm!(
                "ldmia r0!, {{r2, r3, r4, r5, r8, r9, r10, r11}}",
                "stmia r1,  {{r2, r3, r4, r5, r8, r9, r10, r11}}",
                "ldmia r0!, {{r2, r3, r4, r5, r8, r9, r10, r11}}",
                "stmia r1,  {{r2, r3, r4, r5, r8, r9, r10, r11}}",
                inout("r0") MPU_RESET_TABLE.as_ptr() => _,
                in("r1") mpu::Rbar::<Srt>::ADDRESS,
                out("r2") _,
                out("r3") _,
                out("r4") _,
                out("r5") _,
                out("r8") _,
                out("r9") _,
                out("r10") _,
                out("r11") _,
                options(preserves_flags),
            );
        }
    }

    #[allow(clippy::cast_lossless)]
    const fn rbar_reset(region: u8) -> u32 {
        1 << 4 | region as u32 & 0b1111
    }
}
