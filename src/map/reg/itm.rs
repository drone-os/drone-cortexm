use crate::reg::prelude::*;
use drone_core::reg;

reg! {
  /// Trace Privilege Register.
  pub mod ITM TPR;
  0xE000_0E40 0x20 0x0000_0000
  RReg WReg;
  /// Bit mask to enable unprivileged access to ITM stimulus ports.
  PRIVMASK { 0 32 RRRegField WWRegField }
}

reg! {
  /// Trace Control Register.
  pub mod ITM TCR;
  0xE000_0E80 0x20 0x0000_0000
  RReg WReg;
  /// Indicates whether the ITM is currently processing events.
  BUSY { 23 1 RRRegField RoRRegField }
  /// Identifier for multi-source trace stream formatting.
  TraceBusID { 16 7 RRRegField WWRegField }
  /// Global timestamp frequency.
  GTSFREQ { 10 2 RRRegField WWRegField }
  /// Local timestamp prescaler, used with the trace packet reference clock.
  TSPrescale { 8 2 RRRegField WWRegField }
  /// Enables asynchronous clocking of the timestamp counter.
  SWOENA { 4 1 RRRegField WWRegField }
  /// Enables forwarding of hardware event packet from the DWT unit to the ITM
  /// for output to the TPIU.
  TXENA { 3 1 RRRegField WWRegField }
  /// Enables Synchronization packet transmission for a synchronous TPIU.
  SYNCENA { 2 1 RRRegField WWRegField }
  /// Enables Local timestamp generation.
  TSENA { 1 1 RRRegField WWRegField }
  /// Enables the ITM.
  ITMENA { 0 1 RRRegField WWRegField }
}

reg! {
  /// ITM lock access register.
  pub mod ITM LAR;
  0xE000_0FB0 0x20 0x0000_0000
  WReg WoReg;
  /// Write `0xC5ACCE55` to unlock Write Access to the other ITM registers.
  UNLOCK { 0 32 WWRegField WoWRegField }
}
