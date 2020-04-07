#![cfg_attr(feature = "std", allow(unreachable_code, unused_mut))]

use crate::{
    map::reg::{mpu, scb},
    reg::prelude::*,
    thr::ThrTokens,
};
use drone_core::token::Token;

/// Threads initialization token.
///
/// # Safety
///
/// Must be defined only once for a particular set of threads.
pub unsafe trait ThrsInitToken: Token {
    /// The set of thread tokens.
    type ThrTokens: ThrTokens;
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
/// # #![feature(const_fn)]
/// # #![feature(proc_macro_hygiene)]
/// # use drone_core::token::Token;
/// # drone_cortex_m::thr::vtable! {
/// #     use Thr;
/// #     struct Vtable;
/// #     struct Handlers;
/// #     struct Thrs;
/// #     struct ThrsInit;
/// #     static THREADS;
/// # }
/// # drone_cortex_m::thr! {
/// #     use THREADS;
/// #     struct Thr {}
/// #     struct ThrLocal {}
/// # }
/// # drone_cortex_m::cortex_m_reg_tokens! {
/// #     struct Regs;
/// #     !scb_ccr;
/// #     !mpu_type; !mpu_ctrl; !mpu_rnr; !mpu_rbar; !mpu_rasr;
/// # }
/// # fn main() {
/// # let reg = unsafe { Regs::take() };
/// # let thr_init = unsafe { ThrsInit::take() };
/// use drone_cortex_m::{reg::prelude::*, thr};
///
/// let (thr, extended) = thr::init_extended(thr_init);
/// extended.scb_ccr_div_0_trp.set_bit();
/// # }
/// ```
#[allow(clippy::needless_pass_by_value)]
#[inline]
pub fn init_extended<T: ThrsInitToken>(_token: T) -> (T::ThrTokens, ThrInitExtended) {
    let scb_ccr = unsafe { scb::Ccr::<Srt>::take() };
    scb_ccr.store(|r| r.set_stkalign().set_nonbasethrdena());
    let scb::Ccr {
        stkalign,
        bfhfnmign: scb_ccr_bfhfnmign,
        div_0_trp: scb_ccr_div_0_trp,
        unalign_trp: scb_ccr_unalign_trp,
        usersetmpend: scb_ccr_usersetmpend,
        nonbasethrdena,
    } = scb_ccr;
    unsafe {
        mpu_reset();
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
/// # #![feature(const_fn)]
/// # #![feature(proc_macro_hygiene)]
/// # use drone_core::token::Token;
/// # drone_cortex_m::thr::vtable! {
/// #     use Thr;
/// #     struct Vtable;
/// #     struct Handlers;
/// #     struct Thrs;
/// #     struct ThrsInit;
/// #     static THREADS;
/// # }
/// # drone_cortex_m::thr! {
/// #     use THREADS;
/// #     struct Thr {}
/// #     struct ThrLocal {}
/// # }
/// # drone_cortex_m::cortex_m_reg_tokens! {
/// #     struct Regs;
/// #     !scb_ccr;
/// #     !mpu_type; !mpu_ctrl; !mpu_rnr; !mpu_rbar; !mpu_rasr;
/// # }
/// # fn main() {
/// # let reg = unsafe { Regs::take() };
/// # let thr_init = unsafe { ThrsInit::take() };
/// use drone_cortex_m::thr;
///
/// let thr = thr::init(thr_init);
/// # }
/// ```
#[allow(clippy::needless_pass_by_value)]
#[inline]
pub fn init<T: ThrsInitToken>(token: T) -> T::ThrTokens {
    let (thr, _) = init_extended(token);
    thr
}

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

#[allow(unused_assignments, unused_variables)]
unsafe fn mpu_reset() {
    #[cfg(feature = "std")]
    return unimplemented!();
    let mpu_type = mpu::Type::<Srt>::take();
    let mpu_ctrl = mpu::Ctrl::<Srt>::take();
    let mut table_ptr = &MPU_RESET_TABLE;
    if mpu_type.load().dregion() == 0 {
        return;
    }
    mpu_ctrl.reset();
    asm!("
        ldmia $0!, {r5, r6, r8, r9, r10, r11, r12, r14}
        stmia $1, {r5, r6, r8, r9, r10, r11, r12, r14}
        ldmia $0!, {r5, r6, r8, r9, r10, r11, r12, r14}
        stmia $1, {r5, r6, r8, r9, r10, r11, r12, r14}
    "   : "+&rm"(table_ptr)
        : "r"(mpu::Rbar::<Srt>::ADDRESS)
        : "r5", "r6", "r8", "r9", "r10", "r11", "r12", "r14"
        : "volatile"
    );
}

#[allow(clippy::cast_lossless)]
const fn rbar_reset(region: u8) -> u32 {
    1 << 4 | region as u32 & 0b1111
}
