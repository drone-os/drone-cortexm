use crate::sv::{SvCall, SvService};
use core::mem::size_of;

/// Service to switch to a process stack. See
/// [`Switch::switch_context`](Switch::switch_context).
pub struct SwitchContextService {
    stack_ptr: *mut *const u8,
    data_ptr: *mut u8,
}

/// Service to switch back from a process stack. See
/// [`Switch::switch_back`](Switch::switch_back).
pub struct SwitchBackService {
    data_ptr: *mut *mut u8,
    data_size: usize,
}

/// Context switching.
pub trait Switch<T>
where
    Self: SvCall<SwitchContextService>,
    Self: SvCall<SwitchBackService>,
{
    /// Switches to the given process stack.
    ///
    /// # Safety
    ///
    /// * `data` must be word-aligned.
    /// * `*stack_ptr` must be word-aligned.
    unsafe fn switch_context(data: *mut T, stack_ptr: *mut *const u8);

    /// Switches to the previous stack.
    ///
    /// # Safety
    ///
    /// * Must be called only from Process Stack.
    /// * `T` must match with paired [`switch_context`](Switch::switch_context).
    /// * `*data` must be word-aligned.
    unsafe fn switch_back(data: *mut *mut T);
}

unsafe impl Send for SwitchContextService {}
unsafe impl Send for SwitchBackService {}

impl SvService for SwitchContextService {
    #[cfg_attr(feature = "std", allow(unused_variables))]
    unsafe extern "C" fn handler(&mut self) {
        let Self {
            stack_ptr,
            data_ptr,
        } = *self;
        #[cfg(feature = "std")]
        unimplemented!();
        #[cfg(not(feature = "std"))]
        {
            use core::intrinsics::unreachable;
            #[cfg(feature = "fpu")]
            asm!("
                mrs      r3, control
                tst      lr, #0x4
                bne      3f
                tst      lr, #0x10
                it       eq
                vstmdbeq sp!, {s16-s31}
                stmdb    sp!, {r3, r4-r11}
            0:
                ldr      r2, [r0]
                ldmia    r2!, {r3}
                push     {r0, r1, r3, lr}
                cmp      r3, #0
                bne      2f
            1:
                ldmia    r2!, {r3, r4-r11, lr}
                tst      lr, #0x10
                it       eq
                vldmiaeq r2!, {s16-s31}
                msr      psp, r2
                msr      control, r3
                bx       lr
            2:
                movw     r0, #0xED9C
                movt     r0, #0xE000
                ldmia    r3!, {r4-r11}
                stmia    r0, {r4-r11}
                ldmia    r3!, {r4-r11}
                stmia    r0, {r4-r11}
                mov      r3, #5
                str      r3, [r0, #-8]
                b        1b
            3:
                mrs      r2, psp
                tst      lr, #0x10
                it       eq
                vstmdbeq r2!, {s16-s31}
                stmdb    r2!, {r3, r4-r11}
                ldr      r3, [sp]
                str      r2, [r3]
                b        0b
            "   :
                : "{r0}"(stack_ptr), "{r1}"(data_ptr)
                : "cc", "memory"
                : "volatile"
            );
            #[cfg(not(feature = "fpu"))]
            asm!("
                mrs      r3, control
                tst      lr, #0x4
                bne      3f
                stmdb    sp!, {r3, r4-r11}
            0:
                ldr      r2, [r0]
                ldmia    r2!, {r3}
                push     {r0, r1, r3, lr}
                cmp      r3, #0
                bne      2f
            1:
                ldmia    r2!, {r3, r4-r11, lr}
                msr      psp, r2
                msr      control, r3
                bx       lr
            2:
                movw     r0, #0xED9C
                movt     r0, #0xE000
                ldmia    r3!, {r4-r11}
                stmia    r0, {r4-r11}
                ldmia    r3!, {r4-r11}
                stmia    r0, {r4-r11}
                mov      r3, #5
                str      r3, [r0, #-8]
                b        1b
            3:
                mrs      r2, psp
                stmdb    r2!, {r3, r4-r11}
                ldr      r3, [sp]
                str      r2, [r3]
                b        0b
            "   :
                : "{r0}"(stack_ptr), "{r1}"(data_ptr)
                : "cc", "memory"
                : "volatile"
            );
            unreachable();
        }
    }
}

impl SvService for SwitchBackService {
    #[cfg_attr(feature = "std", allow(unused_variables))]
    unsafe extern "C" fn handler(&mut self) {
        let Self {
            data_ptr,
            data_size,
        } = *self;
        #[cfg(feature = "std")]
        unimplemented!();
        #[cfg(not(feature = "std"))]
        {
            use core::intrinsics::unreachable;
            #[cfg(feature = "fpu")]
            asm!("
                movw     r2, #0xED94
                movt     r2, #0xE000
                mov      r3, #0
                str      r3, [r2]
                mrs      r3, control
                mrs      r12, psp
                tst      lr, #0x10
                it       eq
                vstmdbeq r12!, {s16-s31}
                stmdb    r12!, {r3, r4-r11, lr}
                pop      {r2, r3, r4, lr}
                stmdb    r12!, {r4}
                str      r12, [r2]
                ldr      r2, [r0]
                cmp      r2, r3
                beq      2f
                str      r3, [r0]
                and      r12, r1, #3
                subs     r1, r1, r12
                beq      1f
            0:
                ldr      r0, [r2], #4
                str      r0, [r3], #4
                subs     r1, r1, #4
                bne      0b
            1:
                lsrs     r12, r12, #1
                itt      ne
                ldrhne   r0, [r2], #2
                strhne   r0, [r3], #2
                itt      cs
                ldrbcs   r0, [r2], #1
                strbcs   r0, [r3], #1
            2:
                tst      lr, #0x4
                bne      3f
                ldmia    sp!, {r3, r4-r11}
                tst      lr, #0x10
                it       eq
                vldmiaeq sp!, {s16-s31}
                msr      control, r3
                bx       lr
            3:
                ldr      r0, [sp]
                ldr      r0, [r0]
                ldmia    r0!, {r3, r4-r11}
                tst      lr, #0x10
                it       eq
                vldmiaeq r0!, {s16-s31}
                msr      psp, r0
                msr      control, r3
                bx       lr
            "   :
                : "{r0}"(data_ptr), "{r1}"(data_size)
                : "cc", "memory"
                : "volatile"
            );
            #[cfg(not(feature = "fpu"))]
            asm!("
                movw     r2, #0xED94
                movt     r2, #0xE000
                mov      r3, #0
                str      r3, [r2]
                mrs      r3, control
                mrs      r12, psp
                stmdb    r12!, {r3, r4-r11, lr}
                pop      {r2, r3, r4, lr}
                stmdb    r12!, {r4}
                str      r12, [r2]
                ldr      r2, [r0]
                cmp      r2, r3
                beq      2f
                str      r3, [r0]
                and      r12, r1, #3
                subs     r1, r1, r12
                beq      1f
            0:
                ldr      r0, [r2], #4
                str      r0, [r3], #4
                subs     r1, r1, #4
                bne      0b
            1:
                lsrs     r12, r12, #1
                itt      ne
                ldrhne   r0, [r2], #2
                strhne   r0, [r3], #2
                itt      cs
                ldrbcs   r0, [r2], #1
                strbcs   r0, [r3], #1
            2:
                tst      lr, #0x4
                ittt     eq
                ldmiaeq  sp!, {r3, r4-r11}
                msreq    control, r3
                bxeq     lr
                ldr      r0, [sp]
                ldr      r0, [r0]
                ldmia    r0!, {r3, r4-r11}
                msr      psp, r0
                msr      control, r3
                bx       lr
            "   :
                : "{r0}"(data_ptr), "{r1}"(data_size)
                : "cc", "memory"
                : "volatile"
            );
            unreachable();
        }
    }
}

impl<Sv, T> Switch<T> for Sv
where
    Sv: SvCall<SwitchContextService>,
    Sv: SvCall<SwitchBackService>,
{
    unsafe fn switch_context(data: *mut T, stack_ptr: *mut *const u8) {
        Self::call(&mut SwitchContextService {
            stack_ptr,
            data_ptr: data as *mut u8,
        });
    }

    unsafe fn switch_back(data: *mut *mut T) {
        Self::call(&mut SwitchBackService {
            data_ptr: data as *mut *mut u8,
            data_size: size_of::<T>(),
        });
    }
}
