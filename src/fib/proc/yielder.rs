use super::{Data, ProcData};
use crate::fib;
use crate::sv::Switch;
use core::marker::PhantomData;
use core::ptr;

/// A zero-sized token that provides [`proc_yield`](Yielder::proc_yield) method
/// to yield from [`FiberProc`](crate::fib::FiberProc).
pub struct Yielder<Sv, I, Y, R>
where
    Sv: Switch<ProcData<I, Y, R>>,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    _sv: PhantomData<*const Sv>,
    _input: PhantomData<*const I>,
    _yield: PhantomData<*const Y>,
    _return: PhantomData<*const R>,
}

#[allow(clippy::unused_self)]
impl<Sv, I, Y, R> Yielder<Sv, I, Y, R>
where
    Sv: Switch<ProcData<I, Y, R>>,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
    /// Creates a new yielder token.
    ///
    /// # Safety
    ///
    /// The token must be created only in a closure provided to a
    /// [`FiberProc`](crate::fib::FiberProc). The type parameters for the
    /// [`Yielder`] must be equal to the type parameters for the
    /// [`FiberProc`](crate::fib::FiberProc).
    #[inline]
    pub unsafe fn new() -> Self {
        Self { _sv: PhantomData, _input: PhantomData, _yield: PhantomData, _return: PhantomData }
    }

    /// Yields from the [`FiberProc`](crate::fib::FiberProc).
    ///
    /// This method blocks, the stack is saved, and the fiber is suspended.
    #[inline]
    pub fn proc_yield(self, output: Y) -> I {
        unsafe {
            let mut data = Data::from_output(fib::Yielded(output));
            let mut data_ptr = ptr::addr_of_mut!(data);
            Sv::switch_back(&mut data_ptr);
            data_ptr.read().into_input()
        }
    }
}

impl<Sv, I, Y, R> Clone for Yielder<Sv, I, Y, R>
where
    Sv: Switch<ProcData<I, Y, R>>,
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
    Sv: Switch<ProcData<I, Y, R>>,
    I: Send + 'static,
    Y: Send + 'static,
    R: Send + 'static,
{
}

mod compile_tests {
    //! ```compile_fail
    //! use drone_cortexm::{
    //!     fib::Yielder,
    //!     sv,
    //!     sv::{SwitchBackService, SwitchContextService},
    //! };
    //! sv::pool! {
    //!     pool => SERVICES;
    //!     supervisor => pub Sv;
    //!     services => { SwitchContextService; SwitchBackService };
    //! }
    //! fn assert_send<T: Send>() {}
    //! fn main() {
    //!     assert_send::<Yielder<Sv, (), (), ()>>();
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! use drone_cortexm::{
    //!     fib::Yielder,
    //!     sv,
    //!     sv::{SwitchBackService, SwitchContextService},
    //! };
    //! sv::pool! {
    //!     pool => SERVICES;
    //!     supervisor => pub Sv;
    //!     services => { SwitchContextService; SwitchBackService };
    //! }
    //! fn assert_sync<T: Sync>() {}
    //! fn main() {
    //!     assert_sync::<Yielder<Sv, (), (), ()>>();
    //! }
    //! ```
}
