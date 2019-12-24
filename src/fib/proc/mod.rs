mod fiber;
mod yielder;

pub use self::{fiber::FiberProc, yielder::Yielder};

use crate::{fib::FiberState, sv::Switch, thr::ThrSv};
use core::mem::ManuallyDrop;

pub union Data<I, O> {
    input: ManuallyDrop<I>,
    output: ManuallyDrop<O>,
    _align: [u32; 0],
}

type ProcData<I, Y, R> = Data<I, FiberState<Y, R>>;

/// Creates a stackful fiber from the closure `f`.
///
/// The first argument to the closure is
/// [`Fiber::Input`](crate::fib::Fiber::Input).
///
/// This type of fiber yields on each [`proc_yield`](Yielder::proc_yield) call
/// on the second [`Yielder`] argument.
///
/// # Panics
///
/// * If MPU not present.
/// * If `stack_size` is insufficient to store the initial frame.
#[inline]
pub fn new_proc<Sv, I, Y, R, F>(stack_size: usize, f: F) -> FiberProc<Sv, I, Y, R, F>
where
    Sv: Switch<ProcData<I, Y, R>>,
    F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
    F: Send + 'static,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    unsafe { FiberProc::new(stack_size, false, false, f) }
}

/// Creates a stackful fiber from the closure `f`, without memory protection.
///
/// The first argument to the closure is
/// [`Fiber::Input`](crate::fib::Fiber::Input).
///
/// This type of fiber yields on each [`proc_yield`](Yielder::proc_yield) call
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
pub unsafe fn new_proc_unchecked<Sv, I, Y, R, F>(
    stack_size: usize,
    f: F,
) -> FiberProc<Sv, I, Y, R, F>
where
    Sv: Switch<ProcData<I, Y, R>>,
    F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
    F: Send + 'static,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    FiberProc::new(stack_size, false, true, f)
}

/// Creates a stackful fiber from the closure `f`, which will run in
/// unprivileged mode.
///
/// The first argument to the closure is
/// [`Fiber::Input`](crate::fib::Fiber::Input).
///
/// This type of fiber yields on each [`proc_yield`](Yielder::proc_yield) call
/// on the second [`Yielder`] argument.
///
/// # Panics
///
/// * If MPU not present.
/// * If `stack_size` is insufficient to store the initial frame.
#[inline]
pub fn new_proc_unprivileged<Sv, I, Y, R, F>(stack_size: usize, f: F) -> FiberProc<Sv, I, Y, R, F>
where
    Sv: Switch<ProcData<I, Y, R>>,
    F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
    F: Send + 'static,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    unsafe { FiberProc::new(stack_size, true, false, f) }
}

/// Creates a stackful fiber from the closure `f`, which will run in
/// unprivileged mode, without memory protection.
///
/// The first argument to the closure is
/// [`Fiber::Input`](crate::fib::Fiber::Input).
///
/// This type of fiber yields on each [`proc_yield`](Yielder::proc_yield) call
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
pub unsafe fn new_proc_unprivileged_unchecked<Sv, I, Y, R, F>(
    stack_size: usize,
    f: F,
) -> FiberProc<Sv, I, Y, R, F>
where
    Sv: Switch<ProcData<I, Y, R>>,
    F: FnMut(I, Yielder<Sv, I, Y, R>) -> R,
    F: Send + 'static,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    FiberProc::new(stack_size, true, true, f)
}

/// Extends [`ThrToken`](crate::thr::ThrToken) types with `add_proc` methods.
pub trait ThrFiberProc: ThrSv {
    /// Adds a stackful fiber for the closure `f` to the fiber chain.
    ///
    /// # Panics
    ///
    /// * If MPU not present.
    /// * If `stack_size` is insufficient to store the initial frame.
    #[inline]
    fn add_proc<F>(self, stack_size: usize, mut f: F)
    where
        F: FnMut(Yielder<Self::Sv, (), (), ()>),
        F: Send + 'static,
        Self::Sv: Switch<ProcData<(), (), ()>>,
    {
        self.add_fib(new_proc(stack_size, move |(), yielder| f(yielder)))
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
    unsafe fn add_proc_unchecked<F>(self, stack_size: usize, mut f: F)
    where
        F: FnMut(Yielder<Self::Sv, (), (), ()>),
        F: Send + 'static,
        Self::Sv: Switch<ProcData<(), (), ()>>,
    {
        self.add_fib(new_proc_unchecked(stack_size, move |(), yielder| f(yielder)))
    }

    /// Adds a stackful fiber for the closure `f` to the fiber chain, which will
    /// run in unprivileged mode.
    ///
    /// # Panics
    ///
    /// * If MPU not present.
    /// * If `stack_size` is insufficient to store the initial frame.
    #[inline]
    fn add_proc_unprivileged<F>(self, stack_size: usize, mut f: F)
    where
        F: FnMut(Yielder<Self::Sv, (), (), ()>),
        F: Send + 'static,
        Self::Sv: Switch<ProcData<(), (), ()>>,
    {
        self.add_fib(new_proc_unprivileged(stack_size, move |(), yielder| f(yielder)))
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
    unsafe fn add_proc_unprivileged_unchecked<F>(self, stack_size: usize, mut f: F)
    where
        F: FnMut(Yielder<Self::Sv, (), (), ()>),
        F: Send + 'static,
        Self::Sv: Switch<ProcData<(), (), ()>>,
    {
        self.add_fib(new_proc_unprivileged_unchecked(stack_size, move |(), yielder| f(yielder)))
    }
}

impl<T: ThrSv> ThrFiberProc for T {}

impl<I, O> Data<I, O> {
    fn from_input(input: I) -> Self {
        Self { input: ManuallyDrop::new(input) }
    }

    fn from_output(output: O) -> Self {
        Self { output: ManuallyDrop::new(output) }
    }

    unsafe fn into_input(self) -> I {
        ManuallyDrop::into_inner(self.input)
    }

    unsafe fn into_output(self) -> O {
        ManuallyDrop::into_inner(self.output)
    }
}
