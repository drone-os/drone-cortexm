//! This module provides interface to wrap a stackful synchronous code into an
//! asynchronous command loop.
//!
//! **NOTE** This module documentation should be viewed as a continuation of
//! [the `drone_core` documentation](drone_core::proc_loop).
//!
//! To provide an example, imagine a C library for FAT32 file system. Here is
//! how it could be wrapped:
//!
//! ```
//! # #![feature(naked_functions)]
//! # #![feature(never_type)]
//! use core::{future::Future, pin::Pin, slice};
//! use drone_core::ffi::{c_char, CString};
//! use drone_cortexm::{
//!     proc_loop::{self, Context as _, ProcLoop, Sess as _},
//!     sv,
//! };
//! use futures::prelude::*;
//!
//! use drone_cortexm::sv::{SwitchBackService, SwitchContextService};
//!
//! // Stackful fibers need a supervisor.
//! sv! {
//!     supervisor => pub Sv;
//!     array => SERVICES;
//!     services => {
//!         // These services are required for stackful fibers.
//!         SwitchContextService;
//!         SwitchBackService;
//!     }
//! }
//!
//! // Here is the library API.
//! extern "C" {
//!     fn f_read(name: *const c_char, buf: *mut u8, count: u32) -> u32;
//!     fn f_write(name: *const c_char, buf: *const u8, count: u32) -> u32;
//! }
//!
//! // The library is expecting to be linked with the following two function.
//!
//! #[no_mangle]
//! pub extern "C" fn disk_read(buf: *mut u8, sector: u32, count: u32) -> u32 {
//!     // We need to recreate the Yielder with correct type parameters.
//!     let yielder = unsafe { proc_loop::Yielder::<Sv, FatfsRes>::new() };
//!     // Redirect the request to the command loop. This call is blocking.
//!     let req_res = yielder.req(Req::DiskRead { buf, sector, count });
//!     // The result variant must correspond to the request variant.
//!     unsafe { req_res.disk_read }
//! }
//!
//! #[no_mangle]
//! pub extern "C" fn disk_write(buf: *const u8, sector: u32, count: u32) -> u32 {
//!     let yielder = unsafe { proc_loop::Yielder::<Sv, FatfsRes>::new() };
//!     let req_res = yielder.req(Req::DiskWrite { buf, sector, count });
//!     // The result variant must correspond to the request variant.
//!     unsafe { req_res.disk_write }
//! }
//!
//! // We need to map the two functions above to the corresponding functions below.
//!
//! pub async fn disk_read_async(buf: &mut [u8], sector: u32) -> u32 {
//!     // Serve `disk_read` asynchronously.
//!     unimplemented!()
//! }
//!
//! pub async fn disk_write_async(buf: &[u8], sector: u32) -> u32 {
//!     // Serve `disk_write` asynchronously.
//!     unimplemented!()
//! }
//!
//! // All possible commands. We can use only `'static` lifetimes here. That is
//! // why we use `*const str`, `*const [u8]`, `*mut [u8]` instead of `&str`,
//! // `&[u8]`, `&mut [u8]`.
//! pub enum Cmd {
//!     Read { name: *const str, buf: *mut [u8] },
//!     Write { name: *const str, buf: *const [u8] },
//! }
//!
//! // Results for each of the commands above.
//! pub union CmdRes {
//!     pub read: u32,
//!     pub write: u32,
//! }
//!
//! // All possible requests used by `disk_read` and `disk_write` functions above.
//! pub enum Req {
//!     DiskRead { buf: *mut u8, sector: u32, count: u32 },
//!     DiskWrite { buf: *const u8, sector: u32, count: u32 },
//! }
//!
//! // Results for each of the requests above.
//! pub union ReqRes {
//!     pub disk_read: u32,
//!     pub disk_write: u32,
//! }
//!
//! // The use of raw pointers requires this.
//! unsafe impl Send for Cmd {}
//! unsafe impl Send for Req {}
//!
//! // This type will never be instantiated. It is used only to define associated
//! // items with `ProcLoop` trait.
//! pub struct FatfsRes;
//!
//! // This is the type that implements the high-level API of our library.
//! pub struct FatfsSess<'sess>(&'sess mut proc_loop::Fiber<Sv, FatfsRes>);
//!
//! impl ProcLoop for FatfsRes {
//!     type Cmd = Cmd;
//!     type CmdRes = CmdRes;
//!     type Context = proc_loop::Yielder<Sv, FatfsRes>;
//!     type Req = Req;
//!     type ReqRes = ReqRes;
//!
//!     const STACK_SIZE: usize = 0x800;
//!
//!     fn run_cmd(cmd: Cmd, _context: Self::Context) -> CmdRes {
//!         match cmd {
//!             Cmd::Read { name, buf } => {
//!                 // Rebind lifetimes for the raw pointers.
//!                 let name = unsafe { &*buf };
//!                 let buf = unsafe { &mut *buf };
//!                 // Call the library function synchronously. This will block.
//!                 let read = unsafe {
//!                     f_read(
//!                         CString::new(name).unwrap().as_ptr(),
//!                         buf.as_mut_ptr(),
//!                         buf.len() as u32,
//!                     )
//!                 };
//!                 // The result variant must correspond to the command variant.
//!                 CmdRes { read }
//!             }
//!             Cmd::Write { name, buf } => {
//!                 let name = unsafe { &*buf };
//!                 let buf = unsafe { &*buf };
//!                 let write = unsafe {
//!                     f_write(
//!                         CString::new(name).unwrap().as_ptr(),
//!                         buf.as_ptr(),
//!                         buf.len() as u32,
//!                     )
//!                 };
//!                 // The result variant must correspond to the command variant.
//!                 CmdRes { write }
//!             }
//!         }
//!     }
//! }
//!
//! impl proc_loop::Sess for FatfsSess<'_> {
//!     type Error = !;
//!     type Fiber = proc_loop::Fiber<Sv, FatfsRes>;
//!     type ProcLoop = FatfsRes;
//!
//!     fn fib(&mut self) -> Pin<&mut Self::Fiber> {
//!         Pin::new(self.0)
//!     }
//!
//!     fn run_req(
//!         &mut self,
//!         req: <Self::ProcLoop as ProcLoop>::Req,
//!     ) -> Pin<
//!         Box<
//!             dyn Future<Output = Result<<Self::ProcLoop as ProcLoop>::ReqRes, Self::Error>>
//!                 + Send
//!                 + '_,
//!         >,
//!     > {
//!         match req {
//!             Req::DiskRead { buf, sector, count } => {
//!                 let slice = unsafe { slice::from_raw_parts_mut(buf, count as usize) };
//!                 Box::pin(disk_read_async(slice, sector).map(|disk_read| {
//!                     // The result variant must correspond to the request variant.
//!                     Ok(ReqRes { disk_read })
//!                 }))
//!             }
//!             Req::DiskWrite { buf, sector, count } => {
//!                 let slice = unsafe { slice::from_raw_parts(buf, count as usize) };
//!                 Box::pin(disk_write_async(slice, sector).map(|disk_write| {
//!                     // The result variant must correspond to the request variant.
//!                     Ok(ReqRes { disk_write })
//!                 }))
//!             }
//!         }
//!     }
//! }
//!
//! // The high-level API to our library.
//! impl FatfsSess<'_> {
//!     pub async fn read<'a>(&'a mut self, name: &'a str, buf: &'a mut [u8]) -> Result<u32, !> {
//!         let res = self.cmd(Cmd::Read { name, buf }).await?;
//!         // The result variant must correspond to the command variant.
//!         Ok(unsafe { res.read })
//!     }
//!
//!     pub async fn write<'a>(&'a mut self, name: &'a str, buf: &'a [u8]) -> Result<u32, !> {
//!         let res = self.cmd(Cmd::Write { name, buf }).await?;
//!         // The result variant must correspond to the command variant.
//!         Ok(unsafe { res.write })
//!     }
//! }
//!
//! // Here is how we use the defined command loop in asynchronous context.
//! # fn main() {
//! async {
//!     let mut fatfs_proc = proc_loop::Fiber::<Sv, FatfsRes>::new();
//!     let mut fatfs_sess = FatfsSess(&mut fatfs_proc);
//!     let mut buf = [0; 10];
//!     fatfs_sess.read("file1.txt", &mut buf).await?;
//!     fatfs_sess.write("file2.txt", b"hello there!\n").await?;
//!     Ok::<(), !>(())
//! };
//! # }
//! ```

#[doc(no_inline)]
pub use drone_core::proc_loop::*;

use crate::{
    fib::{self, FiberState},
    sv::{SvCall, SwitchBackService, SwitchContextService},
};
use core::pin::Pin;

type InnerYielder<Sv, T> = fib::Yielder<Sv, InnerIn<T>, InnerOut<T>, !>;
type InnerFiber<Sv, T> = fib::FiberProc<Sv, InnerIn<T>, InnerOut<T>, !, CmdLoop<Sv, T>>;
type InnerIn<T> = In<<T as ProcLoop>::Cmd, <T as ProcLoop>::ReqRes>;
type InnerOut<T> = Out<<T as ProcLoop>::Req, <T as ProcLoop>::CmdRes>;
type CmdLoop<Sv, T> =
    fn(In<<T as ProcLoop>::Cmd, <T as ProcLoop>::ReqRes>, InnerYielder<Sv, T>) -> !;

/// A wrapper for [`fib::FiberProc`] that runs the command loop `T`.
pub struct Fiber<Sv, T>(InnerFiber<Sv, T>)
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: ProcLoop<Context = Yielder<Sv, T>>;

/// Yielder for [`Fiber`]'s [`fib::FiberProc`].
pub struct Yielder<Sv, T>(InnerYielder<Sv, T>)
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: ProcLoop<Context = Self>;

#[allow(clippy::new_without_default)]
impl<Sv, T> Fiber<Sv, T>
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: ProcLoop<Context = Yielder<Sv, T>>,
{
    /// Creates a new command loop for `T`.
    ///
    /// # Panics
    ///
    /// If MPU is not present.
    pub fn new() -> Self {
        unsafe { Self::new_with(fib::new_proc) }
    }

    /// Creates a new command loop for `T`, without MPU.
    ///
    /// # Safety
    ///
    /// Unprotected from stack overflow.
    pub unsafe fn new_unchecked() -> Self {
        unsafe { Self::new_with(fib::new_proc_unchecked) }
    }

    unsafe fn new_with(f: unsafe fn(usize, CmdLoop<Sv, T>) -> InnerFiber<Sv, T>) -> Self {
        T::on_create();
        Self(unsafe { f(T::STACK_SIZE, Self::cmd_loop) })
    }

    fn cmd_loop(mut input: In<T::Cmd, T::ReqRes>, yielder: InnerYielder<Sv, T>) -> ! {
        T::on_enter();
        loop {
            input = yielder
                .proc_yield(Out::CmdRes(T::run_cmd(unsafe { input.into_cmd() }, Yielder(yielder))));
        }
    }
}

impl<Sv, T> Drop for Fiber<Sv, T>
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: ProcLoop<Context = Yielder<Sv, T>>,
{
    fn drop(&mut self) {
        T::on_drop();
    }
}

impl<Sv, T> fib::Fiber for Fiber<Sv, T>
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: ProcLoop<Context = Yielder<Sv, T>>,
{
    type Input = InnerIn<T>;
    type Return = !;
    type Yield = InnerOut<T>;

    #[inline]
    fn resume(mut self: Pin<&mut Self>, input: InnerIn<T>) -> FiberState<InnerOut<T>, !> {
        Pin::new(&mut self.0).resume(input)
    }
}

impl<Sv, T> Context<T::Req, T::ReqRes> for Yielder<Sv, T>
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: ProcLoop<Context = Self>,
{
    #[inline]
    unsafe fn new() -> Self {
        Self(unsafe { fib::Yielder::new() })
    }

    #[inline]
    fn req(self, req: T::Req) -> T::ReqRes {
        unsafe { self.0.proc_yield(Out::Req(req)).into_req_res() }
    }
}

impl<Sv, T> Clone for Yielder<Sv, T>
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: ProcLoop<Context = Self>,
{
    fn clone(&self) -> Self {
        unsafe { Self::new() }
    }
}

impl<Sv, T> Copy for Yielder<Sv, T>
where
    Sv: SvCall<SwitchBackService>,
    Sv: SvCall<SwitchContextService>,
    T: ProcLoop<Context = Self>,
{
}
