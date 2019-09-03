#![feature(const_fn)]
#![feature(prelude_import)]

#[prelude_import]
#[allow(unused_imports)]
use drone_cortex_m::prelude::*;

use core::mem::size_of;
use drone_cortex_m::sv::{sv_handler, SvService};

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
    use drone_cortex_m::{map::thr::*, sv, thr::prelude::*};

    use drone_cortex_m::thr;

    trait Int10: IntToken {}
    trait Int5: IntToken {}

    thr::vtable! {
        use Thr;
        use Sv;
        pub struct Vtable;
        pub struct Handlers;
        #[allow(dead_code)]
        pub struct Thrs;
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
}

mod b {
    use drone_cortex_m::thr;

    thr::vtable! {
        use Thr;
        pub struct Vtable;
        pub struct Handlers;
        #[allow(dead_code)]
        pub struct Thrs;
        pub static THREADS;
    }

    thr! {
        use THREADS;
        pub struct Thr {}
        #[allow(dead_code)]
        pub struct ThrLocal {}
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
    assert_eq!(
        (size_of::<a::Vtable>() - size_of::<b::Vtable>()) / size_of::<usize>(),
        11
    );
    assert_eq!(a::SERVICES.len(), 2);
}
