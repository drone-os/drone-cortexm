#![feature(naked_functions)]
#![no_implicit_prelude]

use ::drone_cortexm::{
    sv,
    sv::{Supervisor, SvService},
    thr,
    thr::Thread,
};
use ::std::{assert_eq, mem::size_of};

struct FooService;

struct BarService;

impl SvService for FooService {
    unsafe extern "C" fn handler(&mut self) {}
}

impl SvService for BarService {
    unsafe extern "C" fn handler(&mut self) {}
}

thr::nvic! {
    thread => pub Thr {};

    #[allow(dead_code)]
    local => pub ThrLocal {};

    #[allow(dead_code)]
    index => pub Thrs;

    vtable => pub Vtable;

    init => pub ThrsInit;

    supervisor => Sv;

    threads => {
        exceptions => {
            /// Test doc attribute
            #[doc = "test attribute"]
            pub outer(nmi_handler) nmi;
            /// Test doc attribute
            #[doc = "test attribute"]
            pub naked(Sv::handler) sv_call;
            /// Test doc attribute
            #[doc = "test attribute"]
            pub sys_tick;
        };
        interrupts => {
            /// Test doc attribute
            #[doc = "test attribute"]
            10: pub exti4;
            /// Test doc attribute
            #[doc = "test attribute"]
            5: pub naked(rcc_handler) rcc;
        };
    };
}

fn nmi_handler(_thr: &Thr) {}

extern "C" fn rcc_handler() {}

sv::pool! {
    pool => pub SERVICES;
    supervisor => pub Sv;
    services => {
        FooService;
        BarService;
    }
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
    assert_eq!(Thr::COUNT, 3);
    assert_eq!(size_of::<Vtable>(), 208);
    assert_eq!(SERVICES.len(), 2);
}
