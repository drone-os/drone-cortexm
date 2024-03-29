#![feature(naked_functions)]
#![no_implicit_prelude]

use ::drone_cortexm::sv::{Supervisor, SvService};
use ::drone_cortexm::thr::Thread;
use ::drone_cortexm::{sv, thr};
use ::std::assert_eq;
use ::std::mem::size_of;

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

    vectors => pub Vectors;

    #[repr(align(256))]
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
    Vectors::new(reset);
}

#[test]
fn size() {
    assert_eq!(Thr::COUNT, 3);
    assert_eq!(size_of::<Vectors>(), 208);
    assert_eq!(SERVICES.len(), 2);
}
