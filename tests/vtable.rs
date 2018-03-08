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
extern crate drone_stm32 as drone_plat;
extern crate test;

#[prelude_import]
#[allow(unused_imports)]
use drone_plat::prelude::*;

use core::mem::{size_of, transmute_copy};
use drone_core::heap;
use drone_core::sv::SvService;

heap! {
  struct Heap;
  #[global_allocator]
  static ALLOC;
  size = 0;
  pools = [];
}

struct FooService;

struct BarService;

impl SvService for FooService {
  unsafe extern "C" fn handler(&mut self) {}
}

impl SvService for BarService {
  unsafe extern "C" fn handler(&mut self) {}
}

mod a {
  use super::{BarService, FooService};
  use drone_core::thr;
  use drone_plat::{sv, vtable};
  use drone_plat::thr::int::*;
  use drone_plat::thr::prelude::*;

  trait Int10<T: ThrTag>: IntToken<T> {}
  trait Int5<T: ThrTag>: IntToken<T> {}

  vtable! {
    pub struct Vtable;
    pub struct Handlers;
    #[allow(dead_code)]
    pub struct ThrIdx;
    pub static THREADS;
    extern struct Thr;

    /// Test doc attribute
    #[doc = "test attribute"]
    pub extern NMI;
    /// Test doc attribute
    #[doc = "test attribute"]
    pub SYS_TICK;
    /// Test doc attribute
    #[doc = "test attribute"]
    pub 10: EXTI4;
    /// Test doc attribute
    #[doc = "test attribute"]
    fn 5: RCC;
  }

  thr! {
    pub struct Thr;
    pub struct ThrLocal;
    extern struct Sv;
    extern static THREADS;
  }

  sv! {
    pub struct Sv;
    pub static SERVICES;

    FooService;
    BarService;
  }
}

mod b {
  use drone_core::thr;
  use drone_plat::{sv, vtable};

  vtable! {
    pub struct Vtable;
    pub struct Handlers;
    #[allow(dead_code)]
    pub struct ThrIdx;
    pub static THREADS;
    extern struct Thr;
  }

  thr! {
    pub struct Thr;
    pub struct ThrLocal;
    extern struct Sv;
    extern static THREADS;
  }

  sv! {
    pub struct Sv;
    pub static SERVICES;
  }
}

#[test]
fn new() {
  unsafe extern "C" fn reset() -> ! {
    loop {}
  }
  unsafe extern "C" fn nmi() {}
  unsafe extern "C" fn rcc() {}
  a::Vtable::new(a::Handlers { reset, nmi, rcc });
  b::Vtable::new(b::Handlers { reset });
}

#[test]
fn size() {
  assert_eq!(unsafe { a::THREADS.len() }, 4);
  assert_eq!(unsafe { b::THREADS.len() }, 1);
  assert_eq!((size_of::<a::Vtable>() - size_of::<b::Vtable>()) / 4, 11);
  assert_eq!(a::SERVICES.len(), 2);
  assert_eq!(b::SERVICES.len(), 0);
}

#[test]
fn sv() {
  assert!(unsafe { transmute_copy::<a::Sv, usize>(&a::SERVICES[0]) } & 1 != 0);
  assert!(unsafe { transmute_copy::<a::Sv, usize>(&a::SERVICES[1]) } & 1 != 0);
}
