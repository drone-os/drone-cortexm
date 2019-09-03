use crate::{itm, processor};
use core::{alloc::Layout, panic::PanicInfo};

#[panic_handler]
fn begin_panic(pi: &PanicInfo<'_>) -> ! {
    println!("{}", pi);
    abort()
}

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
    processor::self_reset()
}
