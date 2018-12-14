use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, Ident, IntSuffix, LitInt, Visibility};

struct Sv {
  sv_attrs: Vec<Attribute>,
  sv_vis: Visibility,
  sv_ident: Ident,
  array_attrs: Vec<Attribute>,
  array_vis: Visibility,
  array_ident: Ident,
  services: Vec<Service>,
}

struct Service {
  ident: Ident,
}

impl Parse for Sv {
  fn parse(input: ParseStream) -> Result<Self> {
    let sv_attrs = input.call(Attribute::parse_outer)?;
    let sv_vis = input.parse()?;
    input.parse::<Token![struct]>()?;
    let sv_ident = input.parse()?;
    input.parse::<Token![;]>()?;
    let array_attrs = input.call(Attribute::parse_outer)?;
    let array_vis = input.parse()?;
    input.parse::<Token![static]>()?;
    let array_ident = input.parse()?;
    input.parse::<Token![;]>()?;
    let mut services = Vec::new();
    while !input.is_empty() {
      services.push(input.parse()?);
    }
    Ok(Self {
      sv_attrs,
      sv_vis,
      sv_ident,
      array_attrs,
      array_vis,
      array_ident,
      services,
    })
  }
}

impl Parse for Service {
  fn parse(input: ParseStream) -> Result<Self> {
    let ident = input.parse()?;
    input.parse::<Token![;]>()?;
    Ok(Self { ident })
  }
}

pub fn proc_macro(input: TokenStream) -> TokenStream {
  let Sv {
    sv_attrs,
    sv_vis,
    sv_ident,
    array_attrs,
    array_vis,
    array_ident,
    services,
  } = parse_macro_input!(input as Sv);
  let mut service_counter = 0usize;
  let mut array_tokens = Vec::new();
  let mut service_tokens = Vec::new();
  for Service { ident } in services {
    let index =
      LitInt::new(service_counter as u64, IntSuffix::None, Span::call_site());
    service_counter += 1;
    array_tokens.push(quote! {
      #sv_ident(::drone_cortex_m::sv::service_handler::<#ident>)
    });
    service_tokens.push(quote! {
      impl ::drone_core::sv::SvCall<#ident> for #sv_ident {
        #[inline(always)]
        unsafe fn call(service: &mut #ident) {
          ::drone_cortex_m::sv::sv_call(service, #index);
        }
      }
    });
  }

  let expanded = quote! {
    #(#sv_attrs)*
    #sv_vis struct #sv_ident(unsafe extern "C" fn(*mut *mut u8));

    impl ::drone_core::sv::Supervisor for #sv_ident {
      #[inline(always)]
      fn first() -> *const Self {
        #array_ident.as_ptr()
      }
    }

    #(#array_attrs)*
    #array_vis static #array_ident: [#sv_ident; #service_counter] = [
      #(#array_tokens),*
    ];

    #(#service_tokens)*
  };
  expanded.into()
}
