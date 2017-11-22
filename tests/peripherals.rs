#![feature(alloc)]
#![feature(allocator_api)]
#![feature(compiler_builtins_lib)]
#![feature(global_allocator)]
#![feature(linkage)]
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

use drone_cortex_m::peripherals;

drone::heap! {
  #![global_allocator]
}

#[test]
fn timer_sys_tick_bind() {
  let _ = peripherals::timer::SysTick!();
}
