#![feature(transpose_result)]

extern crate drone_mirror_failure as failure;
extern crate drone_stm32_svd;
extern crate xml;

use drone_stm32_svd::svd_generate;
use failure::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{env, process};
use xml::name::OwnedName;
use xml::reader::XmlEvent as ReaderEvent;
use xml::writer::XmlEvent as WriterEvent;
use xml::{EventReader, EventWriter};

fn main() {
  if let Err(error) = run() {
    eprintln!("{}", error);
    process::exit(1);
  }
}

fn run() -> Result<(), Error> {
  let out_dir = env::var("OUT_DIR")?;
  let out_dir = Path::new(&out_dir);
  let mut reg_map = File::create(out_dir.join("svd_reg_map.rs"))?;
  let mut reg_tokens = File::create(out_dir.join("svd_reg_tokens.rs"))?;
  let mut interrupts = File::create(out_dir.join("svd_interrupts.rs"))?;
  let mut svd = File::create(out_dir.join("svd.xml"))?;
  make_svd(&mut svd)?;
  let mut svd = File::open(out_dir.join("svd.xml"))?;
  svd_generate(&mut svd, &mut reg_map, &mut reg_tokens, &mut interrupts)?;
  Ok(())
}

fn make_svd(svd: &mut File) -> Result<(), Error> {
  if cfg!(feature = "stm32f100") {
    patch("svd_files/STM32F100.svd", svd, patch_stm32f1())?;
  } else if cfg!(feature = "stm32f101") {
    patch("svd_files/STM32F101.svd", svd, patch_stm32f1())?;
  } else if cfg!(feature = "stm32f102") {
    patch("svd_files/STM32F102.svd", svd, patch_stm32f1())?;
  } else if cfg!(feature = "stm32f103") {
    patch("svd_files/STM32F103.svd", svd, patch_stm32f1())?;
  } else if cfg!(feature = "stm32f107") {
    patch("svd_files/STM32F107.svd", svd, patch_stm32f1())?;
  } else if cfg!(feature = "stm32l4x1") {
    patch("svd_files/STM32L4x1.svd", svd, patch_stm32l4())?;
  } else if cfg!(feature = "stm32l4x2") {
    patch("svd_files/STM32L4x2.svd", svd, patch_stm32l4())?;
  } else if cfg!(feature = "stm32l4x3") {
    patch("svd_files/STM32L4x3.svd", svd, patch_stm32l4())?;
  } else if cfg!(feature = "stm32l4x5") {
    patch("svd_files/STM32L4x5.svd", svd, patch_stm32l4())?;
  } else if cfg!(feature = "stm32l4x6") {
    patch("svd_files/STM32L4x6.svd", svd, patch_stm32l4())?;
  } else if cfg!(feature = "stm32l4r5") {
    patch("svd_files/STM32L4R5.svd", svd, patch_stm32l4plus())?;
  } else if cfg!(feature = "stm32l4r7") {
    patch("svd_files/STM32L4R7.svd", svd, patch_stm32l4plus())?;
  } else if cfg!(feature = "stm32l4r9") {
    patch("svd_files/STM32L4R9.svd", svd, patch_stm32l4plus())?;
  } else if cfg!(feature = "stm32l4s5") {
    patch("svd_files/STM32L4S5.svd", svd, patch_stm32l4plus())?;
  } else if cfg!(feature = "stm32l4s7") {
    patch("svd_files/STM32L4S7.svd", svd, patch_stm32l4plus())?;
  } else if cfg!(feature = "stm32l4s9") {
    patch("svd_files/STM32L4S9.svd", svd, patch_stm32l4plus())?;
  } else {
    patch("svd_files/blank.svd", svd, |o, e, path| match e {
      ReaderEvent::StartElement { name, .. }
        if name.local_name == "peripherals"
          && check_path(path, &["device"]) =>
      {
        patch_pass(o, e)?;
        patch_add(o, "svd_files/patch/add_itm.xml")?;
        patch_add(o, "svd_files/patch/add_mpu.xml")?;
        patch_add(o, "svd_files/patch/add_scb.xml")?;
        patch_add(o, "svd_files/patch/add_stk.xml")
      }
      _ => patch_pass(o, e),
    })?;
  }
  Ok(())
}

fn patch_stm32f1(
) -> impl FnMut(&mut EventWriter<&mut File>, &ReaderEvent, &[OwnedName])
  -> Result<(), Error> {
  |o, e, path| match e {
    ReaderEvent::StartElement { name, .. }
      if name.local_name == "peripherals" && check_path(path, &["device"]) =>
    {
      patch_pass(o, e)?;
      patch_add(o, "svd_files/patch/add_itm.xml")?;
      patch_add(o, "svd_files/patch/add_mpu.xml")?;
      patch_add(o, "svd_files/patch/add_scb.xml")?;
      patch_add(o, "svd_files/patch/add_stk.xml")
    }
    _ => patch_pass(o, e),
  }
}

fn patch_stm32l4(
) -> impl FnMut(&mut EventWriter<&mut File>, &ReaderEvent, &[OwnedName])
  -> Result<(), Error> {
  |o, e, path| match e {
    ReaderEvent::StartElement { name, .. }
      if name.local_name == "peripherals" && check_path(path, &["device"]) =>
    {
      patch_pass(o, e)?;
      patch_add(o, "svd_files/patch/add_fpu.xml")?;
      patch_add(o, "svd_files/patch/add_itm.xml")?;
      patch_add(o, "svd_files/patch/add_mpu.xml")?;
      patch_add(o, "svd_files/patch/add_scb.xml")?;
      patch_add(o, "svd_files/patch/add_stk.xml")
    }
    _ => patch_pass(o, e),
  }
}

fn patch_stm32l4plus(
) -> impl FnMut(&mut EventWriter<&mut File>, &ReaderEvent, &[OwnedName])
  -> Result<(), Error> {
  let mut peripheral_name = String::new();
  let mut register_name = String::new();
  move |o, e, path| match e {
    ReaderEvent::StartElement { name, .. }
      if name.local_name == "peripherals" && check_path(path, &["device"]) =>
    {
      patch_pass(o, e)?;
      patch_add(o, "svd_files/patch/add_itm.xml")?;
      patch_add(o, "svd_files/patch/add_dmamux.xml")
    }
    ReaderEvent::StartElement { name, .. }
      if name.local_name == "registers"
        && check_path(path, &["device", "peripherals", "peripheral"])
        && peripheral_name == "SCB" =>
    {
      patch_pass(o, e)?;
      patch_add(o, "svd_files/patch/add_scb_demcr.xml")
    }
    ReaderEvent::Characters(s)
      if check_path(path, &["device", "peripherals", "peripheral", "name"]) =>
    {
      peripheral_name = s.clone();
      patch_pass(o, e)
    }
    ReaderEvent::Characters(s)
      if check_path(
        path,
        &[
          "device",
          "peripherals",
          "peripheral",
          "registers",
          "register",
          "name",
        ],
      ) =>
    {
      register_name = s.clone();
      patch_pass(o, e)
    }
    ReaderEvent::Characters(s)
      if s == "read-only"
        && check_path(
          path,
          &[
            "device",
            "peripherals",
            "peripheral",
            "registers",
            "register",
            "access",
          ],
        )
        && register_name == "MPU_CTRL" =>
    {
      o.write(WriterEvent::Characters("read-write"))?;
      Ok(())
    }
    _ => patch_pass(o, e),
  }
}

fn patch<
  F: FnMut(&mut EventWriter<&mut File>, &ReaderEvent, &[OwnedName])
    -> Result<(), Error>,
>(
  input: &str,
  output: &mut File,
  mut f: F,
) -> Result<(), Error> {
  let input = EventReader::new(BufReader::new(File::open(input)?));
  let mut output = EventWriter::new(output);
  let mut path = Vec::new();
  for event in input {
    let event = event?;
    f(&mut output, &event, &path)?;
    match &event {
      ReaderEvent::StartElement { name, .. } => {
        path.push(name.clone());
      }
      ReaderEvent::EndElement { name, .. } => {
        let tail = path.pop();
        assert_eq!(tail.as_ref(), Some(name));
      }
      _ => {}
    }
  }
  Ok(())
}

fn patch_pass(
  output: &mut EventWriter<&mut File>,
  event: &ReaderEvent,
) -> Result<(), Error> {
  event
    .as_writer_event()
    .map(|x| output.write(x))
    .transpose()?;
  Ok(())
}

fn patch_add(
  output: &mut EventWriter<&mut File>,
  patch: &str,
) -> Result<(), Error> {
  for e in EventReader::new(BufReader::new(File::open(patch)?)) {
    match e? {
      ReaderEvent::StartDocument { .. } | ReaderEvent::EndDocument => {}
      e => patch_pass(output, &e)?,
    }
  }
  Ok(())
}

fn check_path(a: &[OwnedName], b: &[&str]) -> bool {
  a.len() == b.len()
    && a
      .iter()
      .zip(b.iter())
      .try_for_each(|(a, &b)| if a.local_name == b { Some(()) } else { None })
      .is_some()
}
