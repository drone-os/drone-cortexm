use drone_macros_core::{ExternStruct, NewStatic, NewStruct};
use inflector::Inflector;
use proc_macro2::{Span, TokenStream};
use std::collections::HashSet;
use syn::synom::Synom;
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

impl Synom for Vtable {
  named!(parse -> Self, do_parse!(
    vtable: syn!(NewStruct) >>
    handlers: syn!(NewStruct) >>
    index: syn!(NewStruct) >>
    array: syn!(NewStatic) >>
    thr: syn!(ExternStruct) >>
    excs: many0!(syn!(Exc)) >>
    ints: many0!(syn!(Int)) >>
    (Vtable { vtable, handlers, index, array, thr, excs, ints })
  ));
}

impl Synom for Exc {
  named!(parse -> Self, do_parse!(
    attrs: many0!(Attribute::parse_outer) >>
    mode: syn!(Mode) >>
    ident: syn!(Ident) >>
    punct!(;) >>
    (Exc { attrs, mode, ident })
  ));
}

impl Synom for Int {
  named!(parse -> Self, do_parse!(
    attrs: many0!(Attribute::parse_outer) >>
    mode: syn!(Mode) >>
    num: syn!(LitInt) >>
    punct!(:) >>
    ident: syn!(Ident) >>
    punct!(;) >>
    (Int {
      num,
      exc: Exc { attrs, mode, ident },
    })
  ));
}

impl Synom for Mode {
  named!(parse -> Self, alt!(
    keyword!(fn) => {|_| Mode::Fn } |
    do_parse!(
      vis: syn!(Visibility) >>
      ext: option!(keyword!(extern)) >>
      (if ext.is_none() { Mode::Thread(vis) } else { Mode::Extern(vis) })
    )
  ));
}

#[cfg_attr(feature = "cargo-clippy", allow(cyclomatic_complexity))]
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
  } = try_parse2!(call_site, input);
  let int_len = ints
    .iter()
    .map(|int| int.num.value() as usize + 1)
    .max()
    .unwrap_or(0);
  let rt = Ident::new("__vtable_rt", def_site);
  let def_new = Ident::new("new", call_site);
  let mut exc_holes = exc_set();
  let mut def_exc = exc_holes.clone();
  let def_reset = Ident::new("reset", call_site);
  let def_reset_ty = Ident::new("Reset", call_site);
  let def_sv_call = Ident::new("sv_call", call_site);
  let def_nmi = def_exc.def_ident("nmi");
  let def_hard_fault = def_exc.def_ident("hard_fault");
  let def_mem_manage = def_exc.def_ident("mem_manage");
  let def_bus_fault = def_exc.def_ident("bus_fault");
  let def_usage_fault = def_exc.def_ident("usage_fault");
  let def_debug = def_exc.def_ident("debug");
  let def_pend_sv = def_exc.def_ident("pend_sv");
  let def_sys_tick = def_exc.def_ident("sys_tick");
  assert!(def_exc.is_empty());
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
      thr_tokens.push(quote_spanned! { def_site =>
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
        Ident::new(&format!("IntBundle{}", num.value() / 32), def_site);
      thr_tokens.push(quote_spanned! { def_site =>
        impl<T: #rt::ThrTag> #rt::IntToken<T> for #struct_ident<T> {
          type Bundle = #rt::#bundle;

          const INT_NUM: usize = #num;
        }

        impl<T: #rt::ThrTag> #int_trait<T> for #struct_ident<T> {}
      });
    }
    vtable_tokens[num.value() as usize] = Some(quote_spanned! { def_site =>
      #field_ident: Option<#rt::Handler>
    });
  }
  for exc_ident in exc_holes {
    let exc_ident = Ident::new(exc_ident, call_site);
    vtable_ctor_tokens.push(quote_spanned!(def_site => #exc_ident: None));
  }
  let vtable_tokens = vtable_tokens
    .into_iter()
    .enumerate()
    .map(|(i, tokens)| {
      tokens.unwrap_or_else(|| {
        let int_ident = Ident::new(&format!("_int{}", i), def_site);
        vtable_ctor_tokens.push(quote_spanned!(def_site => #int_ident: None));
        quote_spanned!(def_site => #int_ident: Option<#rt::Handler>)
      })
    })
    .collect::<Vec<_>>();
  vtable_ctor_tokens.push(quote_spanned! { def_site =>
    #def_sv_call: #rt::sv_handler::<<#thr_ident as #rt::Thread>::Sv>
  });

  let expanded = quote_spanned! { def_site =>
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
      #def_reset: #rt::ResetHandler,
      #def_nmi: Option<#rt::Handler>,
      #def_hard_fault: Option<#rt::Handler>,
      #def_mem_manage: Option<#rt::Handler>,
      #def_bus_fault: Option<#rt::Handler>,
      #def_usage_fault: Option<#rt::Handler>,
      _reserved0: [#rt::Reserved; 4],
      #def_sv_call: #rt::Handler,
      #def_debug: Option<#rt::Handler>,
      _reserved1: [#rt::Reserved; 1],
      #def_pend_sv: Option<#rt::Handler>,
      #def_sys_tick: Option<#rt::Handler>,
      #(#vtable_tokens),*
    }

    #(#handlers_attrs)*
    #handlers_vis struct #handlers_ident {
      /// Reset exception handler.
      pub #def_reset: #rt::ResetHandler,
      #(#handlers_tokens),*
    }

    #(#index_attrs)*
    #index_vis struct #index_ident {
      /// Reset thread token.
      pub #def_reset: #def_reset_ty<#rt::Ctt>,
      #(#index_tokens),*
    }

    #(#array_attrs)*
    #array_vis static mut #array_ident: [#thr_ident; #thr_counter] = [
      #thr_ident::new(0),
      #(#array_tokens),*
    ];

    impl #vtable_ident {
      /// Creates a new vector table.
      pub const fn #def_new(handlers: #handlers_ident) -> Self {
        Self {
          #def_reset: handlers.#def_reset,
          _reserved0: [#rt::Reserved::Vector; 4],
          _reserved1: [#rt::Reserved::Vector; 1],
          #(#vtable_ctor_tokens),*
        }
      }
    }

    impl #rt::ThrTokens for #index_ident {
      #[inline(always)]
      unsafe fn new() -> Self {
        Self {
          #def_reset: #def_reset_ty::new(),
          #(#index_ctor_tokens),*
        }
      }
    }

    /// Reset thread token.
    pub type #def_reset_ty<T> = #rt::Reset<T, &'static #thr_ident>;

    #(#thr_tokens)*
  };
  expanded.into()
}

#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
fn gen_exc(
  exc: &Exc,
  thr_ident: &Ident,
  rt: &Ident,
  thr_counter: &mut usize,
  vtable_ctor_tokens: &mut Vec<TokenStream>,
  handlers_tokens: &mut Vec<TokenStream>,
  index_tokens: &mut Vec<TokenStream>,
  index_ctor_tokens: &mut Vec<TokenStream>,
  array_tokens: &mut Vec<TokenStream>,
  thr_tokens: &mut Vec<TokenStream>,
) -> (Ident, Option<Ident>) {
  let (def_site, call_site) = (Span::def_site(), Span::call_site());
  let &Exc {
    ref attrs,
    ref mode,
    ref ident,
  } = exc;
  let struct_ident = Ident::new(&ident.to_string().to_pascal_case(), call_site);
  let field_ident = Ident::new(&ident.to_string().to_snake_case(), call_site);
  match *mode {
    Mode::Thread(_) => {
      vtable_ctor_tokens.push(quote_spanned! { def_site =>
        #field_ident: Some(
          #rt::thr_handler::<#struct_ident<#rt::Ltt>, #rt::Ltt>,
        )
      });
    }
    Mode::Extern(_) | Mode::Fn => {
      vtable_ctor_tokens.push(quote_spanned! { def_site =>
        #field_ident: Some(handlers.#field_ident)
      });
      handlers_tokens.push(quote_spanned! { def_site =>
        #(#attrs)*
        pub #field_ident: #rt::Handler
      });
    }
  }
  match *mode {
    Mode::Thread(ref vis) | Mode::Extern(ref vis) => {
      let index = *thr_counter;
      *thr_counter += 1;
      index_tokens.push(quote_spanned! { def_site =>
        #(#attrs)*
        #vis #field_ident: #struct_ident<#rt::Ctt>
      });
      index_ctor_tokens.push(quote_spanned! { def_site =>
        #field_ident: #struct_ident::new()
      });
      array_tokens.push(quote_spanned! { def_site =>
        #thr_ident::new(#index)
      });
      thr_tokens.push(quote_spanned! { def_site =>
        #(#attrs)*
        #[derive(Clone, Copy)]
        #vis struct #struct_ident<T: #rt::ThrTag>(#rt::PhantomData<T>);

        impl<T: #rt::ThrTag> #struct_ident<T> {
          #[inline(always)]
          unsafe fn new() -> Self {
            #struct_ident(#rt::PhantomData)
          }
        }

        impl<T: #rt::ThrTag> #rt::ThrToken<T> for #struct_ident<T> {
          type Thr = #thr_ident;

          const THR_NUM: usize = #index;
        }

        impl<T: #rt::ThrTag> AsRef<#thr_ident> for #struct_ident<T> {
          #[inline(always)]
          fn as_ref(&self) -> &#thr_ident {
            <Self as #rt::ThrToken<T>>::get_thr()
          }
        }

        impl From<#struct_ident<#rt::Ctt>> for #struct_ident<#rt::Ttt> {
          #[inline(always)]
          fn from(_token: #struct_ident<#rt::Ctt>) -> Self {
            unsafe { Self::new() }
          }
        }

        impl From<#struct_ident<#rt::Ctt>> for #struct_ident<#rt::Ltt> {
          #[inline(always)]
          fn from(_token: #struct_ident<#rt::Ctt>) -> Self {
            unsafe { Self::new() }
          }
        }

        impl From<#struct_ident<#rt::Ttt>> for #struct_ident<#rt::Ltt> {
          #[inline(always)]
          fn from(_token: #struct_ident<#rt::Ttt>) -> Self {
            unsafe { Self::new() }
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

trait ExcDefIdent {
  fn def_ident(&mut self, ident: &str) -> Ident;
}

impl ExcDefIdent for HashSet<&'static str> {
  fn def_ident(&mut self, ident: &str) -> Ident {
    Ident::new(self.take(ident).unwrap(), Span::call_site())
  }
}
