/// An interrupt.
pub trait ThreadInterrupt<T: Thread>: ThreadBinding<T> {
  /// An interrupt position within the vector table.
  const POSITION: usize;
}
