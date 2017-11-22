//! Peripheral drivers.

pub mod timer;
pub mod prelude;

/// A peripheral driver.
pub trait Driver {
  /// Corresponding atoms type.
  type Atoms: Atoms;

  /// Destroys the driver and releases contained atoms.
  fn into_atoms(self) -> Self::Atoms;
}

/// A peripheral container of atoms.
pub trait Atoms {
  /// Corresponding driver type.
  type Driver: Driver;

  /// Consumes the atoms and creates a driver.
  fn into_driver(self) -> Self::Driver;
}
