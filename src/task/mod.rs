//! Tasks subsystem.

mod future;
mod stream;

pub use self::future::DroneFuture;
pub use self::stream::{DroneStream, StreamWait};

use futures::executor::Notify;

struct NopNotify;

const NOP_NOTIFY: NopNotify = NopNotify;

impl Notify for NopNotify {
  fn notify(&self, _id: usize) {}
}
