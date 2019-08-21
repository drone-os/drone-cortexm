//! Machinery for wrapping stackful synchronous code into stackless asynchronous
//! command loop.

pub use drone_core::stack_loop::{Context, In, Out, StackLoop, StackLoopSess};

use crate::{
    fib::{self, Fiber, FiberState},
    sv::{SvCall, SwitchBackService, SwitchContextService},
};
use core::pin::Pin;

type CmdLoop<Sv, T> =
    fn(In<<T as StackLoop>::Cmd, <T as StackLoop>::ReqRes>, InnerYielder<Sv, T>) -> !;

type InnerYielder<Sv, T> = fib::Yielder<Sv, InnerIn<T>, InnerOut<T>, !>;

type InnerFiber<Sv, T> = fib::FiberStack<Sv, InnerIn<T>, InnerOut<T>, !, CmdLoop<Sv, T>>;

type InnerIn<T> = In<<T as StackLoop>::Cmd, <T as StackLoop>::ReqRes>;

type InnerOut<T> = Out<<T as StackLoop>::Req, <T as StackLoop>::CmdRes>;

/// A stackful fiber that runs the command loop.
pub struct StackLoopFiber<Sv, T>(InnerFiber<Sv, T>)
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: StackLoop<Context = Yielder<Sv, T>>;

/// A yielder from [`StackLoopFiber`].
pub struct Yielder<Sv, T>(InnerYielder<Sv, T>)
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: StackLoop<Context = Self>;

#[allow(clippy::new_without_default)]
impl<Sv, T> StackLoopFiber<Sv, T>
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: StackLoop<Context = Yielder<Sv, T>>,
{
    /// Creates a new [`StackLoopFiber`].
    ///
    /// # Panics
    ///
    /// * If MPU not present.
    /// * If the adapter is singleton, and a `StackLoopFiber` instance already
    ///   exists.
    pub fn new() -> Self {
        unsafe { Self::new_with(fib::new_stack) }
    }

    /// Creates a new [`StackLoopFiber`].
    ///
    /// # Panics
    ///
    /// * If the adapter is singleton, and a `StackLoopFiber` instance already
    ///   exists.
    ///
    /// # Safety
    ///
    /// Unprotected from stack overflow.
    pub unsafe fn new_unchecked() -> Self {
        Self::new_with(fib::new_stack_unchecked)
    }

    unsafe fn new_with(f: unsafe fn(usize, CmdLoop<Sv, T>) -> InnerFiber<Sv, T>) -> Self {
        T::on_create();
        Self(f(T::STACK_SIZE, Self::cmd_loop))
    }

    fn cmd_loop(mut input: In<T::Cmd, T::ReqRes>, yielder: InnerYielder<Sv, T>) -> ! {
        T::on_enter();
        loop {
            input = yielder.stack_yield(Out::CmdRes(T::run_cmd(
                unsafe { input.into_cmd() },
                Yielder(yielder),
            )));
        }
    }
}

impl<Sv, T> Drop for StackLoopFiber<Sv, T>
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: StackLoop<Context = Yielder<Sv, T>>,
{
    fn drop(&mut self) {
        T::on_drop();
    }
}

impl<Sv, T> Fiber for StackLoopFiber<Sv, T>
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: StackLoop<Context = Yielder<Sv, T>>,
{
    type Input = InnerIn<T>;
    type Yield = InnerOut<T>;
    type Return = !;

    #[inline]
    fn resume(mut self: Pin<&mut Self>, input: InnerIn<T>) -> FiberState<InnerOut<T>, !> {
        Pin::new(&mut self.0).resume(input)
    }
}

impl<Sv, T> Context<T::Req, T::ReqRes> for Yielder<Sv, T>
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: StackLoop<Context = Self>,
{
    #[inline]
    unsafe fn new() -> Self {
        Self(fib::Yielder::new())
    }

    #[inline]
    fn req(&self, req: T::Req) -> T::ReqRes {
        unsafe { self.0.stack_yield(Out::Req(req)).into_req_res() }
    }
}
