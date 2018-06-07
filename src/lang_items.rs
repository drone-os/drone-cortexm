use core::panic::PanicInfo;
use {cpu, itm};

#[linkage = "weak"]
#[panic_implementation]
fn begin_panic(pi: &PanicInfo) -> ! {
  println!("{}", pi);
  itm::flush();
  cpu::self_reset()
}

#[linkage = "weak"]
#[lang = "oom"]
fn oom() -> ! {
  cpu::self_reset()
}
