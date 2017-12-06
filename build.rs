extern crate drone_cortex_m_svd;
extern crate failure;

use drone_cortex_m_svd::svd_generate;
use failure::Error;
use std::env;
use std::fs::File;
use std::path::Path;
use std::process;

fn main() {
  if let Err(error) = run() {
    eprintln!("{}", error);
    process::exit(1);
  }
}

fn run() -> Result<(), Error> {
  let out_dir = env::var("OUT_DIR")?;
  let out_dir = Path::new(&out_dir);
  let mut svd_out = File::create(out_dir.join("svd_map.rs"))?;
  if let Some(svd_file) = svd_from_feature() {
    let mut svd_in = File::open(svd_file)?;
    svd_generate(&mut svd_in, &mut svd_out)?;
  }
  Ok(())
}

fn svd_from_feature() -> Option<&'static str> {
  #[allow(unreachable_patterns)]
  match () {
    #[cfg(feature = "stm32f100")]
    () => Some("svd_files/STM32F100.svd"),
    #[cfg(feature = "stm32f101")]
    () => Some("svd_files/STM32F101.svd"),
    #[cfg(feature = "stm32f102")]
    () => Some("svd_files/STM32F102.svd"),
    #[cfg(feature = "stm32f103")]
    () => Some("svd_files/STM32F103.svd"),
    #[cfg(feature = "stm32f107")]
    () => Some("svd_files/STM32F107.svd"),
    #[cfg(feature = "stm32l4x1")]
    () => Some("svd_files/STM32L4x1.svd"),
    #[cfg(feature = "stm32l4x2")]
    () => Some("svd_files/STM32L4x2.svd"),
    #[cfg(feature = "stm32l4x3")]
    () => Some("svd_files/STM32L4x3.svd"),
    #[cfg(feature = "stm32l4x5")]
    () => Some("svd_files/STM32L4x5.svd"),
    #[cfg(feature = "stm32l4x6")]
    () => Some("svd_files/STM32L4x6.svd"),
    () => None,
  }
}
