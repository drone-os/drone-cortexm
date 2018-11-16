use drone_macros_core::{ExternStruct, NewStatic, NewStruct};
use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, Ident, LitInt, Visibility};

struct Vtable {
  vtable: NewStruct,
  handlers: NewStruct,
  index: NewStruct,
  array: NewStatic,
  thr: ExternStruct,
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
  fn parse(input: ParseStream) -> Result<Self> {
    let vtable = input.parse()?;
    let handlers = input.parse()?;
    let index = input.parse()?;
    let array = input.parse()?;
    let thr = input.parse()?;
    let mut excs = Vec::new();
    while input.fork().parse::<Exc>().is_ok() {
      excs.push(input.parse()?);
    }
    let mut ints = Vec::new();
    while !input.is_empty() {
      ints.push(input.parse()?);
    }
    Ok(Self {
      vtable,
      handlers,
      index,
      array,
      thr,
      excs,
      ints,
    })
  }
}

impl Parse for Exc {
  fn parse(input: ParseStream) -> Result<Self> {
    let attrs = input.call(Attribute::parse_outer)?;
    let mode = input.parse()?;
    let ident = input.parse()?;
    input.parse::<Token![;]>()?;
    Ok(Self { attrs, mode, ident })
  }
}

impl Parse for Int {
  fn parse(input: ParseStream) -> Result<Self> {
    let attrs = input.call(Attribute::parse_outer)?;
    let mode = input.parse()?;
    let num = input.parse()?;
    input.parse::<Token![:]>()?;
    let ident = input.parse()?;
    input.parse::<Token![;]>()?;
    Ok(Self {
      num,
      exc: Exc { attrs, mode, ident },
    })
  }
}

impl Parse for Mode {
  fn parse(input: ParseStream) -> Result<Self> {
    if input.peek(Token![fn]) {
      input.parse::<Token![fn]>()?;
      Ok(Mode::Fn)
    } else {
      let vis = input.parse::<Visibility>()?;
      if input.parse::<Option<Token![extern]>>()?.is_none() {
        Ok(Mode::Thread(vis))
      } else {
        Ok(Mode::Extern(vis))
      }
    }
  }
}

#[allow(clippy::cyclomatic_complexity)]
pub fn proc_macro(input: TokenStream) -> TokenStream {
  let (def_site, call_site) = (Span::def_site(), Span::call_site());
  let Vtable {
    vtable:
      NewStruct {
        attrs: vtable_attrs,
        vis: vtable_vis,
        ident: vtable_ident,
      },
    handlers:
      NewStruct {
        attrs: handlers_attrs,
        vis: handlers_vis,
        ident: handlers_ident,
      },
    index:
      NewStruct {
        attrs: index_attrs,
        vis: index_vis,
        ident: index_ident,
      },
    array:
      NewStatic {
        attrs: array_attrs,
        vis: array_vis,
        ident: array_ident,
      },
    thr: ExternStruct { ident: thr_ident },
    excs,
    ints,
  } = parse_macro_input!(input as Vtable);
  let int_len = ints
    .iter()
    .map(|int| int.num.value() as usize + 1)
    .max()
    .unwrap_or(0);
  let rt = Ident::new("__vtable_rt", def_site);
  let def_reserved0 = Ident::new("_reserved0", def_site);
  let def_reserved1 = Ident::new("_reserved1", def_site);
  let mut exc_holes = exc_set();
  let mut vtable_tokens = vec![None; int_len];
  let mut vtable_ctor_tokens = Vec::new();
  let mut handlers_tokens = Vec::new();
  let mut index_tokens = Vec::new();
  let mut index_ctor_tokens = Vec::new();
  let mut array_tokens = Vec::new();
  let mut thr_tokens = Vec::new();
  let mut thr_counter = 1;
  for exc in excs {
    let (field_ident, struct_ident) = gen_exc(
      &exc,
      &thr_ident,
      &rt,
      &mut thr_counter,
      &mut vtable_ctor_tokens,
      &mut handlers_tokens,
      &mut index_tokens,
      &mut index_ctor_tokens,
      &mut array_tokens,
      &mut thr_tokens,
    );
    if let Some(struct_ident) = struct_ident {
      let int_trait = Ident::new(&format!("Int{}", struct_ident), call_site);
      thr_tokens.push(quote! {
        impl<T: #rt::ThrTag> #int_trait<T> for #struct_ident<T> {}
      });
    }
    assert!(
      exc_holes.remove(field_ident.to_string().as_str()),
      "Unknown exception name: {}",
      exc.ident.to_string(),
    );
  }
  for Int { num, exc } in ints {
    let (field_ident, struct_ident) = gen_exc(
      &exc,
      &thr_ident,
      &rt,
      &mut thr_counter,
      &mut vtable_ctor_tokens,
      &mut handlers_tokens,
      &mut index_tokens,
      &mut index_ctor_tokens,
      &mut array_tokens,
      &mut thr_tokens,
    );
    if let Some(struct_ident) = struct_ident {
      let int_trait = Ident::new(&format!("Int{}", num.value()), call_site);
      let bundle =
        Ident::new(&format!("IntBundle{}", num.value() / 32), call_site);
      thr_tokens.push(quote! {
        impl<T: #rt::ThrTag> #rt::IntToken<T> for #struct_ident<T> {
          type Bundle = #rt::#bundle;

          const INT_NUM: usize = #num;
        }

        impl<T: #rt::ThrTag> #int_trait<T> for #struct_ident<T> {}
      });
    }
    vtable_tokens[num.value() as usize] = Some(quote! {
      #field_ident: Option<#rt::Handler>
    });
  }
  for exc_ident in exc_holes {
    let exc_ident = Ident::new(exc_ident, call_site);
    vtable_ctor_tokens.push(quote!(#exc_ident: None));
  }
  let vtable_tokens = vtable_tokens
    .into_iter()
    .enumerate()
    .map(|(i, tokens)| {
      tokens.unwrap_or_else(|| {
        let int_ident = Ident::new(&format!("_int{}", i), def_site);
        vtable_ctor_tokens.push(quote!(#int_ident: None));
        quote!(#int_ident: Option<#rt::Handler>)
      })
    })
    .collect::<Vec<_>>();
  vtable_ctor_tokens.push(quote! {
    sv_call: #rt::sv_handler::<<#thr_ident as #rt::Thread>::Sv>
  });

  let expanded = quote! {
    mod #rt {
      extern crate core;
      extern crate drone_core;
      extern crate drone_stm32 as drone_plat;

      pub use self::core::marker::PhantomData;
      pub use self::drone_core::thr::ThrTokens;
      pub use self::drone_plat::sv::sv_handler;
      pub use self::drone_plat::thr::thr_handler;
      pub use self::drone_plat::thr::int::*;
      pub use self::drone_plat::thr::prelude::*;
      pub use self::drone_plat::thr::vtable::{Handler, Reserved, Reset,
                                              ResetHandler};
    }

    #(#vtable_attrs)*
    #[allow(dead_code)]
    #vtable_vis struct #vtable_ident {
      reset: #rt::ResetHandler,
      nmi: Option<#rt::Handler>,
      hard_fault: Option<#rt::Handler>,
      mem_manage: Option<#rt::Handler>,
      bus_fault: Option<#rt::Handler>,
      usage_fault: Option<#rt::Handler>,
      #def_reserved0: [#rt::Reserved; 4],
      sv_call: #rt::Handler,
      debug: Option<#rt::Handler>,
      #def_reserved1: [#rt::Reserved; 1],
      pend_sv: Option<#rt::Handler>,
      sys_tick: Option<#rt::Handler>,
      #(#vtable_tokens),*
    }

    #(#handlers_attrs)*
    #handlers_vis struct #handlers_ident {
      /// Reset exception handler.
      pub reset: #rt::ResetHandler,
      #(#handlers_tokens),*
    }

    #(#index_attrs)*
    #index_vis struct #index_ident {
      /// Reset thread token.
      pub reset: Reset<#rt::Utt>,
      #(#index_tokens),*
    }

    #(#array_attrs)*
    #array_vis static mut #array_ident: [#thr_ident; #thr_counter] = [
      #thr_ident::new(0),
      #(#array_tokens),*
    ];

    impl #vtable_ident {
      /// Creates a new vector table.
      pub const fn new(handlers: #handlers_ident) -> Self {
        Self {
          reset: handlers.reset,
          #def_reserved0: [#rt::Reserved::Vector; 4],
          #def_reserved1: [#rt::Reserved::Vector; 1],
          #(#vtable_ctor_tokens),*
        }
      }
    }

    impl #rt::ThrTokens for #index_ident {
      #[inline(always)]
      unsafe fn new() -> Self {
        Self {
          reset: #rt::ThrToken::<#rt::Utt>::new(),
          #(#index_ctor_tokens),*
        }
      }
    }

    /// Reset thread token.
    pub type Reset<T> = #rt::Reset<T, &'static #thr_ident>;

    #(#thr_tokens)*
  };
  expanded.into()
}

#[allow(clippy::too_many_arguments)]
fn gen_exc(
  exc: &Exc,
  thr_ident: &Ident,
  rt: &Ident,
  thr_counter: &mut usize,
  vtable_ctor_tokens: &mut Vec<TokenStream2>,
  handlers_tokens: &mut Vec<TokenStream2>,
  index_tokens: &mut Vec<TokenStream2>,
  index_ctor_tokens: &mut Vec<TokenStream2>,
  array_tokens: &mut Vec<TokenStream2>,
  thr_tokens: &mut Vec<TokenStream2>,
) -> (Ident, Option<Ident>) {
  let call_site = Span::call_site();
  let &Exc {
    ref attrs,
    ref mode,
    ref ident,
  } = exc;
  let struct_ident = Ident::new(&ident.to_string().to_pascal_case(), call_site);
  let field_ident = Ident::new(&ident.to_string().to_snake_case(), call_site);
  match *mode {
    Mode::Thread(_) => {
      vtable_ctor_tokens.push(quote! {
        #field_ident: Some(
          #rt::thr_handler::<#struct_ident<#rt::Att>, #rt::Att>,
        )
      });
    }
    Mode::Extern(_) | Mode::Fn => {
      vtable_ctor_tokens.push(quote! {
        #field_ident: Some(handlers.#field_ident)
      });
      handlers_tokens.push(quote! {
        #(#attrs)*
        pub #field_ident: #rt::Handler
      });
    }
  }
  match *mode {
    Mode::Thread(ref vis) | Mode::Extern(ref vis) => {
      let index = *thr_counter;
      *thr_counter += 1;
      index_tokens.push(quote! {
        #(#attrs)*
        #vis #field_ident: #struct_ident<#rt::Utt>
      });
      index_ctor_tokens.push(quote! {
        #field_ident: #rt::ThrToken::<#rt::Utt>::new()
      });
      array_tokens.push(quote! {
        #thr_ident::new(#index)
      });
      thr_tokens.push(quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        #vis struct #struct_ident<T: #rt::ThrTag>(#rt::PhantomData<T>);

        impl<T: #rt::ThrTag> #rt::ThrToken<T> for #struct_ident<T> {
          type Thr = #thr_ident;
          type UThrToken = #struct_ident<#rt::Utt>;
          type TThrToken = #struct_ident<#rt::Ttt>;
          type AThrToken = #struct_ident<#rt::Att>;

          const THR_NUM: usize = #index;

          #[inline(always)]
          unsafe fn new() -> Self {
            #struct_ident(#rt::PhantomData)
          }
        }

        impl<T: #rt::ThrTag> AsRef<#thr_ident> for #struct_ident<T> {
          #[inline(always)]
          fn as_ref(&self) -> &#thr_ident {
            unsafe { <Self as #rt::ThrToken<T>>::get_thr() }
          }
        }
      });
      (field_ident, Some(struct_ident))
    }
    Mode::Fn => (field_ident, None),
  }
}

fn exc_set() -> HashSet<&'static str> {
  let mut set = HashSet::new();
  set.insert("nmi");
  set.insert("hard_fault");
  set.insert("mem_manage");
  set.insert("bus_fault");
  set.insert("usage_fault");
  set.insert("debug");
  set.insert("pend_sv");
  set.insert("sys_tick");
  set
}
