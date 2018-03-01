use drone_macros_core::{ExternStruct, NewStatic, NewStruct};
use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::Tokens;
use syn::{Attribute, Ident, LitInt, Visibility};
use syn::synom::Synom;

struct Vtable {
  vtable: NewStruct,
  tokens: NewStruct,
  array: NewStatic,
  thr: ExternStruct,
  excs: Vec<Exc>,
  ints: Vec<Int>,
}

struct Exc {
  attrs: Vec<Attribute>,
  vis: Visibility,
  ident: Ident,
}

struct Int {
  num: LitInt,
  exc: Exc,
}

impl Synom for Exc {
  named!(parse -> Self, do_parse!(
    attrs: many0!(Attribute::parse_outer) >>
    vis: syn!(Visibility) >>
    ident: syn!(Ident) >>
    punct!(;) >>
    (Exc { attrs, vis, ident })
  ));
}

impl Synom for Int {
  named!(parse -> Self, do_parse!(
    attrs: many0!(Attribute::parse_outer) >>
    vis: syn!(Visibility) >>
    num: syn!(LitInt) >>
    punct!(:) >>
    ident: syn!(Ident) >>
    punct!(;) >>
    (Int {
      num,
      exc: Exc { attrs, vis, ident },
    })
  ));
}

impl Synom for Vtable {
  named!(parse -> Self, do_parse!(
    vtable: syn!(NewStruct) >>
    tokens: syn!(NewStruct) >>
    array: syn!(NewStatic) >>
    thr: syn!(ExternStruct) >>
    excs: many0!(syn!(Exc)) >>
    ints: many0!(syn!(Int)) >>
    (Vtable { vtable, tokens, array, thr, excs, ints })
  ));
}

pub fn proc_macro(input: TokenStream) -> TokenStream {
  let call_site = Span::call_site();
  let Vtable {
    vtable:
      NewStruct {
        attrs: vtable_attrs,
        vis: vtable_vis,
        ident: vtable_ident,
      },
    tokens:
      NewStruct {
        attrs: tokens_attrs,
        vis: tokens_vis,
        ident: tokens_ident,
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
  } = try_parse!(call_site, input);
  let array_len = excs.len() + ints.len() + 1;
  let rt = Ident::from("__vtable_rt");
  let new_ident = Ident::new("new", call_site);
  let int_extent = ints
    .iter()
    .map(|int| int.num.value() + 1)
    .max()
    .unwrap_or(0);
  let mut int_idents = (0..int_extent)
    .map(|n| Ident::from(format!("_int{}", n)))
    .collect::<Vec<_>>();
  let mut vtable_tokens = Vec::new();
  let mut vtable_ctor_tokens = Vec::new();
  let mut vtable_ctor_default_tokens = Vec::new();
  let mut tokens_tokens = Vec::new();
  let mut tokens_ctor_tokens = Vec::new();
  let mut array_tokens = Vec::new();
  let mut thr_tokens = Vec::new();
  let mut thr_counter = 0;
  for exc in excs {
    thr_counter += 1;
    let (struct_ident, _) = gen_exc(
      thr_counter,
      exc,
      &thr_ident,
      &rt,
      &mut vtable_ctor_tokens,
      &mut tokens_tokens,
      &mut tokens_ctor_tokens,
      &mut array_tokens,
      &mut thr_tokens,
    );
    let int_trait = Ident::from(format!("Int{}", struct_ident));
    thr_tokens.push(quote! {
      impl<T: #rt::ThrTag> #rt::#int_trait<T> for #struct_ident<T> {}
    });
  }
  for Int { num, exc } in ints {
    thr_counter += 1;
    let (struct_ident, field_ident) = gen_exc(
      thr_counter,
      exc,
      &thr_ident,
      &rt,
      &mut vtable_ctor_tokens,
      &mut tokens_tokens,
      &mut tokens_ctor_tokens,
      &mut array_tokens,
      &mut thr_tokens,
    );
    let int_trait = Ident::from(format!("Int{}", num.value()));
    let bundle = Ident::from(format!("IntBundle{}", num.value() / 32));
    thr_tokens.push(quote! {
      impl<T: #rt::ThrTag> #rt::IntToken<T> for #struct_ident<T> {
        type Bundle = #rt::#bundle;

        const INT_NUM: usize = #num;
      }

      impl<T: #rt::ThrTag> #rt::#int_trait<T> for #struct_ident<T> {}
    });
    int_idents[num.value() as usize] = field_ident;
  }
  for int_ident in int_idents {
    vtable_tokens.push(quote! {
      #int_ident: Option<#rt::Handler>
    });
    vtable_ctor_default_tokens.push(quote! {
      #int_ident: None
    });
  }

  let expanded = quote! {
    mod #rt {
      extern crate core;
      extern crate drone_core;
      extern crate drone_stm32 as drone_plat;

      pub use self::core::marker::PhantomData;
      pub use self::drone_core::thr::ThrTokens;
      pub use self::drone_plat::thr::int::*;
      pub use self::drone_plat::thr::prelude::*;
      pub use self::drone_plat::thr::vtable::{Handler, Reserved, ResetHandler};
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
      _reserved0: [#rt::Reserved; 4],
      sv_call: Option<#rt::Handler>,
      debug: Option<#rt::Handler>,
      _reserved1: [#rt::Reserved; 1],
      pend_sv: Option<#rt::Handler>,
      sys_tick: Option<#rt::Handler>,
      #(#vtable_tokens,)*
    }

    impl #vtable_ident {
      /// Creates a new vector table.
      #[inline(always)]
      pub const fn #new_ident(reset: #rt::ResetHandler) -> Self {
        Self {
          #(#vtable_ctor_tokens,)*
          ..Self {
            reset,
            nmi: None,
            hard_fault: None,
            mem_manage: None,
            bus_fault: None,
            usage_fault: None,
            _reserved0: [#rt::Reserved::Vector; 4],
            sv_call: None,
            debug: None,
            _reserved1: [#rt::Reserved::Vector; 1],
            pend_sv: None,
            sys_tick: None,
            #(#vtable_ctor_default_tokens,)*
          }
        }
      }
    }

    #(#tokens_attrs)*
    #tokens_vis struct #tokens_ident {
      #(#tokens_tokens),*
    }

    impl #rt::ThrTokens for #tokens_ident {
      #[inline(always)]
      unsafe fn new() -> Self {
        Self {
          #(#tokens_ctor_tokens),*
        }
      }
    }

    #(#array_attrs)*
    #array_vis static mut #array_ident: [#thr_ident; #array_len] = [
      #thr_ident::new(0),
      #(#array_tokens),*
    ];

    #(#thr_tokens)*
  };
  expanded.into()
}

fn gen_exc(
  index: usize,
  exc: Exc,
  thr_ident: &Ident,
  rt: &Ident,
  vtable_ctor_tokens: &mut Vec<Tokens>,
  tokens_tokens: &mut Vec<Tokens>,
  tokens_ctor_tokens: &mut Vec<Tokens>,
  array_tokens: &mut Vec<Tokens>,
  thr_tokens: &mut Vec<Tokens>,
) -> (Ident, Ident) {
  let call_site = Span::call_site();
  let Exc {
    ref attrs,
    ref vis,
    ref ident,
  } = exc;
  let vtable_field_ident = Ident::from(ident.as_ref().to_snake_case());
  let struct_ident = Ident::new(&ident.as_ref().to_pascal_case(), call_site);
  let field_ident = Ident::new(vtable_field_ident.as_ref(), call_site);
  vtable_ctor_tokens.push(quote! {
    #vtable_field_ident: Some(
      <#struct_ident<#rt::Ltt> as #rt::ThrToken<#rt::Ltt>>::handler,
    )
  });
  tokens_tokens.push(quote! {
    #(#attrs)*
    #vis #field_ident: #struct_ident<#rt::Ctt>
  });
  tokens_ctor_tokens.push(quote! {
    #field_ident: #struct_ident::new()
  });
  array_tokens.push(quote! {
    #thr_ident::new(#index)
  });
  thr_tokens.push(quote! {
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
        #rt::ThrToken::as_thr(self)
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
  (struct_ident, vtable_field_ident)
}
