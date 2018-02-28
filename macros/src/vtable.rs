use drone_macros2_core::{ExternStruct, NewStatic, NewStruct};
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
  thread: ExternStruct,
  exceptions: Vec<Exception>,
  interrupts: Vec<Interrupt>,
}

struct Exception {
  attrs: Vec<Attribute>,
  vis: Visibility,
  ident: Ident,
}

struct Interrupt {
  number: LitInt,
  exception: Exception,
}

impl Synom for Exception {
  named!(parse -> Self, do_parse!(
    attrs: many0!(Attribute::parse_outer) >>
    vis: syn!(Visibility) >>
    ident: syn!(Ident) >>
    punct!(;) >>
    (Exception { attrs, vis, ident })
  ));
}

impl Synom for Interrupt {
  named!(parse -> Self, do_parse!(
    attrs: many0!(Attribute::parse_outer) >>
    vis: syn!(Visibility) >>
    number: syn!(LitInt) >>
    punct!(:) >>
    ident: syn!(Ident) >>
    punct!(;) >>
    (Interrupt {
      number,
      exception: Exception { attrs, vis, ident },
    })
  ));
}

impl Synom for Vtable {
  named!(parse -> Self, do_parse!(
    vtable: syn!(NewStruct) >>
    tokens: syn!(NewStruct) >>
    array: syn!(NewStatic) >>
    thread: syn!(ExternStruct) >>
    exceptions: many0!(syn!(Exception)) >>
    interrupts: many0!(syn!(Interrupt)) >>
    (Vtable {
      vtable,
      tokens,
      array,
      thread,
      exceptions,
      interrupts,
    })
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
    thread: ExternStruct {
      ident: thread_ident,
    },
    exceptions,
    interrupts,
  } = try_parse!(call_site, input);
  let array_len = exceptions.len() + interrupts.len() + 1;
  let rt = Ident::from("__vtable_rt");
  let new_ident = Ident::new("new", call_site);
  let irq_extent = interrupts
    .iter()
    .map(|irq| irq.number.value() + 1)
    .max()
    .unwrap_or(0);
  let mut irq_idents = (0..irq_extent)
    .map(|n| Ident::from(format!("_irq{}", n)))
    .collect::<Vec<_>>();
  let mut vtable_tokens = Vec::new();
  let mut vtable_ctor_tokens = Vec::new();
  let mut vtable_ctor_default_tokens = Vec::new();
  let mut tokens_tokens = Vec::new();
  let mut tokens_ctor_tokens = Vec::new();
  let mut array_tokens = Vec::new();
  let mut thread_tokens = Vec::new();
  let mut thread_counter = 0;
  for exception in exceptions {
    thread_counter += 1;
    let (struct_ident, _) = gen_exception(
      thread_counter,
      exception,
      &thread_ident,
      &rt,
      &mut vtable_ctor_tokens,
      &mut tokens_tokens,
      &mut tokens_ctor_tokens,
      &mut array_tokens,
      &mut thread_tokens,
    );
    let irq_trait = Ident::from(format!("Irq{}", struct_ident));
    thread_tokens.push(quote! {
      impl<T: #rt::ThdTag> #rt::#irq_trait<T> for #struct_ident<T> {}
    });
  }
  for Interrupt { number, exception } in interrupts {
    thread_counter += 1;
    let (struct_ident, field_ident) = gen_exception(
      thread_counter,
      exception,
      &thread_ident,
      &rt,
      &mut vtable_ctor_tokens,
      &mut tokens_tokens,
      &mut tokens_ctor_tokens,
      &mut array_tokens,
      &mut thread_tokens,
    );
    let irq_trait = Ident::from(format!("Irq{}", number.value()));
    let bundle = Ident::from(format!("IrqBundle{}", number.value() / 32));
    thread_tokens.push(quote! {
      impl<T: #rt::ThdTag> #rt::IrqToken<T> for #struct_ident<T> {
        type Bundle = #rt::#bundle;

        const IRQ_NUM: usize = #number;
      }

      impl<T: #rt::ThdTag> #rt::#irq_trait<T> for #struct_ident<T> {}
    });
    irq_idents[number.value() as usize] = field_ident;
  }
  for irq_ident in irq_idents {
    vtable_tokens.push(quote! {
      #irq_ident: Option<#rt::Handler>
    });
    vtable_ctor_default_tokens.push(quote! {
      #irq_ident: None
    });
  }

  let expanded = quote! {
    mod #rt {
      extern crate core;
      extern crate drone_core;
      extern crate drone_stm32 as drone_plfm;

      pub use self::core::marker::PhantomData;
      pub use self::drone_core::thread::ThdTokens;
      pub use self::drone_plfm::thread::irq::*;
      pub use self::drone_plfm::thread::prelude::*;
      pub use self::drone_plfm::thread::vtable::{Handler, Reserved,
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

    impl #rt::ThdTokens for #tokens_ident {
      #[inline(always)]
      unsafe fn new() -> Self {
        Self {
          #(#tokens_ctor_tokens),*
        }
      }
    }

    #(#array_attrs)*
    #array_vis static mut #array_ident: [#thread_ident; #array_len] = [
      #thread_ident::new(0),
      #(#array_tokens),*
    ];

    #(#thread_tokens)*
  };
  expanded.into()
}

fn gen_exception(
  index: usize,
  exception: Exception,
  thread_ident: &Ident,
  rt: &Ident,
  vtable_ctor_tokens: &mut Vec<Tokens>,
  tokens_tokens: &mut Vec<Tokens>,
  tokens_ctor_tokens: &mut Vec<Tokens>,
  array_tokens: &mut Vec<Tokens>,
  thread_tokens: &mut Vec<Tokens>,
) -> (Ident, Ident) {
  let call_site = Span::call_site();
  let Exception {
    ref attrs,
    ref vis,
    ref ident,
  } = exception;
  let vtable_field_ident = Ident::from(ident.as_ref().to_snake_case());
  let struct_ident = Ident::new(&ident.as_ref().to_pascal_case(), call_site);
  let field_ident = Ident::new(vtable_field_ident.as_ref(), call_site);
  vtable_ctor_tokens.push(quote! {
    #vtable_field_ident: Some(
      <#struct_ident<#rt::Ltt> as #rt::ThdToken<#rt::Ltt>>::handler,
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
    #thread_ident::new(#index)
  });
  thread_tokens.push(quote! {
    #(#attrs)*
    #[derive(Clone, Copy)]
    #vis struct #struct_ident<T: #rt::ThdTag>(#rt::PhantomData<T>);

    impl<T: #rt::ThdTag> #struct_ident<T> {
      #[inline(always)]
      unsafe fn new() -> Self {
        #struct_ident(#rt::PhantomData)
      }
    }

    impl<T: #rt::ThdTag> #rt::ThdToken<T> for #struct_ident<T> {
      type Thd = #thread_ident;

      const THD_NUM: usize = #index;
    }

    impl<T: #rt::ThdTag> AsRef<#thread_ident> for #struct_ident<T> {
      #[inline(always)]
      fn as_ref(&self) -> &#thread_ident {
        #rt::ThdToken::as_thd(self)
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
