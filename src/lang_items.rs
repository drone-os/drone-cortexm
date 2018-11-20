use core::alloc::Layout;
use core::panic::PanicInfo;
use {cpu, itm};

const ABORT_DELAY: u32 = 0x400;

#[linkage = "weak"]
#[panic_handler]
fn begin_panic(pi: &PanicInfo) -> ! {
  println!("{}", pi);
  abort()
}

#[linkage = "weak"]
#[lang = "oom"]
fn oom(layout: Layout) -> ! {
  println!(
    "Couldn't allocate memory of size {}. Aborting!",
    layout.size()
  );
  abort()
}

fn abort() -> ! {
  itm::flush();
  cpu::spin(ABORT_DELAY);
  cpu::self_reset()
}
