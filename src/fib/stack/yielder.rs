use super::{Data, StackData};
use crate::{fib::FiberState, sv::Switch};
use core::{marker::PhantomData, mem::forget};

/// A zero-sized token that provides [`stack_yield`](Yielder::stack_yield)
/// method to yield from [`FiberStack`](crate::fib::FiberStack).
pub struct Yielder<Sv, I, Y, R>
where
    Sv: Switch<StackData<I, Y, R>>,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    _sv: PhantomData<*const Sv>,
    _input: PhantomData<*const I>,
    _yield: PhantomData<*const Y>,
    _return: PhantomData<*const R>,
}

impl<Sv, I, Y, R> Yielder<Sv, I, Y, R>
where
    Sv: Switch<StackData<I, Y, R>>,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    /// Creates a new yielder token.
    ///
    /// # Safety
    ///
    /// The token must be created only in a closure provided to a
    /// [`FiberStack`](crate::fib::FiberStack). The type parameters for the
    /// [`Yielder`] must be equal to the type parameters for the
    /// [`FiberStack`](crate::fib::FiberStack).
    #[inline]
    pub unsafe fn new() -> Self {
        Self {
            _sv: PhantomData,
            _input: PhantomData,
            _yield: PhantomData,
            _return: PhantomData,
        }
    }

    /// Yields from the [`FiberStack`](crate::fib::FiberStack).
    ///
    /// This method blocks, the stack is saved, and the fiber is suspended.
    #[inline]
    pub fn stack_yield(self, output: Y) -> I {
        unsafe {
            let output = FiberState::Yielded(output);
            let mut data = Data { output };
            let mut data_ptr = &mut data as *mut _;
            Sv::switch_back(&mut data_ptr);
            forget(data);
            data_ptr.read().input
        }
    }
}

impl<Sv, I, Y, R> Clone for Yielder<Sv, I, Y, R>
where
    Sv: Switch<StackData<I, Y, R>>,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    fn clone(&self) -> Self {
        unsafe { Self::new() }
    }
}

impl<Sv, I, Y, R> Copy for Yielder<Sv, I, Y, R>
where
    Sv: Switch<StackData<I, Y, R>>,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
}
