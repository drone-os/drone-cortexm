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
extern crate drone;
extern crate drone_cortex_m;
extern crate test;

#[prelude_import]
#[allow(unused_imports)]
use drone_cortex_m::prelude::*;

use core::mem::size_of;

drone::heap! {
  Heap;
  #[global_allocator]
  ALLOC;
}

mod vtable1 {
  #![allow(dead_code)]

  use super::*;

  drone::thread::thread_local!(ThreadLocal; THREADS);

  trait Irq10<T> {}
  trait Irq5<T> {}

  drone_cortex_m::vtable! {
    /// Test doc attribute
    #[doc = "test attribute"]
    VectorTable;
    /// Test doc attribute
    #[doc = "test attribute"]
    Bindings;
    /// Test doc attribute
    #[doc = "test attribute"]
    THREADS;
    ThreadLocal;

    /// Test doc attribute
    #[doc = "test attribute"]
    NMI;
    /// Test doc attribute
    #[doc = "test attribute"]
    SYS_TICK;
    /// Test doc attribute
    #[doc = "test attribute"]
    10: EXTI4;
    /// Test doc attribute
    #[doc = "test attribute"]
    5: RCC;
  }
}

mod vtable2 {
  #![allow(dead_code)]

  use super::*;

  drone::thread::thread_local!(ThreadLocal; THREADS);
  drone_cortex_m::vtable!(VectorTable; Bindings; THREADS; ThreadLocal);
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
