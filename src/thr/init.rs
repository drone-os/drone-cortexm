#![cfg_attr(feature = "std", allow(unreachable_code, unused_mut))]

use crate::{
    map::{
        periph::{mpu::MpuPeriph, thr::ThrPeriph},
        reg::{mpu, scb},
    },
    reg::prelude::*,
    thr::ThrTokens,
};

/// A set of register tokens returned by `thr::init!` macro.
#[allow(missing_docs)]
pub struct ThrInitPeriph {
    pub scb_ccr_bfhfnmign: scb::ccr::Bfhfnmign<Srt>,
    pub scb_ccr_div_0_trp: scb::ccr::Div0Trp<Srt>,
    pub scb_ccr_unalign_trp: scb::ccr::UnalignTrp<Srt>,
    pub scb_ccr_usersetmpend: scb::ccr::Usersetmpend<Srt>,
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

#[doc(hidden)]
#[macro_export]
macro_rules! thr_init {
    ($reg:ident, $thr_tokens:ident) => {
        $crate::thr::init::<$thr_tokens>(
            $crate::map::periph::mpu::periph_mpu!($reg),
            $crate::map::periph::thr::periph_thr!($reg),
        )
    };
}

#[doc(hidden)]
#[inline]
pub fn init<T: ThrTokens>(mpu: MpuPeriph, thr: ThrPeriph) -> (T, ThrInitPeriph) {
    let ThrPeriph { scb_ccr } = thr;
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
        mpu_reset(&mpu);
        drop(mpu);
        drop(stkalign);
        drop(nonbasethrdena);
        (T::take(), ThrInitPeriph {
            scb_ccr_bfhfnmign,
            scb_ccr_div_0_trp,
            scb_ccr_unalign_trp,
            scb_ccr_usersetmpend,
        })
    }
}

#[allow(unused_assignments, unused_variables)]
unsafe fn mpu_reset(mpu: &MpuPeriph) {
    #[cfg(feature = "std")]
    return unimplemented!();
    let mut table_ptr = &MPU_RESET_TABLE;
    if mpu.mpu_type.load().dregion() == 0 {
        return;
    }
    mpu.mpu_ctrl.reset();
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
