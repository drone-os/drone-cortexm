//! Crate errors.

#![allow(missing_docs)]

use serde_xml_rs;
use std;

error_chain! {
  foreign_links {
    Io(std::io::Error);
    ParseInt(std::num::ParseIntError);
    Xml(serde_xml_rs::Error);
  }
}
