#![feature(alloc)]
#![feature(allocator_api)]
#![feature(allocator_internals)]
#![feature(compiler_builtins_lib)]
#![feature(const_cell_new)]
#![feature(const_fn)]
#![feature(const_ptr_null_mut)]
#![feature(global_allocator)]
#![feature(prelude_import)]
#![feature(proc_macro)]
#![feature(slice_get_slice)]
#![no_std]

extern crate alloc;
extern crate compiler_builtins;
extern crate drone_core;
extern crate drone_stm32;
extern crate test;

#[prelude_import]
#[allow(unused_imports)]
use drone_stm32::prelude::*;

use core::mem::size_of;

drone_core::heap! {
  Heap;
  #[global_allocator]
  ALLOC;
}

mod vtable {
  #![allow(dead_code)]

  use drone_core::thread::thread_local;
  use drone_stm32::vtable;

  vtable! {
    /// Test doc attribute
    #[doc = "test attribute"]
    pub struct VectorTable1;
    /// Test doc attribute
    #[doc = "test attribute"]
    pub struct ThreadIndex1;
    /// Test doc attribute
    #[doc = "test attribute"]
    static THREADS1;
    extern struct ThreadLocal1;

    /// Test doc attribute
    #[doc = "test attribute"]
    pub NMI;
    /// Test doc attribute
    #[doc = "test attribute"]
    pub SYS_TICK;
    /// Test doc attribute
    #[doc = "test attribute"]
    pub 10: EXTI4;
    /// Test doc attribute
    #[doc = "test attribute"]
    pub 5: RCC;
  }

  vtable! {
    pub struct VectorTable2;
    pub struct ThreadIndex2;
    static THREADS2;
    extern struct ThreadLocal2;
  }

  thread_local! {
    pub struct ThreadLocal1;
    extern static THREADS1;
  }

  thread_local! {
    pub struct ThreadLocal2;
    extern static THREADS2;
  }
}

#[test]
fn new() {
  unsafe extern "C" fn reset() -> ! {
    loop {}
  }
  vtable::VectorTable1::new(reset);
  vtable::VectorTable2::new(reset);
}

#[test]
fn size() {
  assert_eq!(
    (size_of::<vtable::VectorTable1>() - size_of::<vtable::VectorTable2>()) / 4,
    11
  );
}
