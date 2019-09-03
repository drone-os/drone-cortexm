mod fiber;
mod yielder;

pub use self::{fiber::FiberStack, yielder::Yielder};

use crate::{fib::FiberState, sv::Switch, thr::ThrSv};

#[allow(unions_with_drop_fields)]
pub union Data<I, O> {
    input: I,
    output: O,
    _align: [u32; 0],
}

type StackData<I, Y, R> = Data<I, FiberState<Y, R>>;

/// Creates a stackful fiber from the closure `f`.
///
/// The first argument to the closure is
/// [`Fiber::Input`](crate::fib::Fiber::Input).
///
/// This type of fiber yields on each [`stack_yield`](Yielder::stack_yield) call
/// on the second [`Yielder`] argument.
///
/// # Panics
///
/// * If MPU not present.
/// * If `stack_size` is insufficient to store the initial frame.
#[inline]
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

/// Creates a stackful fiber from the closure `f`, without memory protection.
///
/// The first argument to the closure is
/// [`Fiber::Input`](crate::fib::Fiber::Input).
///
/// This type of fiber yields on each [`stack_yield`](Yielder::stack_yield) call
/// on the second [`Yielder`] argument.
///
/// # Safety
///
/// Stack overflow is unchecked.
///
/// # Panics
///
/// * If `stack_size` is insufficient to store the initial frame.
#[inline]
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

/// Creates a stackful fiber from the closure `f`, which will run in
/// unprivileged mode.
///
/// The first argument to the closure is
/// [`Fiber::Input`](crate::fib::Fiber::Input).
///
/// This type of fiber yields on each [`stack_yield`](Yielder::stack_yield) call
/// on the second [`Yielder`] argument.
///
/// # Panics
///
/// * If MPU not present.
/// * If `stack_size` is insufficient to store the initial frame.
#[inline]
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

/// Creates a stackful fiber from the closure `f`, which will run in
/// unprivileged mode, without memory protection.
///
/// The first argument to the closure is
/// [`Fiber::Input`](crate::fib::Fiber::Input).
///
/// This type of fiber yields on each [`stack_yield`](Yielder::stack_yield) call
/// on the second [`Yielder`] argument.
///
/// # Safety
///
/// Stack overflow is unchecked.
///
/// # Panics
///
/// * If `stack_size` is insufficient to store the initial frame.
#[inline]
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

/// Extends [`ThrToken`](crate::thr::ThrToken) types with `add_stack` methods.
pub trait ThrFiberStack: ThrSv {
    /// Adds a stackful fiber for the closure `f` to the fiber chain.
    ///
    /// # Panics
    ///
    /// * If MPU not present.
    /// * If `stack_size` is insufficient to store the initial frame.
    #[inline]
    fn add_stack<F>(self, stack_size: usize, mut f: F)
    where
        F: FnMut(Yielder<Self::Sv, (), (), ()>),
        F: Send + 'static,
        Self::Sv: Switch<StackData<(), (), ()>>,
    {
        self.add_fib(new_stack(stack_size, move |(), yielder| f(yielder)))
    }

    /// Adds a stackful fiber for the closure `f` to the fiber chain, without
    /// memory protection.
    ///
    /// # Safety
    ///
    /// Stack overflow is unchecked.
    ///
    /// # Panics
    ///
    /// * If `stack_size` is insufficient to store the initial frame.
    #[inline]
    unsafe fn add_stack_unchecked<F>(self, stack_size: usize, mut f: F)
    where
        F: FnMut(Yielder<Self::Sv, (), (), ()>),
        F: Send + 'static,
        Self::Sv: Switch<StackData<(), (), ()>>,
    {
        self.add_fib(new_stack_unchecked(stack_size, move |(), yielder| {
            f(yielder)
        }))
    }

    /// Adds a stackful fiber for the closure `f` to the fiber chain, which will
    /// run in unprivileged mode.
    ///
    /// # Panics
    ///
    /// * If MPU not present.
    /// * If `stack_size` is insufficient to store the initial frame.
    #[inline]
    fn add_stack_unprivileged<F>(self, stack_size: usize, mut f: F)
    where
        F: FnMut(Yielder<Self::Sv, (), (), ()>),
        F: Send + 'static,
        Self::Sv: Switch<StackData<(), (), ()>>,
    {
        self.add_fib(new_stack_unprivileged(stack_size, move |(), yielder| {
            f(yielder)
        }))
    }

    /// Adds a stackful fiber for the closure `f` to the fiber chain, which will
    /// run in unprivileged mode, without memory protection.
    ///
    /// # Safety
    ///
    /// Stack overflow is unchecked.
    ///
    /// # Panics
    ///
    /// * If `stack_size` is insufficient to store the initial frame.
    #[inline]
    unsafe fn add_stack_unprivileged_unchecked<F>(self, stack_size: usize, mut f: F)
    where
        F: FnMut(Yielder<Self::Sv, (), (), ()>),
        F: Send + 'static,
        Self::Sv: Switch<StackData<(), (), ()>>,
    {
        self.add_fib(new_stack_unprivileged_unchecked(
            stack_size,
            move |(), yielder| f(yielder),
        ))
    }
}

impl<T: ThrSv> ThrFiberStack for T {}
