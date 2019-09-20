use crate::{itm, processor};
use core::{alloc::Layout, panic::PanicInfo};

#[panic_handler]
fn begin_panic(pi: &PanicInfo<'_>) -> ! {
    eprintln!("{}", pi);
    abort()
}

#[lang = "oom"]
fn oom(layout: Layout) -> ! {
    eprintln!(
        "Couldn't allocate memory of size {}. Aborting!",
        layout.size()
    );
    abort()
}

fn abort() -> ! {
    itm::flush();
    processor::self_reset()
}
