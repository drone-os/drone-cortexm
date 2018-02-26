use drone_macros2_core::{ExternStruct, NewStatic, NewStruct};
use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::Tokens;
use syn::{parse, Attribute, Ident, LitInt, Visibility};
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
  let input = parse::<Vtable>(input).unwrap();
  let vtable_attrs = input.vtable.attrs;
  let vtable_vis = input.vtable.vis;
  let vtable_ident = input.vtable.ident;
  let tokens_attrs = input.tokens.attrs;
  let tokens_vis = input.tokens.vis;
  let tokens_ident = input.tokens.ident;
  let array_attrs = input.array.attrs;
  let array_vis = input.array.vis;
  let array_ident = input.array.ident;
  let thread_ident = input.thread.ident;
  let array_len = input.exceptions.len() + input.interrupts.len() + 1;
  let irq_extent = input
    .interrupts
    .iter()
    .map(|irq| irq.number.value() + 1)
    .max()
    .unwrap_or(0);
  let mut irq_ident = (0..irq_extent)
    .map(|n| Ident::from(format!("_irq{}", n)))
    .collect::<Vec<_>>();
  let mut vtable_ctor_tokens = Vec::new();
  let mut tokens_tokens = Vec::new();
  let mut tokens_ctor_tokens = Vec::new();
  let mut array_tokens = Vec::new();
  let mut thread_tokens = Vec::new();
  for (index, exception) in input.exceptions.iter().enumerate() {
    let (struct_ident, _) = gen_exception(
      index,
      exception,
      &thread_ident,
      &mut vtable_ctor_tokens,
      &mut tokens_tokens,
      &mut tokens_ctor_tokens,
      &mut array_tokens,
      &mut thread_tokens,
    );
    let irq_trait = Ident::from(format!("Irq{}", struct_ident));
    thread_tokens.push(quote! {
      impl<T: rt::ThreadTag> rt::#irq_trait<T> for #struct_ident<T> {}
    });
  }
  for (index, irq) in input.interrupts.iter().enumerate() {
    let (struct_ident, field_ident) = gen_exception(
      index,
      &irq.exception,
      &thread_ident,
      &mut vtable_ctor_tokens,
      &mut tokens_tokens,
      &mut tokens_ctor_tokens,
      &mut array_tokens,
      &mut thread_tokens,
    );
    let number = irq.number.value() as usize;
    let irq_trait = Ident::from(format!("Irq{}", number));
    let bundle = Ident::from(format!("IrqBundle{}", number / 32));
    thread_tokens.push(quote! {
      impl<T: rt::ThreadTag> rt::IrqToken<T> for #struct_ident<T> {
        type Bundle = rt::#bundle;

        const IRQ_NUMBER: usize = #number;
      }

      impl<T: rt::ThreadTag> rt::#irq_trait<T> for #struct_ident<T> {}
    });
    irq_ident[number] = field_ident;
  }
  let new_ident = Ident::new("new", call_site);
  let irq_ident = &irq_ident;

  let expanded = quote! {
    mod rt {
      extern crate core;
      extern crate drone_core;
      extern crate drone_stm32;

      pub use self::core::marker::PhantomData;
      pub use self::drone_core::thread::ThreadTokens;
      pub use self::drone_stm32::thread::irq::*;
      pub use self::drone_stm32::thread::prelude::*;
      pub use self::drone_stm32::thread::vtable::{Handler, Reserved,
                                                  ResetHandler};
    }

    #(#vtable_attrs)*
    #vtable_vis struct #vtable_ident {
      reset: rt::ResetHandler,
      nmi: Option<rt::Handler>,
      hard_fault: Option<rt::Handler>,
      mem_manage: Option<rt::Handler>,
      bus_fault: Option<rt::Handler>,
      usage_fault: Option<rt::Handler>,
      _reserved0: [rt::Reserved; 4],
      sv_call: Option<rt::Handler>,
      debug: Option<rt::Handler>,
      _reserved1: [rt::Reserved; 1],
      pend_sv: Option<rt::Handler>,
      sys_tick: Option<rt::Handler>,
      #(#irq_ident: Option<rt::Handler>,)*
    }

    impl #vtable_ident {
      /// Creates a new vector table.
      #[inline(always)]
      pub const fn #new_ident(reset: rt::ResetHandler) -> Self {
        Self {
          #(#vtable_ctor_tokens,)*
          ..Self {
            reset,
            nmi: None,
            hard_fault: None,
            mem_manage: None,
            bus_fault: None,
            usage_fault: None,
            _reserved0: [rt::Reserved::Vector; 4],
            sv_call: None,
            debug: None,
            _reserved1: [rt::Reserved::Vector; 1],
            pend_sv: None,
            sys_tick: None,
            #(#irq_ident: None,)*
          }
        }
      }
    }

    #(#tokens_attrs)*
    #tokens_vis struct #tokens_ident {
      #(#tokens_tokens),*
    }

    impl rt::ThreadTokens for #tokens_ident {
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
  mut index: usize,
  exception: &Exception,
  thread_ident: &Ident,
  vtable_ctor_tokens: &mut Vec<Tokens>,
  tokens_tokens: &mut Vec<Tokens>,
  tokens_ctor_tokens: &mut Vec<Tokens>,
  array_tokens: &mut Vec<Tokens>,
  thread_tokens: &mut Vec<Tokens>,
) -> (Ident, Ident) {
  let call_site = Span::call_site();
  let &Exception {
    ref attrs,
    ref vis,
    ref ident,
  } = exception;
  let vtable_field_ident = Ident::from(ident.as_ref().to_snake_case());
  let struct_ident = Ident::new(&ident.as_ref().to_pascal_case(), call_site);
  let field_ident = Ident::new(vtable_field_ident.as_ref(), call_site);
  index += 1;
  vtable_ctor_tokens.push(quote! {
    #vtable_field_ident: Some(
      <#struct_ident<rt::Ltt> as rt::ThreadToken<rt::Ltt>>::handler,
    )
  });
  tokens_tokens.push(quote! {
    #(#attrs)*
    #vis #field_ident: #struct_ident<rt::Ctt>
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
    #vis struct #struct_ident<T: rt::ThreadTag>(rt::PhantomData<T>);

    impl<T: rt::ThreadTag> #struct_ident<T> {
      #[inline(always)]
      unsafe fn new() -> Self {
        #struct_ident(rt::PhantomData)
      }
    }

    impl<T: rt::ThreadTag> rt::ThreadToken<T> for #struct_ident<T> {
      type Thread = #thread_ident;

      const THREAD_NUMBER: usize = #index;
    }

    impl<T: rt::ThreadTag> AsRef<#thread_ident> for #struct_ident<T> {
      #[inline(always)]
      fn as_ref(&self) -> &#thread_ident {
        rt::ThreadToken::as_thd(self)
      }
    }

    impl From<#struct_ident<rt::Ctt>> for #struct_ident<rt::Ttt> {
      #[inline(always)]
      fn from(_token: #struct_ident<rt::Ctt>) -> Self {
        unsafe { Self::new() }
      }
    }

    impl From<#struct_ident<rt::Ctt>> for #struct_ident<rt::Ltt> {
      #[inline(always)]
      fn from(_token: #struct_ident<rt::Ctt>) -> Self {
        unsafe { Self::new() }
      }
    }

    impl From<#struct_ident<rt::Ttt>> for #struct_ident<rt::Ltt> {
      #[inline(always)]
      fn from(_token: #struct_ident<rt::Ttt>) -> Self {
        unsafe { Self::new() }
      }
    }
  });
  (struct_ident, vtable_field_ident)
}
