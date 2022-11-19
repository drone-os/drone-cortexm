#![cfg_attr(feature = "host", allow(dead_code, unreachable_code, unused_variables, unused_imports))]

use crate::map::periph;
use crate::map::reg::scb;
use crate::reg::prelude::*;
use drone_core::token::Token;

/// Threads initialization token.
///
/// # Safety
///
/// * Must be defined only once for a particular set of threads.
/// * `ThrTokens` associated type must contain only thread tokens.
pub unsafe trait ThrsInitToken: Token {
    /// The set of thread tokens.
    type ThrTokens: Token;

    /// Initializes the thread system and returns a set of thread tokens.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use drone_core::token::Token;
    /// # mod thr {
    /// #     drone_cortexm::thr::nvic! {
    /// #         thread => pub Thr {};
    /// #         local => pub Local {};
    /// #         vectors => pub Vectors;
    /// #         index => pub Index;
    /// #         init => pub Init;
    /// #         threads => {};
    /// #     }
    /// # }
    /// use drone_cortexm::map::cortexm_reg_tokens;
    /// use drone_cortexm::map::periph::{Mpu, Thr};
    /// use drone_cortexm::reg::prelude::*;
    /// use drone_cortexm::thr::prelude::*;
    /// use drone_cortexm::thr::ThrInitExtended;
    /// use drone_cortexm::{periph_mpu, periph_thr};
    ///
    /// cortexm_reg_tokens! {
    ///     index => Regs;
    /// }
    ///
    /// fn handler(reg: Regs, thr: thr::Init) {
    ///     let (thr, extended) = thr.init_extended(periph_mpu!(reg), periph_thr!(reg));
    ///     extended.scb_ccr_div_0_trp.set_bit();
    /// }
    ///
    /// fn main() {
    ///     handler(unsafe { Regs::take() }, unsafe { thr::Init::take() });
    /// }
    /// ```
    #[allow(clippy::drop_non_drop)]
    #[inline]
    #[must_use]
    fn init_extended(
        self,
        #[cfg(feature = "memory-protection-unit")] mpu: periph::Mpu,
        thr: periph::Thr,
    ) -> (Self::ThrTokens, ThrInitExtended) {
        let periph::Thr { scb_ccr } = thr;
        scb_ccr.store(|r| r.set_stkalign().set_nonbasethrdena());
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
            mpu::reset(&mpu);
        }
        drop(stkalign);
        drop(nonbasethrdena);
        (unsafe { Self::ThrTokens::take() }, ThrInitExtended {
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
    /// # use drone_core::token::Token;
    /// # mod thr {
    /// #     drone_cortexm::thr::nvic! {
    /// #         thread => pub Thr {};
    /// #         local => pub Local {};
    /// #         vectors => pub Vectors;
    /// #         index => pub Index;
    /// #         init => pub Init;
    /// #         threads => {};
    /// #     }
    /// # }
    /// use drone_cortexm::map::cortexm_reg_tokens;
    /// use drone_cortexm::map::periph::{Mpu, Thr};
    /// use drone_cortexm::thr::prelude::*;
    /// use drone_cortexm::{periph_mpu, periph_thr};
    ///
    /// cortexm_reg_tokens! {
    ///     index => Regs;
    /// }
    ///
    /// fn handler(reg: Regs, thr: thr::Init) {
    ///     let thr = thr.init(periph_mpu!(reg), periph_thr!(reg));
    /// }
    ///
    /// fn main() {
    ///     handler(unsafe { Regs::take() }, unsafe { thr::Init::take() });
    /// }
    /// ```
    #[inline]
    #[must_use]
    fn init(
        self,
        #[cfg(feature = "memory-protection-unit")] mpu: periph::Mpu,
        thr: periph::Thr,
    ) -> Self::ThrTokens {
        let (thr, _) = self.init_extended(
            #[cfg(feature = "memory-protection-unit")]
            mpu,
            thr,
        );
        thr
    }
}

/// A set of register tokens returned by [`ThrsInitToken::init_extended`].
#[allow(missing_docs)]
pub struct ThrInitExtended {
    pub scb_ccr_bfhfnmign: scb::ccr::Bfhfnmign<Srt>,
    pub scb_ccr_div_0_trp: scb::ccr::Div0Trp<Srt>,
    pub scb_ccr_unalign_trp: scb::ccr::UnalignTrp<Srt>,
    pub scb_ccr_usersetmpend: scb::ccr::Usersetmpend<Srt>,
}

#[cfg(feature = "memory-protection-unit")]
mod mpu {
    use crate::map::periph;
    use crate::map::reg::mpu;
    use crate::reg::prelude::*;
    #[cfg(not(feature = "host"))]
    use core::arch::asm;

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

    pub(super) unsafe fn reset(mpu: &periph::Mpu) {
        #[cfg(feature = "host")]
        return unimplemented!();
        if mpu.mpu_type.load().dregion() == 0 {
            return;
        }
        mpu.mpu_ctrl.reset();
        #[cfg(not(feature = "host"))]
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
