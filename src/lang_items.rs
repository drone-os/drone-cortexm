use core::alloc::Layout;
use core::panic::PanicInfo;
use {cpu, itm};

#[linkage = "weak"]
#[panic_handler]
fn begin_panic(pi: &PanicInfo) -> ! {
  println!("{}", pi);
  itm::flush();
  cpu::self_reset()
}

#[linkage = "weak"]
#[lang = "oom"]
fn oom(_layout: Layout) -> ! {
  cpu::self_reset()
}
