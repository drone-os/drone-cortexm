use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Attribute, Ident, LitInt, Token, Visibility,
};

struct Input {
    pool: Pool,
    sv: Sv,
    services: Services,
}

struct Pool {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
}

struct Sv {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
}

struct Services {
    services: Vec<Service>,
}

struct Service {
    ident: Ident,
}

impl Parse for Input {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut pool = None;
        let mut sv = None;
        let mut services = None;
        while !input.is_empty() {
            let attrs = input.call(Attribute::parse_outer)?;
            let ident = input.parse::<Ident>()?;
            input.parse::<Token![=>]>()?;
            if ident == "pool" {
                if pool.is_none() {
                    pool = Some(Pool::parse(input, attrs)?);
                } else {
                    return Err(input.error("multiple `pool` specifications"));
                }
            } else if ident == "supervisor" {
                if sv.is_none() {
                    sv = Some(Sv::parse(input, attrs)?);
                } else {
                    return Err(input.error("multiple `supervisor` specifications"));
                }
            } else if attrs.is_empty() && ident == "services" {
                if services.is_none() {
                    services = Some(input.parse()?);
                } else {
                    return Err(input.error("multiple `services` specifications"));
                }
            } else {
                return Err(input.error(format!("unknown key: `{}`", ident)));
            }
            if !input.is_empty() {
                input.parse::<Token![;]>()?;
            }
        }
        Ok(Self {
            pool: pool.ok_or_else(|| input.error("missing `pool` specification"))?,
            sv: sv.ok_or_else(|| input.error("missing `sv` specification"))?,
            services: services.ok_or_else(|| input.error("missing `services` specification"))?,
        })
    }
}

impl Pool {
    fn parse(input: ParseStream<'_>, attrs: Vec<Attribute>) -> Result<Self> {
        let vis = input.parse()?;
        let ident = input.parse()?;
        Ok(Self { attrs, vis, ident })
    }
}

impl Sv {
    fn parse(input: ParseStream<'_>, attrs: Vec<Attribute>) -> Result<Self> {
        let vis = input.parse()?;
        let ident = input.parse()?;
        Ok(Self { attrs, vis, ident })
    }
}

impl Parse for Services {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let input2;
        braced!(input2 in input);
        let mut services = Vec::new();
        while !input2.is_empty() {
            let ident = input2.parse::<Ident>()?;
            services.push(Service { ident });
            if !input2.is_empty() {
                input2.parse::<Token![;]>()?;
            }
        }
        Ok(Self { services })
    }
}

pub fn proc_macro(input: TokenStream) -> TokenStream {
    let Input { pool, sv, services } = parse_macro_input!(input as Input);
    let Pool { attrs: pool_attrs, vis: pool_vis, ident: pool_ident } = pool;
    let Sv { attrs: sv_attrs, vis: sv_vis, ident: sv_ident } = sv;
    let Services { services } = services;
    let mut pool_tokens = Vec::new();
    let mut service_counter = 0_usize;
    let mut service_tokens = Vec::new();
    for Service { ident } in services {
        let index = LitInt::new(&service_counter.to_string(), Span::call_site());
        service_counter += 1;
        pool_tokens.push(quote! {
            #sv_ident(::drone_cortexm::sv::service_handler::<#ident>)
        });
        service_tokens.push(quote! {
            impl ::drone_cortexm::sv::SvCall<#ident> for #sv_ident {
                #[inline]
                unsafe fn call(service: &mut #ident) {
                    ::drone_cortexm::sv::sv_call::<_, #index>(service);
                }
            }
        });
    }
    let static_ident = format_ident!("{}", pool_ident.to_string().to_screaming_snake_case());
    let expanded = quote! {
        #(#pool_attrs)*
        #pool_vis static #static_ident: #pool_ident = #pool_ident::new();

        #(#pool_attrs)*
        #[repr(C)]
        #pool_vis struct #pool_ident {
            services: [#sv_ident; #service_counter],
        }

        impl #pool_ident {
            const fn new() -> Self {
                Self {
                    services: [#(#pool_tokens),*],
                }
            }
        }

        #(#sv_attrs)*
        #sv_vis struct #sv_ident(unsafe extern "C" fn(*mut *mut u8));

        impl ::drone_cortexm::sv::Supervisor for #sv_ident {
            #[cfg_attr(not(feature = "std"), naked)]
            unsafe extern "C" fn handler() {
                #[cfg_attr(feature = "std", allow(unreachable_code))]
                #[cfg(feature = "std")]
                return ::std::unimplemented!();
                #[cfg(not(feature = "std"))]
                unsafe {
                    asm!(
                        "tst lr, #4",
                        "ite eq",
                        "mrseq r0, msp",
                        "mrsne r0, psp",
                        "ldr r1, [r0, #24]",
                        "ldrb r1, [r1, #-2]",
                        "movw r0, #:lower16:{0}",
                        "movt r0, #:upper16:{0}",
                        "ldr pc, [r0, r1, lsl #2]",
                        sym #static_ident,
                        options(noreturn),
                    );
                }
            }
        }

        #(#service_tokens)*
    };
    expanded.into()
}
