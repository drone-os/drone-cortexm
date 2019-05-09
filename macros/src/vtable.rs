use drone_macros_core::{new_def_ident, new_ident};
use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::{collections::HashSet, convert::TryFrom};
use syn::{
  parse::{Parse, ParseStream, Result},
  parse_macro_input, Attribute, ExprPath, Ident, LitInt, Token, Visibility,
};

struct Vtable {
  vtable_attrs: Vec<Attribute>,
  vtable_vis: Visibility,
  vtable_ident: Ident,
  handlers_attrs: Vec<Attribute>,
  handlers_vis: Visibility,
  handlers_ident: Ident,
  index_attrs: Vec<Attribute>,
  index_vis: Visibility,
  index_ident: Ident,
  array_attrs: Vec<Attribute>,
  array_vis: Visibility,
  array_ident: Ident,
  thr: ExprPath,
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
    let array_attrs = input.call(Attribute::parse_outer)?;
    let array_vis = input.parse()?;
    input.parse::<Token![static]>()?;
    let array_ident = input.parse()?;
    input.parse::<Token![;]>()?;
    input.parse::<Token![extern]>()?;
    input.parse::<Token![struct]>()?;
    let thr = input.parse()?;
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
      vtable_attrs,
      vtable_vis,
      vtable_ident,
      handlers_attrs,
      handlers_vis,
      handlers_ident,
      index_attrs,
      index_vis,
      index_ident,
      array_attrs,
      array_vis,
      array_ident,
      thr,
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
    Ok(Self {
      num,
      exc: Exc { attrs, mode, ident },
    })
  }
}

impl Parse for Mode {
  fn parse(input: ParseStream<'_>) -> Result<Self> {
    if input.peek(Token![fn]) {
      input.parse::<Token![fn]>()?;
      Ok(Mode::Fn)
    } else {
      let vis = input.parse::<Visibility>()?;
      if input.parse::<Option<Token![extern]>>()?.is_some() {
        Ok(Mode::Extern(vis))
      } else {
        Ok(Mode::Thread(vis))
      }
    }
  }
}

#[allow(clippy::cognitive_complexity)]
pub fn proc_macro(input: TokenStream) -> TokenStream {
  let Vtable {
    vtable_attrs,
    vtable_vis,
    vtable_ident,
    handlers_attrs,
    handlers_vis,
    handlers_ident,
    index_attrs,
    index_vis,
    index_ident,
    array_attrs,
    array_vis,
    array_ident,
    thr,
    excs,
    ints,
  } = parse_macro_input!(input as Vtable);
  let int_len = ints
    .iter()
    .map(|int| usize::try_from(int.num.value()).unwrap() + 1)
    .max()
    .unwrap_or(0);
  let def_reserved0 = new_def_ident!("_reserved0");
  let def_reserved1 = new_def_ident!("_reserved1");
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
      &thr,
      &mut thr_counter,
      &mut vtable_ctor_tokens,
      &mut handlers_tokens,
      &mut index_tokens,
      &mut index_ctor_tokens,
      &mut array_tokens,
      &mut thr_tokens,
    );
    if let Some(struct_ident) = struct_ident {
      let int_trait = new_ident!("Int{}", struct_ident);
      thr_tokens.push(quote! {
        impl<T: ::drone_core::thr::ThrTag> #int_trait<T> for #struct_ident<T> {}
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
      &thr,
      &mut thr_counter,
      &mut vtable_ctor_tokens,
      &mut handlers_tokens,
      &mut index_tokens,
      &mut index_ctor_tokens,
      &mut array_tokens,
      &mut thr_tokens,
    );
    if let Some(struct_ident) = struct_ident {
      let int_trait = new_ident!("Int{}", num.value());
      let bundle = new_ident!("IntBundle{}", num.value() / 32);
      thr_tokens.push(quote! {
        impl<T> ::drone_cortex_m::thr::IntToken<T> for #struct_ident<T>
        where
          T: ::drone_core::thr::ThrTag,
        {
          type Bundle = ::drone_cortex_m::map::thr::#bundle;

          const INT_NUM: usize = #num;
        }

        impl<T: ::drone_core::thr::ThrTag> #int_trait<T> for #struct_ident<T> {}
      });
    }
    vtable_tokens[usize::try_from(num.value()).unwrap()] = Some(quote! {
      #field_ident: Option<::drone_cortex_m::thr::vtable::Handler>
    });
  }
  for exc_ident in exc_holes {
    let exc_ident = new_ident!("{}", exc_ident);
    vtable_ctor_tokens.push(quote!(#exc_ident: None));
  }
  let vtable_tokens = vtable_tokens
    .into_iter()
    .enumerate()
    .map(|(i, tokens)| {
      tokens.unwrap_or_else(|| {
        let int_ident = new_def_ident!("_int{}", i);
        vtable_ctor_tokens.push(quote!(#int_ident: None));
        quote!(#int_ident: Option<::drone_cortex_m::thr::vtable::Handler>)
      })
    })
    .collect::<Vec<_>>();

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
      #def_reserved0: [::drone_cortex_m::thr::vtable::Reserved; 4],
      sv_call: Option<::drone_cortex_m::thr::vtable::Handler>,
      debug: Option<::drone_cortex_m::thr::vtable::Handler>,
      #def_reserved1: [::drone_cortex_m::thr::vtable::Reserved; 1],
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
      pub reset: Reset<::drone_core::thr::Ptt>,
      #(#index_tokens),*
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
          reset: handlers.reset,
          #def_reserved0: [::drone_cortex_m::thr::vtable::Reserved::Vector; 4],
          #def_reserved1: [::drone_cortex_m::thr::vtable::Reserved::Vector; 1],
          #(#vtable_ctor_tokens),*
        }
      }
    }

    unsafe impl ::drone_core::token::Tokens for #index_ident {
      #[inline]
      unsafe fn take() -> Self {
        Self {
          reset: ::drone_core::thr::ThrToken::<::drone_core::thr::Ptt>::take(),
          #(#index_ctor_tokens),*
        }
      }
    }

    unsafe impl ::drone_cortex_m::thr::ThrTokens for #index_ident {}

    /// Reset thread token.
    pub type Reset<T> = ::drone_cortex_m::thr::vtable::Reset<T, &'static #thr>;

    #(#thr_tokens)*
  };
  expanded.into()
}

#[allow(clippy::too_many_arguments)]
fn gen_exc(
  exc: &Exc,
  thr: &ExprPath,
  thr_counter: &mut usize,
  vtable_ctor_tokens: &mut Vec<TokenStream2>,
  handlers_tokens: &mut Vec<TokenStream2>,
  index_tokens: &mut Vec<TokenStream2>,
  index_ctor_tokens: &mut Vec<TokenStream2>,
  array_tokens: &mut Vec<TokenStream2>,
  thr_tokens: &mut Vec<TokenStream2>,
) -> (Ident, Option<Ident>) {
  let &Exc {
    ref attrs,
    ref mode,
    ref ident,
  } = exc;
  let field_ident = ident.to_string().to_snake_case();
  let field_ident = new_ident!("{}", field_ident);
  let struct_ident = new_ident!("{}", ident.to_string().to_pascal_case());
  match *mode {
    Mode::Thread(_) => {
      vtable_ctor_tokens.push(quote! {
        #field_ident: Some(::drone_cortex_m::thr::thr_handler::<
          #struct_ident<::drone_core::thr::Att>,
          ::drone_core::thr::Att,
        >)
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
        #vis #field_ident: #struct_ident<::drone_core::thr::Ptt>
      });
      index_ctor_tokens.push(quote! {
        #field_ident: ::drone_core::thr::ThrToken::<
          ::drone_core::thr::Ptt,
        >::take()
      });
      array_tokens.push(quote! {
        #thr::new(#index)
      });
      thr_tokens.push(quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        #vis struct #struct_ident<T: ::drone_core::thr::ThrTag>(
          ::core::marker::PhantomData<T>,
        );

        impl<T> ::drone_core::thr::ThrToken<T> for #struct_ident<T>
        where
          T: ::drone_core::thr::ThrTag,
        {
          type Thr = #thr;
          type TThrToken = #struct_ident<::drone_core::thr::Ttt>;
          type AThrToken = #struct_ident<::drone_core::thr::Att>;
          type PThrToken = #struct_ident<::drone_core::thr::Ptt>;

          const THR_NUM: usize = #index;

          #[inline]
          unsafe fn take() -> Self {
            #struct_ident(::core::marker::PhantomData)
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
  set.insert("sv_call");
  set.insert("debug");
  set.insert("pend_sv");
  set.insert("sys_tick");
  set
}
