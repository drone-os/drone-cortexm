use core::mem::size_of;
use drone_core::sv::{SvCall, SvService};

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
  // Store R4-R11 on the callee stack. Update callee stack pointer if it is PSP.
  // Store `stack_ptr`, `data_ptr`, and `LR` on MSP for future use by
  // `switch_back`. Return to the new stack.
  unsafe extern "C" fn handler(&mut self) {
    let Self {
      stack_ptr,
      data_ptr,
    } = *self;
    asm!("
      tst    lr, #4
      itt    eq
      pusheq {r4-r11}
      beq    0f

      mrs    r2, psp
      stmfd  r2!, {r4-r11}
      ldr    r3, [sp]
      str    r2, [r3]

    0:
      push   {r0, r1, lr}
      ldr    r0, [r0]
      ldmfd  r0!, {r4-r11}
      msr    psp, r0
      orr    lr, #0x1C
    " :
      : "{r0}"(stack_ptr), "{r1}"(data_ptr)
      : "cc", "memory"
      : "volatile");
  }
}

impl SvService for SwitchBackService {
  // Store R4-R11 on the stored `stack_ptr`. Copy bytes from the provided
  // `*data_ptr` to the stored `data_ptr`. Return to the old stack.
  unsafe extern "C" fn handler(&mut self) {
    let Self {
      data_ptr,
      data_size,
    } = *self;
    asm!("
      pop    {r2}
      mrs    r3, psp
      stmfd  r3!, {r4-r11}
      str    r3, [r2]

      pop    {r3, lr}
      ldr    r2, [r0]
      cmp    r2, r3
      beq    2f
      str    r3, [r0]

      and    r4, r1, #3
      subs   r1, r1, r4
      beq    1f
    0:
      ldr    r0, [r2], #4
      str    r0, [r3], #4
      subs   r1, r1, #4
      bne    0b
    1:
      lsrs   r4, r4, #1
      itt    ne
      ldrhne r0, [r2], #2
      strhne r0, [r3], #2
      itt    cs
      ldrbcs r0, [r2], #1
      strbcs r0, [r3], #1

    2:
      tst    lr, #4
      itt    eq
      popeq  {r4-r11}
      bxeq   lr

      ldr    r0, [sp]
      ldr    r0, [r0]
      ldmfd  r0!, {r4-r11}
      msr    psp, r0
    " :
      : "{r0}"(data_ptr), "{r1}"(data_size)
      : "cc", "memory"
      : "volatile");
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
