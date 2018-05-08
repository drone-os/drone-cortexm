//! Adapter pattern for transforming stackful synchronous subroutines into
//! fibers.

pub use drone_core::stack_adapter::*;

use drone_core::sv::SvCall;
use fib::{self, Fiber, FiberState};
use sv::{SwitchBackService, SwitchContextService};

/// A stack storage for the adapter `A`.
pub struct Stack<Sv, A>(AdapterFiber<Sv, A>)
where
  Sv: SvCall<SwitchBackService>,
  Sv: SvCall<SwitchContextService>,
  A: Adapter<Stack = Stack<Sv, A>, Context = Yielder<Sv, A>>;

/// A zero-sized type to make requests.
pub struct Yielder<Sv, A>(AdapterYielder<Sv, A>)
where
  Sv: SvCall<SwitchBackService>,
  Sv: SvCall<SwitchContextService>,
  A: Adapter<Stack = Stack<Sv, A>, Context = Yielder<Sv, A>>;

type AdapterFiber<Sv, A> = fib::FiberStack<
  Sv,
  In<<A as Adapter>::Cmd, <A as Adapter>::ReqRes>,
  Out<<A as Adapter>::Req, <A as Adapter>::CmdRes>,
  !,
  CmdLoopFn<Sv, A>,
>;

type AdapterYielder<Sv, A> = fib::Yielder<
  Sv,
  In<<A as Adapter>::Cmd, <A as Adapter>::ReqRes>,
  Out<<A as Adapter>::Req, <A as Adapter>::CmdRes>,
  !,
>;

type CmdLoopFn<Sv, A> =
  fn(In<<A as Adapter>::Cmd, <A as Adapter>::ReqRes>, AdapterYielder<Sv, A>)
    -> !;

impl<Sv, A> Stack<Sv, A>
where
  Sv: SvCall<SwitchBackService>,
  Sv: SvCall<SwitchContextService>,
  A: Adapter<Stack = Self, Context = Yielder<Sv, A>>,
{
  /// Creates a new `Stack`.
  ///
  /// # Panics
  ///
  /// If MPU not present.
  pub fn new() -> Self {
    unsafe { Self::new_with(fib::new_stack) }
  }

  /// Creates a new `Stack`.
  ///
  /// # Safety
  ///
  /// Unprotected from stack overflow.
  pub unsafe fn new_unchecked() -> Self {
    Self::new_with(fib::new_stack_unchecked)
  }

  fn cmd_loop(
    mut input: In<A::Cmd, A::ReqRes>,
    yielder: AdapterYielder<Sv, A>,
  ) -> ! {
    loop {
      input = yielder.stack_yield(Out::CmdRes(A::run_cmd(
        unsafe { input.into_cmd() },
        Yielder(yielder),
      )));
    }
  }

  unsafe fn new_with(
    f: unsafe fn(usize, CmdLoopFn<Sv, A>) -> AdapterFiber<Sv, A>,
  ) -> Self {
    Stack(f(A::STACK_SIZE, Self::cmd_loop))
  }
}

impl<Sv, A> Fiber for Stack<Sv, A>
where
  Sv: SvCall<SwitchBackService>,
  Sv: SvCall<SwitchContextService>,
  A: Adapter<Stack = Self, Context = Yielder<Sv, A>>,
{
  type Input = In<A::Cmd, A::ReqRes>;
  type Yield = Out<A::Req, A::CmdRes>;
  type Return = !;

  fn resume(
    &mut self,
    input: Self::Input,
  ) -> FiberState<Self::Yield, Self::Return> {
    self.0.resume(input)
  }
}

impl<Sv, A> Context<A::Req, A::ReqRes> for Yielder<Sv, A>
where
  Sv: SvCall<SwitchBackService>,
  Sv: SvCall<SwitchContextService>,
  A: Adapter<Stack = Stack<Sv, A>, Context = Self>,
{
  unsafe fn new() -> Self {
    Yielder(fib::Yielder::new())
  }

  fn req(&self, req: A::Req) -> A::ReqRes {
    unsafe { self.0.stack_yield(Out::Req(req)).into_req_res() }
  }
}
