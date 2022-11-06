use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::{
    braced, parenthesized, parse_macro_input, Attribute, ExprPath, Ident, LitInt, Token, Visibility,
};

struct Input {
    thr: Thr,
    local: Local,
    index: Index,
    vtable: Vtable,
    init: Init,
    sv: Option<Sv>,
    threads: Threads,
}

struct Thr {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    tokens: TokenStream2,
}

struct Local {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    tokens: TokenStream2,
}

struct Index {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
}

struct Vtable {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
}

struct Init {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
}

struct Sv {
    path: ExprPath,
}

struct Threads {
    threads: Vec<Thread>,
}

enum Thread {
    Exception(ThreadSpec),
    Interrupt(u16, ThreadSpec),
}

struct ThreadSpec {
    attrs: Vec<Attribute>,
    vis: Visibility,
    kind: ThreadKind,
    ident: Ident,
}

enum ThreadKind {
    Inner,
    Outer(ExprPath),
    Naked(ExprPath),
}

impl Parse for Input {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut thr = None;
        let mut local = None;
        let mut index = None;
        let mut vtable = None;
        let mut init = None;
        let mut sv = None;
        let mut threads = None;
        while !input.is_empty() {
            let attrs = input.call(Attribute::parse_outer)?;
            let ident = input.parse::<Ident>()?;
            input.parse::<Token![=>]>()?;
            if ident == "thread" {
                if thr.is_none() {
                    thr = Some(Thr::parse(input, attrs)?);
                } else {
                    return Err(input.error("multiple `thread` specifications"));
                }
            } else if ident == "local" {
                if local.is_none() {
                    local = Some(Local::parse(input, attrs)?);
                } else {
                    return Err(input.error("multiple `local` specifications"));
                }
            } else if ident == "index" {
                if index.is_none() {
                    index = Some(Index::parse(input, attrs)?);
                } else {
                    return Err(input.error("multiple `index` specifications"));
                }
            } else if ident == "vtable" {
                if vtable.is_none() {
                    vtable = Some(Vtable::parse(input, attrs)?);
                } else {
                    return Err(input.error("multiple `vtable` specifications"));
                }
            } else if ident == "init" {
                if init.is_none() {
                    init = Some(Init::parse(input, attrs)?);
                } else {
                    return Err(input.error("multiple `init` specifications"));
                }
            } else if attrs.is_empty() && ident == "supervisor" {
                if sv.is_none() {
                    sv = Some(input.parse()?);
                } else {
                    return Err(input.error("multiple `sv` specifications"));
                }
            } else if attrs.is_empty() && ident == "threads" {
                if threads.is_none() {
                    threads = Some(input.parse()?);
                } else {
                    return Err(input.error("multiple `threads` specifications"));
                }
            } else {
                return Err(input.error(format!("unknown key: `{}`", ident)));
            }
            if !input.is_empty() {
                input.parse::<Token![;]>()?;
            }
        }
        Ok(Self {
            thr: thr.ok_or_else(|| input.error("missing `thread` specification"))?,
            local: local.ok_or_else(|| input.error("missing `local` specification"))?,
            index: index.ok_or_else(|| input.error("missing `index` specification"))?,
            vtable: vtable.ok_or_else(|| input.error("missing `vtable` specification"))?,
            init: init.ok_or_else(|| input.error("missing `init` specification"))?,
            sv,
            threads: threads.ok_or_else(|| input.error("missing `threads` specification"))?,
        })
    }
}

impl Thr {
    fn parse(input: ParseStream<'_>, attrs: Vec<Attribute>) -> Result<Self> {
        let vis = input.parse()?;
        let ident = input.parse()?;
        let input2;
        braced!(input2 in input);
        let tokens = input2.parse()?;
        Ok(Self { attrs, vis, ident, tokens })
    }
}

impl Local {
    fn parse(input: ParseStream<'_>, attrs: Vec<Attribute>) -> Result<Self> {
        let vis = input.parse()?;
        let ident = input.parse()?;
        let input2;
        braced!(input2 in input);
        let tokens = input2.parse()?;
        Ok(Self { attrs, vis, ident, tokens })
    }
}

impl Index {
    fn parse(input: ParseStream<'_>, attrs: Vec<Attribute>) -> Result<Self> {
        let vis = input.parse()?;
        let ident = input.parse()?;
        Ok(Self { attrs, vis, ident })
    }
}

impl Vtable {
    fn parse(input: ParseStream<'_>, attrs: Vec<Attribute>) -> Result<Self> {
        let vis = input.parse()?;
        let ident = input.parse()?;
        Ok(Self { attrs, vis, ident })
    }
}

impl Init {
    fn parse(input: ParseStream<'_>, attrs: Vec<Attribute>) -> Result<Self> {
        let vis = input.parse()?;
        let ident = input.parse()?;
        Ok(Self { attrs, vis, ident })
    }
}

impl Parse for Sv {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let path = input.parse()?;
        Ok(Self { path })
    }
}

impl Parse for Threads {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let input2;
        braced!(input2 in input);
        let mut threads = Vec::new();
        while !input2.is_empty() {
            let attrs = input2.call(Attribute::parse_outer)?;
            let ident = input2.parse::<Ident>()?;
            input2.parse::<Token![=>]>()?;
            if attrs.is_empty() && ident == "exceptions" {
                let input3;
                braced!(input3 in input2);
                while !input3.is_empty() {
                    let attrs = input3.call(Attribute::parse_outer)?;
                    let vis = input3.parse()?;
                    let kind = input3.parse()?;
                    let ident = input3.parse()?;
                    threads.push(Thread::Exception(ThreadSpec { attrs, vis, kind, ident }));
                    if !input3.is_empty() {
                        input3.parse::<Token![;]>()?;
                    }
                }
            } else if attrs.is_empty() && ident == "interrupts" {
                let input3;
                braced!(input3 in input2);
                while !input3.is_empty() {
                    let attrs = input3.call(Attribute::parse_outer)?;
                    let num = input3.parse::<LitInt>()?.base10_parse()?;
                    input3.parse::<Token![:]>()?;
                    let vis = input3.parse()?;
                    let kind = input3.parse()?;
                    let ident = input3.parse()?;
                    threads.push(Thread::Interrupt(num, ThreadSpec { attrs, vis, kind, ident }));
                    if !input3.is_empty() {
                        input3.parse::<Token![;]>()?;
                    }
                }
            } else {
                return Err(input2.error(format!("Unexpected ident `{}`", ident)));
            }
            if !input2.is_empty() {
                input2.parse::<Token![;]>()?;
            }
        }
        Ok(Self { threads })
    }
}

impl Parse for ThreadKind {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        match input.fork().parse::<Ident>() {
            Ok(ident) if ident == "outer" => {
                input.parse::<Ident>()?;
                let input2;
                parenthesized!(input2 in input);
                let path = input2.parse()?;
                Ok(Self::Outer(path))
            }
            Ok(ident) if ident == "naked" => {
                input.parse::<Ident>()?;
                let input2;
                parenthesized!(input2 in input);
                let path = input2.parse()?;
                Ok(Self::Naked(path))
            }
            _ => Ok(Self::Inner),
        }
    }
}

pub fn proc_macro(input: TokenStream) -> TokenStream {
    let Input { thr, local, index, vtable, init, sv, threads } = parse_macro_input!(input as Input);
    let Threads { threads } = threads;
    let (threads, naked_threads) = partition_threads(threads);
    let def_thr_pool = def_thr_pool(&thr, &local, &index, &threads);
    let def_vtable = def_vtable(&thr, &vtable, &threads, &naked_threads);
    let def_init = def_init(&index, &init);
    let thr_tokens =
        threads.iter().flat_map(|thread| def_thr_token(&sv, thread)).collect::<Vec<_>>();
    quote! {
        #def_thr_pool
        #def_vtable
        #def_init
        #(#thr_tokens)*
        ::drone_cortexm::reg::claim!("scb_ccr");
        ::drone_cortexm::reg::claim!("mpu_type");
        ::drone_cortexm::reg::claim!("mpu_ctrl");
        ::drone_cortexm::reg::claim!("mpu_rnr");
        ::drone_cortexm::reg::claim!("mpu_rbar");
        ::drone_cortexm::reg::claim!("mpu_rasr");
    }
    .into()
}

fn partition_threads(threads: Vec<Thread>) -> (Vec<Thread>, Vec<Thread>) {
    threads.into_iter().partition(|thread| match thread {
        Thread::Exception(spec) | Thread::Interrupt(_, spec) => {
            let ThreadSpec { kind, .. } = spec;
            match kind {
                ThreadKind::Inner | ThreadKind::Outer(_) => true,
                ThreadKind::Naked(_) => false,
            }
        }
    })
}

#[allow(clippy::too_many_lines, clippy::cast_possible_truncation)]
fn def_vtable(
    thr: &Thr,
    vtable: &Vtable,
    threads: &[Thread],
    naked_threads: &[Thread],
) -> TokenStream2 {
    let Thr { ident: thr_ident, .. } = thr;
    let Vtable { attrs: vtable_attrs, vis: vtable_vis, ident: vtable_ident } = vtable;
    let resume = format_ident!("{}_resume", thr_ident.to_string().to_snake_case());
    let mut tokens = Vec::new();
    let mut vtable_tokens = Vec::new();
    let mut vtable_ctor_tokens = Vec::new();
    let mut vtable_ctor_default_tokens = Vec::new();
    let mut resume_tokens = None;
    for (idx, thread) in threads
        .iter()
        .enumerate()
        .map(|(idx, thread)| (Some(idx as u16), thread))
        .chain(naked_threads.iter().map(|thread| (None, thread)))
    {
        match thread {
            Thread::Exception(spec) | Thread::Interrupt(_, spec) => {
                let ThreadSpec { kind, ident, .. } = spec;
                let field_ident = format_ident!("{}", ident);
                match kind {
                    ThreadKind::Inner => {
                        let ident = format_ident!("thr_handler_{}", idx.unwrap());
                        resume_tokens.get_or_insert_with(|| {
                            quote! {
                                #[inline(never)]
                                unsafe fn #resume(thr: &#thr_ident) {
                                    unsafe { ::drone_core::thr::Thread::resume(thr) };
                                }
                            }
                        });
                        tokens.push(quote! {
                            unsafe extern "C" fn #ident() {
                                unsafe { <#thr_ident as ::drone_core::thr::Thread>::call(#idx, #resume) };
                            }
                        });
                        vtable_ctor_tokens.push(quote! {
                            #field_ident: ::core::option::Option::Some(#ident)
                        });
                    }
                    ThreadKind::Outer(path) => {
                        let ident = format_ident!("thr_handler_{}_outer", idx.unwrap());
                        tokens.push(quote! {
                            unsafe extern "C" fn #ident() {
                                unsafe { <#thr_ident as ::drone_core::thr::Thread>::call(#idx, #path) };
                            }
                        });
                        vtable_ctor_tokens.push(quote! {
                            #field_ident: ::core::option::Option::Some(#ident)
                        });
                    }
                    ThreadKind::Naked(path) => {
                        vtable_ctor_tokens.push(quote! {
                            #field_ident: ::core::option::Option::Some(#path)
                        });
                    }
                }
                if let Thread::Interrupt(num, _) = thread {
                    let num = *num as usize;
                    if vtable_tokens.len() < num + 1 {
                        vtable_tokens.resize(num + 1, None);
                    }
                    vtable_tokens[num] = Some(quote! {
                        #field_ident: ::core::option::Option<unsafe extern "C" fn()>
                    });
                    vtable_ctor_default_tokens.push(quote! {
                        #field_ident: ::core::option::Option::None
                    });
                }
            }
        }
    }
    let vtable_tokens = vtable_tokens
        .into_iter()
        .enumerate()
        .map(|(i, tokens)| {
            tokens.unwrap_or_else(|| {
                let field_ident = format_ident!("_int{}", i);
                vtable_ctor_default_tokens.push(quote! {
                    #field_ident: ::core::option::Option::None
                });
                quote! {
                    #field_ident: ::core::option::Option<unsafe extern "C" fn()>
                }
            })
        })
        .collect::<Vec<_>>();
    quote! {
        #(#vtable_attrs)*
        #[repr(C)]
        #[allow(dead_code)]
        #vtable_vis struct #vtable_ident {
            reset: unsafe extern "C" fn() -> !,
            nmi: ::core::option::Option<unsafe extern "C" fn()>,
            hard_fault: ::core::option::Option<unsafe extern "C" fn()>,
            mem_manage: ::core::option::Option<unsafe extern "C" fn()>,
            bus_fault: ::core::option::Option<unsafe extern "C" fn()>,
            usage_fault: ::core::option::Option<unsafe extern "C" fn()>,
            #[cfg(feature = "security-extension")]
            secure_fault: ::core::option::Option<unsafe extern "C" fn()>,
            #[cfg(not(feature = "security-extension"))]
            _reserved0: [usize; 1],
            _reserved1: [usize; 3],
            sv_call: ::core::option::Option<unsafe extern "C" fn()>,
            debug: ::core::option::Option<unsafe extern "C" fn()>,
            _reserved2: [usize; 1],
            pend_sv: ::core::option::Option<unsafe extern "C" fn()>,
            sys_tick: ::core::option::Option<unsafe extern "C" fn()>,
            #(#vtable_tokens),*
        }

        impl #vtable_ident {
            /// Creates a new vector table.
            pub const fn new(reset: unsafe extern "C" fn() -> !) -> Self {
                Self {
                    #(#vtable_ctor_tokens,)*
                    ..Self {
                        reset,
                        nmi: ::core::option::Option::None,
                        hard_fault: ::core::option::Option::None,
                        mem_manage: ::core::option::Option::None,
                        bus_fault: ::core::option::Option::None,
                        usage_fault: ::core::option::Option::None,
                        #[cfg(feature = "security-extension")]
                        secure_fault: ::core::option::Option::None,
                        #[cfg(not(feature = "security-extension"))]
                        _reserved0: [0; 1],
                        _reserved1: [0; 3],
                        sv_call: ::core::option::Option::None,
                        debug: ::core::option::Option::None,
                        _reserved2: [0; 1],
                        pend_sv: ::core::option::Option::None,
                        sys_tick: ::core::option::Option::None,
                        #(#vtable_ctor_default_tokens,)*
                    }
                }
            }

            /// Relocates the vector table to the location pointed by `&mut self`.
            ///
            /// # Safety
            ///
            /// The address of `self` minus 4 bytes must be properly aligned
            /// according to the number of implemented bits in the `VTOR`
            /// register.
            ///
            /// The function rewrites contents of VTOR register without taking
            /// into account register tokens.
            pub unsafe fn relocate(&mut self) {
                unsafe {
                    ::drone_cortexm::thr::relocate_vtable(
                        (self as *mut Self).cast(),
                        ::core::mem::size_of::<Self>(),
                    );
                }
            }
        }

        #resume_tokens
        #(#tokens)*
    }
}

fn def_init(index: &Index, init: &Init) -> TokenStream2 {
    let Init { attrs: init_attrs, vis: init_vis, ident: init_ident } = init;
    let Index { ident: index_ident, .. } = index;
    quote! {
        #(#init_attrs)*
        #init_vis struct #init_ident {
            __priv: (),
        }

        unsafe impl ::drone_core::token::Token for #init_ident {
            #[inline]
            unsafe fn take() -> Self {
                Self {
                    __priv: (),
                }
            }
        }

        unsafe impl ::drone_cortexm::thr::ThrsInitToken for #init_ident {
            type ThrTokens = #index_ident;
        }
    }
}

fn def_thr_pool(thr: &Thr, local: &Local, index: &Index, threads: &[Thread]) -> TokenStream2 {
    let Thr { attrs: thr_attrs, vis: thr_vis, ident: thr_ident, tokens: thr_tokens } = thr;
    let Local { attrs: local_attrs, vis: local_vis, ident: local_ident, tokens: local_tokens } =
        local;
    let Index { attrs: index_attrs, vis: index_vis, ident: index_ident } = index;
    let mut threads_tokens = Vec::new();
    for thread in threads {
        match thread {
            Thread::Exception(spec) | Thread::Interrupt(_, spec) => {
                let ThreadSpec { attrs, vis, ident, .. } = spec;
                threads_tokens.push(quote! {
                    #(#attrs)* #vis #ident
                });
            }
        }
    }
    quote! {
        ::drone_core::thr::pool! {
            #(#thr_attrs)*
            thread => #thr_vis #thr_ident { #thr_tokens };

            #(#local_attrs)*
            local => #local_vis #local_ident { #local_tokens };

            #(#index_attrs)*
            index => #index_vis #index_ident;

            threads => {
                #(#threads_tokens;)*
            };
        }
    }
}

fn def_thr_token(sv: &Option<Sv>, thread: &Thread) -> Vec<TokenStream2> {
    let mut tokens = Vec::new();
    match thread {
        Thread::Exception(spec) | Thread::Interrupt(_, spec) => {
            let ThreadSpec { kind, ident, .. } = spec;
            match kind {
                ThreadKind::Inner | ThreadKind::Outer(_) => {
                    let struct_ident = format_ident!("{}", ident.to_string().to_upper_camel_case());
                    if let Some(Sv { path: sv_path }) = sv {
                        tokens.push(quote! {
                            impl ::drone_cortexm::thr::ThrSv for #struct_ident {
                                type Sv = #sv_path;
                            }
                        });
                    }
                    if let Thread::Interrupt(num, _) = thread {
                        let nvic_block = format_ident!("NvicBlock{}", num / 32);
                        tokens.push(quote! {
                            impl ::drone_cortexm::thr::IntToken for #struct_ident {
                                type NvicBlock = ::drone_cortexm::map::thr::#nvic_block;

                                const INT_NUM: u16 = #num;
                            }

                            impl ::drone_core::thr::ThrExec for #struct_ident {
                                #[inline]
                                fn wakeup(self) {
                                    unsafe {
                                        <Self as ::drone_cortexm::thr::IntToken>::wakeup_unchecked();
                                    }
                                }

                                #[inline]
                                fn waker(self) -> ::core::task::Waker {
                                    unsafe {
                                        <Self as ::drone_cortexm::thr::IntToken>::waker_unchecked()
                                    }
                                }
                            }
                        });
                    }
                }
                ThreadKind::Naked(_) => {}
            }
        }
    }
    tokens
}
