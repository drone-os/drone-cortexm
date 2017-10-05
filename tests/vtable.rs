#![feature(alloc)]
#![feature(allocator_api)]
#![feature(allocator_internals)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(global_allocator)]
#![feature(proc_macro)]
#![feature(slice_get_slice)]
#![no_std]

extern crate alloc;
extern crate compiler_builtins;
extern crate drone;
extern crate drone_cortex_m;
extern crate test;

use core::mem::size_of;

drone::heap! {
  #![global_allocator]
}

mod vtable1 {
  ::drone_cortex_m::vtable! {
    //! Test doc attribute
    #![doc = "test attribute"]
    /// Test doc attribute
    #[allow(dead_code)]
    nmi;
    /// Test doc attribute
    #[allow(dead_code)]
    sys_tick;
    /// Test doc attribute
    #[allow(dead_code)]
    10: exti4;
    /// Test doc attribute
    #[allow(dead_code)]
    5: rcc;
  }
}

mod vtable2 {
  ::drone_cortex_m::vtable!();
}

#[test]
fn new() {
  unsafe extern "C" fn reset() -> ! {
    loop {}
  }
  vtable1::VectorTable::new(reset);
  vtable2::VectorTable::new(reset);
}

#[test]
fn size() {
  assert_eq!(
    (size_of::<vtable1::VectorTable>() - size_of::<vtable2::VectorTable>()) / 4,
    11
  );
}
