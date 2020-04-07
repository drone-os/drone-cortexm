#![feature(const_fn)]
#![feature(prelude_import)]

#[prelude_import]
#[allow(unused_imports)]
use drone_cortex_m::prelude::*;

use core::mem::size_of;
use drone_cortex_m::{
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

trait Int10: IntToken {}
trait Int5: IntToken {}

thr::vtable! {
    use Thr;
    use Sv;
    pub struct Vtable;
    pub struct Handlers;
    #[allow(dead_code)]
    pub struct Thrs;
    pub struct ThrsInit;
    pub static THREADS;

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
    use THREADS;
    pub struct Thr {}
    #[allow(dead_code)]
    pub struct ThrLocal {}
}

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
    unsafe extern "C" fn nmi() {}
    unsafe extern "C" fn rcc() {}
    Vtable::new(Handlers { reset, nmi, sv_call: sv_handler::<Sv>, rcc });
}

#[test]
fn size() {
    assert_eq!(unsafe { THREADS.len() }, 4);
    assert_eq!(size_of::<Vtable>(), 208);
    assert_eq!(SERVICES.len(), 2);
}
