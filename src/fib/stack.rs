use crate::{fib::FiberState, sv::Switch, thr::prelude::*};

mod fiber;
mod yielder;

pub use self::{fiber::FiberStack, yielder::Yielder};

#[allow(unions_with_drop_fields)]
pub union Data<I, O> {
    input: I,
    output: O,
    _align: [u32; 0],
}

type StackData<I, Y, R> = Data<I, FiberState<Y, R>>;

/// Creates a new stackful fiber.
///
/// # Panics
///
/// * If `stack_size` is insufficient to store an initial frame.
/// * If MPU not present.
pub fn new_stack<Sv, I, Y, R, F>(stack_size: usize, f: F) -> FiberStack<Sv, I, Y, R, F>
where
    Sv: Switch<StackData<I, Y, R>>,
    F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
    F: Send + 'static,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    unsafe { FiberStack::new(stack_size, false, false, f) }
}

/// Creates a new stackful fiber.
///
/// # Safety
///
/// Unprotected from stack overflow.
///
/// # Panics
///
/// * If `stack_size` is insufficient to store an initial frame.
pub unsafe fn new_stack_unchecked<Sv, I, Y, R, F>(
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
    FiberStack::new(stack_size, false, true, f)
}

/// Creates a new stackful fiber.
///
/// # Panics
///
/// * If `stack_size` is insufficient to store an initial frame.
/// * If MPU not present.
pub fn new_stack_unprivileged<Sv, I, Y, R, F>(stack_size: usize, f: F) -> FiberStack<Sv, I, Y, R, F>
where
    Sv: Switch<StackData<I, Y, R>>,
    F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
    F: Send + 'static,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    unsafe { FiberStack::new(stack_size, true, false, f) }
}

/// Creates a new stackful fiber.
///
/// # Safety
///
/// Unprotected from stack overflow.
///
/// # Panics
///
/// * If `stack_size` is insufficient to store an initial frame.
pub unsafe fn new_stack_unprivileged_unchecked<Sv, I, Y, R, F>(
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
    FiberStack::new(stack_size, true, true, f)
}

/// Stackful fiber extension to the thread token.
pub trait ThrFiberStack<T: ThrAttach>: ThrToken<T> {
    /// Adds a new stackful fiber.
    ///
    /// # Panics
    ///
    /// * If `stack_size` is insufficient to store an initial frame.
    /// * If MPU not present.
    fn add_stack<F>(self, stack_size: usize, mut f: F)
    where
        F: FnMut(Yielder<<Self::Thr as Thread>::Sv, (), (), ()>),
        F: Send + 'static,
        <Self::Thr as Thread>::Sv: Switch<StackData<(), (), ()>>,
    {
        self.add_fib(new_stack(stack_size, move |(), yielder| f(yielder)))
    }

    /// Adds a new stackful fiber.
    ///
    /// # Safety
    ///
    /// Unprotected from stack overflow.
    ///
    /// # Panics
    ///
    /// * If `stack_size` is insufficient to store an initial frame.
    unsafe fn add_stack_unchecked<F>(self, stack_size: usize, mut f: F)
    where
        F: FnMut(Yielder<<Self::Thr as Thread>::Sv, (), (), ()>),
        F: Send + 'static,
        <Self::Thr as Thread>::Sv: Switch<StackData<(), (), ()>>,
    {
        self.add_fib(new_stack_unchecked(stack_size, move |(), yielder| {
            f(yielder)
        }))
    }

    /// Adds a new stackful fiber.
    ///
    /// # Panics
    ///
    /// * If `stack_size` is insufficient to store an initial frame.
    /// * If MPU not present.
    fn add_stack_unprivileged<F>(self, stack_size: usize, mut f: F)
    where
        F: FnMut(Yielder<<Self::Thr as Thread>::Sv, (), (), ()>),
        F: Send + 'static,
        <Self::Thr as Thread>::Sv: Switch<StackData<(), (), ()>>,
    {
        self.add_fib(new_stack_unprivileged(stack_size, move |(), yielder| {
            f(yielder)
        }))
    }

    /// Adds a new stackful fiber.
    ///
    /// # Safety
    ///
    /// Unprotected from stack overflow.
    ///
    /// # Panics
    ///
    /// * If `stack_size` is insufficient to store an initial frame.
    unsafe fn add_stack_unprivileged_unchecked<F>(self, stack_size: usize, mut f: F)
    where
        F: FnMut(Yielder<<Self::Thr as Thread>::Sv, (), (), ()>),
        F: Send + 'static,
        <Self::Thr as Thread>::Sv: Switch<StackData<(), (), ()>>,
    {
        self.add_fib(new_stack_unprivileged_unchecked(
            stack_size,
            move |(), yielder| f(yielder),
        ))
    }
}
