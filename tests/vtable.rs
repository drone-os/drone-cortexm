#![feature(allocator_api)]
#![feature(allocator_internals)]
#![feature(const_fn)]
#![feature(prelude_import)]
#![no_std]

extern crate drone_core;
extern crate drone_cortex_m;
extern crate test;
use drone_cortex_m as drone_plat;

#[prelude_import]
#[allow(unused_imports)]
use drone_plat::prelude::*;

use core::mem::{size_of, transmute_copy};
use drone_core::{heap, sv::SvService};
use drone_plat::sv::sv_handler;

heap! {
  struct Heap;
  size = 0;
  pools = [];
}

#[global_allocator]
static mut ALLOC: Heap = Heap::new();

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
  use drone_plat::{map::thr::*, sv, thr::prelude::*, vtable};

  trait Int10<T: ThrTag>: IntToken<T> {}
  trait Int5<T: ThrTag>: IntToken<T> {}

  vtable! {
    pub struct Vtable;
    pub struct Handlers;
    #[allow(dead_code)]
    pub struct Thrs;
    pub static THREADS;
    extern struct Thr;

    /// Test doc attribute
    #[doc = "test attribute"]
    pub extern NMI;
    /// Test doc attribute
    #[doc = "test attribute"]
    fn SV_CALL;
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
    #[allow(dead_code)]
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
  use drone_plat::vtable;

  vtable! {
    pub struct Vtable;
    pub struct Handlers;
    #[allow(dead_code)]
    pub struct Thrs;
    pub static THREADS;
    extern struct Thr;
  }

  thr! {
    pub struct Thr;
    #[allow(dead_code)]
    pub struct ThrLocal;
    extern static THREADS;
  }
}

#[test]
fn new() {
  unsafe extern "C" fn reset() -> ! {
    loop {}
  }
  unsafe extern "C" fn nmi() {}
  unsafe extern "C" fn rcc() {}
  a::Vtable::new(a::Handlers {
    reset,
    nmi,
    sv_call: sv_handler::<a::Sv>,
    rcc,
  });
  b::Vtable::new(b::Handlers { reset });
}

#[test]
fn size() {
  assert_eq!(unsafe { a::THREADS.len() }, 4);
  assert_eq!(unsafe { b::THREADS.len() }, 1);
  assert_eq!((size_of::<a::Vtable>() - size_of::<b::Vtable>()) / 4, 11);
  assert_eq!(a::SERVICES.len(), 2);
}

#[test]
fn sv() {
  assert!(unsafe { transmute_copy::<a::Sv, usize>(&a::SERVICES[0]) } & 1 != 0);
  assert!(unsafe { transmute_copy::<a::Sv, usize>(&a::SERVICES[1]) } & 1 != 0);
}
