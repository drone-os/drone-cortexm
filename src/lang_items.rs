use core::fmt;
use {cpu, itm};

#[linkage = "weak"]
#[lang = "panic_fmt"]
unsafe extern "C" fn begin(
  args: fmt::Arguments,
  file: &'static str,
  line: u32,
  _col: u32,
) -> ! {
  print!("panicked at '");
  itm::write_fmt(args);
  println!("', {}:{}", file, line);
  itm::flush();
  cpu::self_reset()
}

#[linkage = "weak"]
#[lang = "oom"]
fn oom() -> ! {
  cpu::self_reset()
}
