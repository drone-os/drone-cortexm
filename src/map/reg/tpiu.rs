use reg::prelude::*;

reg! {
  /// Selected Pin Protocol Register.
  pub mod TPIU SPPR;
  0xE004_00F0 0x20 0x0000_0001
  RReg WReg;
  /// Specified the protocol for trace output from the TPIU.
  TXMODE { 0 2 RRRegField WWRegField }
}

reg! {
  /// Formatter and Flush Control Register.
  pub mod TPIU FFCR;
  0xE004_0304 0x20 0x0000_0102
  RReg WReg;
  /// This bit Reads-As-One (RAO), specifying that triggers are inserted when
  /// a trigger pin is asserted.
  TrigIn { 8 1 RRRegField RoRRegField }
  /// Enable continuous formatting.
  EnFCont { 1 1 RRRegField WWRegField }
}
