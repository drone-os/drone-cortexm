use fib::FiberState;
use sv::Switch;
use thr::prelude::*;

mod fiber;
mod yielder;

pub use self::fiber::FiberStack;
pub use self::yielder::Yielder;

#[allow(unions_with_drop_fields)]
pub union Data<I, O> {
  input: I,
  output: O,
  _align: [u32; 0],
}

type StackData<I, Y, R> = Data<I, FiberState<Y, R>>;

/// Creates a new stackful fiber.
pub fn new_stack<Sv, I, Y, R, F>(
  stack_size: usize,
  f: F,
) -> FiberStack<Sv, I, Y, R, F>
where
  Sv: Switch<StackData<I, Y, R>>,
  F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
  F: Send + 'static,
  I: Send + 'static,
  Y: Send + 'static,
  R: Send + 'static,
{
  FiberStack::new(stack_size, f)
}

/// Adds a new stackful fiber on the given `thr`.
pub fn add_stack<T, U, F>(thr: T, stack_size: usize, mut f: F)
where
  T: ThrToken<U>,
  U: ThrTag,
  F: FnMut(Yielder<<T::Thr as Thread>::Sv, (), (), ()>),
  F: Send + 'static,
  <T::Thr as Thread>::Sv: Switch<StackData<(), (), ()>>,
{
  thr
    .as_ref()
    .fib_chain()
    .add(new_stack(stack_size, move |(), yielder| f(yielder)))
}
