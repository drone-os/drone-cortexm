use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Attribute, ExprPath, Ident, LitInt, Token, Visibility,
};

struct Vtable {
    thr: ExprPath,
    sv: Option<ExprPath>,
    vtable_attrs: Vec<Attribute>,
    vtable_vis: Visibility,
    vtable_ident: Ident,
    handlers_attrs: Vec<Attribute>,
    handlers_vis: Visibility,
    handlers_ident: Ident,
    index_attrs: Vec<Attribute>,
    index_vis: Visibility,
    index_ident: Ident,
    init_attrs: Vec<Attribute>,
    init_vis: Visibility,
    init_ident: Ident,
    array_attrs: Vec<Attribute>,
    array_vis: Visibility,
    array_ident: Ident,
    excs: Vec<Exc>,
    ints: Vec<Int>,
}

struct Exc {
    attrs: Vec<Attribute>,
    mode: Mode,
    ident: Ident,
}

struct Int {
    num: LitInt,
    exc: Exc,
}

enum Mode {
    Thread(Visibility),
    Extern(Visibility),
    Fn,
}

impl Parse for Vtable {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        input.parse::<Token![use]>()?;
        let thr = input.parse()?;
        input.parse::<Token![;]>()?;
        let sv = if input.peek(Token![use]) {
            input.parse::<Token![use]>()?;
            let sv = input.parse()?;
            input.parse::<Token![;]>()?;
            Some(sv)
        } else {
            None
        };
        let vtable_attrs = input.call(Attribute::parse_outer)?;
        let vtable_vis = input.parse()?;
        input.parse::<Token![struct]>()?;
        let vtable_ident = input.parse()?;
        input.parse::<Token![;]>()?;
        let handlers_attrs = input.call(Attribute::parse_outer)?;
        let handlers_vis = input.parse()?;
        input.parse::<Token![struct]>()?;
        let handlers_ident = input.parse()?;
        input.parse::<Token![;]>()?;
        let index_attrs = input.call(Attribute::parse_outer)?;
        let index_vis = input.parse()?;
        input.parse::<Token![struct]>()?;
        let index_ident = input.parse()?;
        input.parse::<Token![;]>()?;
        let init_attrs = input.call(Attribute::parse_outer)?;
        let init_vis = input.parse()?;
        input.parse::<Token![struct]>()?;
        let init_ident = input.parse()?;
        input.parse::<Token![;]>()?;
        let array_attrs = input.call(Attribute::parse_outer)?;
        let array_vis = input.parse()?;
        input.parse::<Token![static]>()?;
        let array_ident = input.parse()?;
        input.parse::<Token![;]>()?;
        let mut excs = Vec::new();
        while input.fork().parse::<Exc>().is_ok() {
            excs.push(input.parse()?);
        }
        let mut ints = Vec::new();
        while !input.is_empty() {
            ints.push(input.parse()?);
        }
        Ok(Self {
            thr,
            sv,
            vtable_attrs,
            vtable_vis,
            vtable_ident,
            handlers_attrs,
            handlers_vis,
            handlers_ident,
            index_attrs,
            index_vis,
            index_ident,
            init_attrs,
            init_vis,
            init_ident,
            array_attrs,
            array_vis,
            array_ident,
            excs,
            ints,
        })
    }
}

impl Parse for Exc {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let mode = input.parse()?;
        let ident = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(Self { attrs, mode, ident })
    }
}

impl Parse for Int {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let mode = input.parse()?;
        let num = input.parse()?;
        input.parse::<Token![:]>()?;
        let ident = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(Self { num, exc: Exc { attrs, mode, ident } })
    }
}

impl Parse for Mode {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(Token![fn]) {
            input.parse::<Token![fn]>()?;
            Ok(Self::Fn)
        } else {
            let vis = input.parse::<Visibility>()?;
            if input.parse::<Option<Token![extern]>>()?.is_some() {
                Ok(Self::Extern(vis))
            } else {
                Ok(Self::Thread(vis))
            }
        }
    }
}

#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
pub fn proc_macro(input: TokenStream) -> TokenStream {
    let Vtable {
        thr,
        sv,
        vtable_attrs,
        vtable_vis,
        vtable_ident,
        handlers_attrs,
        handlers_vis,
        handlers_ident,
        index_attrs,
        index_vis,
        index_ident,
        init_attrs,
        init_vis,
        init_ident,
        array_attrs,
        array_vis,
        array_ident,
        excs,
        ints,
    } = parse_macro_input!(input as Vtable);
    let int_len =
        ints.iter().map(|int| int.num.base10_parse::<usize>().unwrap() + 1).max().unwrap_or(0);
    let mut vtable_tokens = vec![None; int_len];
    let mut vtable_ctor_default_tokens = Vec::new();
    let mut vtable_ctor_tokens = Vec::new();
    let mut handlers_tokens = Vec::new();
    let mut index_tokens = Vec::new();
    let mut index_ctor_tokens = Vec::new();
    let mut array_tokens = Vec::new();
    let mut thr_tokens = Vec::new();
    let mut thr_counter = 1;
    for exc in excs {
        let (_, struct_ident) = gen_exc(
            &exc,
            &thr,
            sv.as_ref(),
            &mut thr_counter,
            &mut vtable_ctor_tokens,
            &mut handlers_tokens,
            &mut index_tokens,
            &mut index_ctor_tokens,
            &mut array_tokens,
            &mut thr_tokens,
        );
        if let Some(struct_ident) = struct_ident {
            let int_trait = format_ident!("Int{}", struct_ident);
            thr_tokens.push(quote! {
                impl #int_trait for #struct_ident {}
            });
        }
    }
    for Int { num, exc } in ints {
        let (field_ident, struct_ident) = gen_exc(
            &exc,
            &thr,
            sv.as_ref(),
            &mut thr_counter,
            &mut vtable_ctor_tokens,
            &mut handlers_tokens,
            &mut index_tokens,
            &mut index_ctor_tokens,
            &mut array_tokens,
            &mut thr_tokens,
        );
        if let Some(struct_ident) = struct_ident {
            let int_trait = format_ident!("Int{}", num.base10_digits());
            let nvic_block =
                format_ident!("NvicBlock{}", num.base10_parse::<usize>().unwrap() / 32);
            thr_tokens.push(quote! {
                impl ::drone_cortex_m::thr::IntToken for #struct_ident {
                    type NvicBlock = ::drone_cortex_m::map::thr::#nvic_block;

                    const INT_NUM: usize = #num;
                }

                impl #int_trait for #struct_ident {}
            });
        }
        vtable_tokens[num.base10_parse::<usize>().unwrap()] = Some(quote! {
            #field_ident: Option<::drone_cortex_m::thr::vtable::Handler>
        });
        vtable_ctor_default_tokens.push(quote! {
            #field_ident: None
        });
    }
    let vtable_tokens = vtable_tokens
        .into_iter()
        .enumerate()
        .map(|(i, tokens)| {
            tokens.unwrap_or_else(|| {
                let int_ident = format_ident!("_int{}", i);
                vtable_ctor_default_tokens.push(quote! {
                    #int_ident: None
                });
                quote! {
                    #int_ident: Option<::drone_cortex_m::thr::vtable::Handler>
                }
            })
        })
        .collect::<Vec<_>>();

    thr_tokens.push(quote! {
        /// Reset thread token.
        #[derive(Clone, Copy)]
        pub struct Reset(());

        unsafe impl ::drone_core::token::Token for Reset {
            #[inline]
            unsafe fn take() -> Self {
                Reset(())
            }
        }

        unsafe impl ::drone_core::thr::ThrToken for Reset {
            type Thr = #thr;

            const THR_NUM: usize = 0;
        }
    });
    if let Some(sv) = sv {
        thr_tokens.push(quote! {
            impl ::drone_cortex_m::thr::ThrSv for Reset {
                type Sv = #sv;
            }
        });
    }
    let expanded = quote! {
        #(#vtable_attrs)*
        #[allow(dead_code)]
        #vtable_vis struct #vtable_ident {
            reset: ::drone_cortex_m::thr::vtable::ResetHandler,
            nmi: Option<::drone_cortex_m::thr::vtable::Handler>,
            hard_fault: Option<::drone_cortex_m::thr::vtable::Handler>,
            mem_manage: Option<::drone_cortex_m::thr::vtable::Handler>,
            bus_fault: Option<::drone_cortex_m::thr::vtable::Handler>,
            usage_fault: Option<::drone_cortex_m::thr::vtable::Handler>,
            #[cfg(all(
                feature = "security-extension",
                any(
                    cortex_m_core = "cortex_m33_r0p2",
                    cortex_m_core = "cortex_m33_r0p3",
                    cortex_m_core = "cortex_m33_r0p4",
                    cortex_m_core = "cortex_m33f_r0p2",
                    cortex_m_core = "cortex_m33f_r0p3",
                    cortex_m_core = "cortex_m33f_r0p4",
                )
            ))]
            secure_fault: Option<::drone_cortex_m::thr::vtable::Handler>,
            #[cfg(not(all(
                feature = "security-extension",
                any(
                    cortex_m_core = "cortex_m33_r0p2",
                    cortex_m_core = "cortex_m33_r0p3",
                    cortex_m_core = "cortex_m33_r0p4",
                    cortex_m_core = "cortex_m33f_r0p2",
                    cortex_m_core = "cortex_m33f_r0p3",
                    cortex_m_core = "cortex_m33f_r0p4",
                )
            )))]
            _reserved0: [::drone_cortex_m::thr::vtable::Reserved; 1],
            _reserved1: [::drone_cortex_m::thr::vtable::Reserved; 3],
            sv_call: Option<::drone_cortex_m::thr::vtable::Handler>,
            debug: Option<::drone_cortex_m::thr::vtable::Handler>,
            _reserved2: [::drone_cortex_m::thr::vtable::Reserved; 1],
            pend_sv: Option<::drone_cortex_m::thr::vtable::Handler>,
            sys_tick: Option<::drone_cortex_m::thr::vtable::Handler>,
            #(#vtable_tokens),*
        }

        #(#handlers_attrs)*
        #handlers_vis struct #handlers_ident {
            /// Reset exception handler.
            pub reset: ::drone_cortex_m::thr::vtable::ResetHandler,
            #(#handlers_tokens),*
        }

        #(#index_attrs)*
        #index_vis struct #index_ident {
            /// Reset thread token.
            pub reset: Reset,
            #(#index_tokens),*
        }

        #(#init_attrs)*
        #init_vis struct #init_ident {
            __priv: (),
        }

        #(#array_attrs)*
        #array_vis static mut #array_ident: [#thr; #thr_counter] = [
            #thr::new(0),
            #(#array_tokens),*
        ];

        impl #vtable_ident {
            /// Creates a new vector table.
            pub const fn new(handlers: #handlers_ident) -> Self {
                Self {
                    #(#vtable_ctor_tokens,)*
                    ..Self {
                        reset: handlers.reset,
                        nmi: None,
                        hard_fault: None,
                        mem_manage: None,
                        bus_fault: None,
                        usage_fault: None,
                        #[cfg(all(
                            feature = "security-extension",
                            any(
                                cortex_m_core = "cortex_m33_r0p2",
                                cortex_m_core = "cortex_m33_r0p3",
                                cortex_m_core = "cortex_m33_r0p4",
                                cortex_m_core = "cortex_m33f_r0p2",
                                cortex_m_core = "cortex_m33f_r0p3",
                                cortex_m_core = "cortex_m33f_r0p4",
                            )
                        ))]
                        secure_fault: None,
                        #[cfg(not(all(
                            feature = "security-extension",
                            any(
                                cortex_m_core = "cortex_m33_r0p2",
                                cortex_m_core = "cortex_m33_r0p3",
                                cortex_m_core = "cortex_m33_r0p4",
                                cortex_m_core = "cortex_m33f_r0p2",
                                cortex_m_core = "cortex_m33f_r0p3",
                                cortex_m_core = "cortex_m33f_r0p4",
                            )
                        )))]
                        _reserved0: [::drone_cortex_m::thr::vtable::Reserved::Vector; 1],
                        _reserved1: [::drone_cortex_m::thr::vtable::Reserved::Vector; 3],
                        sv_call: None,
                        debug: None,
                        _reserved2: [::drone_cortex_m::thr::vtable::Reserved::Vector; 1],
                        pend_sv: None,
                        sys_tick: None,
                        #(#vtable_ctor_default_tokens,)*
                    }
                }
            }
        }

        unsafe impl ::drone_core::token::Token for #index_ident {
            #[inline]
            unsafe fn take() -> Self {
                Self {
                    reset: ::drone_core::token::Token::take(),
                    #(#index_ctor_tokens),*
                }
            }
        }

        unsafe impl ::drone_cortex_m::thr::ThrTokens for #index_ident {}

        unsafe impl ::drone_core::token::Token for #init_ident {
            #[inline]
            unsafe fn take() -> Self {
                Self {
                    __priv: (),
                }
            }
        }

        unsafe impl ::drone_cortex_m::thr::ThrsInitToken for #init_ident {
            type ThrTokens = #index_ident;
        }

        #(#thr_tokens)*

        ::drone_cortex_m::reg::assert_taken!("scb_ccr");
        ::drone_cortex_m::reg::assert_taken!("mpu_type");
        ::drone_cortex_m::reg::assert_taken!("mpu_ctrl");
        ::drone_cortex_m::reg::assert_taken!("mpu_rnr");
        ::drone_cortex_m::reg::assert_taken!("mpu_rbar");
        ::drone_cortex_m::reg::assert_taken!("mpu_rasr");
    };
    expanded.into()
}

#[allow(clippy::too_many_arguments)]
fn gen_exc(
    exc: &Exc,
    thr: &ExprPath,
    sv: Option<&ExprPath>,
    thr_counter: &mut usize,
    vtable_ctor_tokens: &mut Vec<TokenStream2>,
    handlers_tokens: &mut Vec<TokenStream2>,
    index_tokens: &mut Vec<TokenStream2>,
    index_ctor_tokens: &mut Vec<TokenStream2>,
    array_tokens: &mut Vec<TokenStream2>,
    thr_tokens: &mut Vec<TokenStream2>,
) -> (Ident, Option<Ident>) {
    let &Exc { ref attrs, ref mode, ref ident } = exc;
    let field_ident = ident.to_string().to_snake_case();
    let field_ident = format_ident!("{}", field_ident);
    let struct_ident = format_ident!("{}", ident.to_string().to_pascal_case());
    match *mode {
        Mode::Thread(_) => {
            vtable_ctor_tokens.push(quote! {
                #field_ident: Some(::drone_cortex_m::thr::thr_handler::<#struct_ident>)
            });
        }
        Mode::Extern(_) | Mode::Fn => {
            vtable_ctor_tokens.push(quote! {
                #field_ident: Some(handlers.#field_ident)
            });
            handlers_tokens.push(quote! {
                #(#attrs)*
                pub #field_ident: ::drone_cortex_m::thr::vtable::Handler
            });
        }
    }
    match *mode {
        Mode::Thread(ref vis) | Mode::Extern(ref vis) => {
            let index = *thr_counter;
            *thr_counter += 1;
            index_tokens.push(quote! {
                #(#attrs)*
                #vis #field_ident: #struct_ident
            });
            index_ctor_tokens.push(quote! {
                #field_ident: ::drone_core::token::Token::take()
            });
            array_tokens.push(quote! {
                #thr::new(#index)
            });
            thr_tokens.push(quote! {
                #(#attrs)*
                #[derive(Clone, Copy)]
                #vis struct #struct_ident(());

                unsafe impl ::drone_core::token::Token for #struct_ident {
                    #[inline]
                    unsafe fn take() -> Self {
                        #struct_ident(())
                    }
                }

                unsafe impl ::drone_core::thr::ThrToken for #struct_ident {
                    type Thr = #thr;

                    const THR_NUM: usize = #index;
                }
            });
            if let Some(sv) = sv {
                thr_tokens.push(quote! {
                    impl ::drone_cortex_m::thr::ThrSv for #struct_ident {
                        type Sv = #sv;
                    }
                });
            }
            (field_ident, Some(struct_ident))
        }
        Mode::Fn => (field_ident, None),
    }
}
