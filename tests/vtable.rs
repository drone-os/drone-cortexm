#![feature(const_fn)]
#![feature(prelude_import)]

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;

use core::mem::size_of;
use drone_cortexm::{
    map::thr::*,
    sv,
    sv::{sv_handler, SvService},
    thr,
    thr::prelude::*,
};

struct FooService;

struct BarService;

impl SvService for FooService {
    unsafe extern "C" fn handler(&mut self) {}
}

impl SvService for BarService {
    unsafe extern "C" fn handler(&mut self) {}
}

thr! {
    thread => pub Thr {};

    #[allow(dead_code)]
    local => pub ThrLocal {};

    vtable => pub Vtable;

    #[allow(dead_code)]
    index => pub Thrs;

    init => pub ThrsInit;

    supervisor => Sv;

    threads => {
        exceptions => {
            /// Test doc attribute
            #[doc = "test attribute"]
            pub outer(nmi_handler) nmi,
            /// Test doc attribute
            #[doc = "test attribute"]
            pub naked(sv_handler::<Sv>) sv_call,
            /// Test doc attribute
            #[doc = "test attribute"]
            pub sys_tick,
        },
        interrupts => {
            /// Test doc attribute
            #[doc = "test attribute"]
            10: pub exti4,
            /// Test doc attribute
            #[doc = "test attribute"]
            5: pub naked(rcc_handler) rcc,
        },
    };
}

extern "C" fn nmi_handler() {}

extern "C" fn rcc_handler() {}

sv! {
    pub struct Sv;
    pub static SERVICES;

    FooService;
    BarService;
}

#[test]
fn new() {
    unsafe extern "C" fn reset() -> ! {
        loop {}
    }
    Vtable::new(reset);
}

#[test]
fn size() {
    assert_eq!(unsafe { THREADS.len() }, 4);
    assert_eq!(size_of::<Vtable>(), 208);
    assert_eq!(SERVICES.len(), 2);
}
