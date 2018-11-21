use drone_macros_core::{NewStatic, NewStruct};
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, IntSuffix, LitInt};

struct Sv {
  sv: NewStruct,
  array: NewStatic,
  services: Vec<Service>,
}

struct Service {
  ident: Ident,
}

impl Parse for Sv {
  fn parse(input: ParseStream) -> Result<Self> {
    let sv = input.parse()?;
    let array = input.parse()?;
    let mut services = Vec::new();
    while !input.is_empty() {
      services.push(input.parse()?);
    }
    Ok(Self {
      sv,
      array,
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
  let (def_site, call_site) = (Span::def_site(), Span::call_site());
  let Sv {
    sv:
      NewStruct {
        attrs: sv_attrs,
        vis: sv_vis,
        ident: sv_ident,
      },
    array:
      NewStatic {
        attrs: array_attrs,
        vis: array_vis,
        ident: array_ident,
      },
    services,
  } = parse_macro_input!(input as Sv);
  let rt = Ident::new("__sv_rt", def_site);
  let mut service_counter = 0usize;
  let mut array_tokens = Vec::new();
  let mut service_tokens = Vec::new();
  for Service { ident } in services {
    let index = LitInt::new(service_counter as u64, IntSuffix::None, call_site);
    service_counter += 1;
    array_tokens.push(quote! {
      #sv_ident(#rt::service_handler::<#ident>)
    });
    service_tokens.push(quote! {
      impl #rt::SvCall<#ident> for #sv_ident {
        #[inline(always)]
        unsafe fn call(service: &mut #ident) {
          #rt::sv_call(service, #index);
        }
      }
    });
  }

  let expanded = quote! {
    mod #rt {
      extern crate drone_core;
      extern crate drone_cortex_m as drone_plat;

      pub use self::drone_core::sv::{Supervisor, SvCall};
      pub use self::drone_plat::sv::{service_handler, sv_call};
    }

    #(#sv_attrs)*
    #sv_vis struct #sv_ident(unsafe extern "C" fn(*mut *mut u8));

    impl #rt::Supervisor for #sv_ident {
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
