use drone_macros_core::{parse_extern_name, parse_own_name};
use failure::{err_msg, Error};
use inflector::Inflector;
use proc_macro::TokenStream;
use quote::Tokens;
use syn::{parse_token_trees, Ident, IntTy, Lit, Token, TokenTree};

pub(crate) fn vtable(input: TokenStream) -> Result<Tokens, Error> {
  let input = parse_token_trees(&input.to_string()).map_err(err_msg)?;
  let mut input = input.into_iter();
  let mut threads = Vec::new();
  let (attrs, name) = parse_own_name(&mut input)?;
  let (tokens_attrs, tokens_name) = parse_own_name(&mut input)?;
  let (static_attrs, static_name) = parse_own_name(&mut input)?;
  let thread_local = parse_extern_name(&mut input)?;
  let name =
    name.ok_or_else(|| format_err!("Unexpected end of macro invokation"))?;
  let tokens_name = tokens_name
    .ok_or_else(|| format_err!("Unexpected end of macro invokation"))?;
  let static_name = static_name
    .ok_or_else(|| format_err!("Unexpected end of macro invokation"))?;
  let thread_local = thread_local
    .ok_or_else(|| format_err!("Unexpected end of macro invokation"))?;
  'outer: loop {
    let mut attrs = Vec::new();
    loop {
      match input.next() {
        Some(TokenTree::Token(Token::DocComment(ref string)))
          if string.starts_with("///") =>
        {
          let string = string.trim_left_matches("///");
          attrs.push(quote!(#[doc = #string]));
        }
        Some(TokenTree::Token(Token::Pound)) => match input.next() {
          Some(TokenTree::Delimited(delimited)) => {
            attrs.push(quote!(# #delimited))
          }
          token => Err(format_err!("Invalid tokens after `#`: {:?}", token))?,
        },
        Some(TokenTree::Token(Token::Ident(name))) => {
          match input.next() {
            Some(TokenTree::Token(Token::Semi)) => (),
            token => {
              Err(format_err!("Invalid token after `{}`: {:?}", name, token))?
            }
          }
          threads.push((attrs, None, name));
          break;
        }
        Some(TokenTree::Token(Token::Literal(Lit::Int(
          number,
          IntTy::Unsuffixed,
        )))) => {
          match input.next() {
            Some(TokenTree::Token(Token::Colon)) => (),
            token => {
              Err(format_err!("Invalid token after `{}`: {:?}", number, token))?
            }
          }
          let name = match input.next() {
            Some(TokenTree::Token(Token::Ident(name))) => name,
            token => Err(format_err!(
              "Invalid token after `{}:`: {:?}",
              number,
              token
            ))?,
          };
          match input.next() {
            Some(TokenTree::Token(Token::Semi)) => (),
            token => {
              Err(format_err!("Invalid token after `{}`: {:?}", name, token))?
            }
          }
          threads.push((attrs, Some(number), name));
          break;
        }
        None => break 'outer,
        token => Err(format_err!("Invalid token: {:?}", token))?,
      }
    }
  }

  let irq_count = threads
    .iter()
    .filter_map(|&(_, number, _)| number)
    .max()
    .map(|x| x + 1)
    .unwrap_or(0);
  let mut irq_name = (0..irq_count)
    .map(|n| Ident::new(format!("_irq{}", n)))
    .collect::<Vec<_>>();
  let thread_count = Lit::Int(threads.len() as u64 + 1, IntTy::Unsuffixed);
  let mut thread_tokens = Vec::new();
  let mut thread_ctor_tokens = Vec::new();
  let mut thread_static_tokens = Vec::new();
  let mut thread_tokens_struct_tokens = Vec::new();
  let mut thread_tokens_impl_tokens = Vec::new();
  thread_static_tokens.push(quote!(#thread_local::new(0)));
  for (index, thread) in threads.into_iter().enumerate() {
    let (
      tokens,
      ctor_tokens,
      static_tokens,
      tokens_struct_tokens,
      tokens_impl_tokens,
    ) = parse_thread(index, thread, &thread_local, &mut irq_name)?;
    thread_tokens.push(tokens);
    thread_ctor_tokens.push(ctor_tokens);
    thread_static_tokens.push(static_tokens);
    thread_tokens_struct_tokens.push(tokens_struct_tokens);
    thread_tokens_impl_tokens.push(tokens_impl_tokens);
  }
  let irq_name = &irq_name;

  Ok(quote! {
    #[allow(unused_imports)]
    use ::drone_core::thread::ThreadTokens;
    #[allow(unused_imports)]
    use ::drone_cortex_m::thread::{Handler, Reserved, ResetHandler};
    #[allow(unused_imports)]
    use ::drone_cortex_m::thread::interrupts::*;
    #[allow(unused_imports)]
    use ::drone_cortex_m::thread::prelude::*;

    #(#attrs)*
    #[allow(dead_code)]
    pub struct #name {
      reset: ResetHandler,
      nmi: Option<Handler>,
      hard_fault: Option<Handler>,
      mem_manage: Option<Handler>,
      bus_fault: Option<Handler>,
      usage_fault: Option<Handler>,
      _reserved0: [Reserved; 4],
      sv_call: Option<Handler>,
      debug: Option<Handler>,
      _reserved1: [Reserved; 1],
      pend_sv: Option<Handler>,
      sys_tick: Option<Handler>,
      #(
        #irq_name: Option<Handler>,
      )*
    }

    impl #name {
      /// Creates a new vector table.
      #[inline(always)]
      pub const fn new(reset: ResetHandler) -> #name {
        #name {
          #(#thread_ctor_tokens,)*
          ..#name {
            reset,
            nmi: None,
            hard_fault: None,
            mem_manage: None,
            bus_fault: None,
            usage_fault: None,
            _reserved0: [Reserved::Vector; 4],
            sv_call: None,
            debug: None,
            _reserved1: [Reserved::Vector; 1],
            pend_sv: None,
            sys_tick: None,
            #(
              #irq_name: None,
            )*
          }
        }
      }
    }

    #(#tokens_attrs)*
    pub struct #tokens_name {
      #(#thread_tokens_struct_tokens),*
    }

    impl ThreadTokens<#thread_local> for #tokens_name {
      unsafe fn new() -> Self {
        Self {
          #(#thread_tokens_impl_tokens),*
        }
      }
    }

    #(#static_attrs)*
    static mut #static_name: [#thread_local; #thread_count] = [
      #(#thread_static_tokens),*
    ];

    #(#thread_tokens)*
  })
}

fn parse_thread(
  index: usize,
  (attrs, number, name): (Vec<Tokens>, Option<u64>, Ident),
  thread_local: &Ident,
  irq_name: &mut [Ident],
) -> Result<(Tokens, Tokens, Tokens, Tokens, Tokens), Error> {
  let field_name = Ident::new(name.as_ref().to_snake_case());
  let struct_name = Ident::new(name.as_ref().to_pascal_case());
  let index = Lit::Int(index as u64 + 1, IntTy::Unsuffixed);
  let attrs = &attrs;

  if let Some(number) = number {
    irq_name[number as usize] = field_name.clone();
  }

  let interrupt = match number {
    Some(number) => {
      let irq_trait = Ident::new(format!("Irq{}", number));
      let number = Lit::Int(number, IntTy::Unsuffixed);
      quote! {
        impl InterruptNumber for #struct_name {
          const INTERRUPT_NUMBER: usize = #number;
        }

        impl #irq_trait for #struct_name {}
      }
    }
    None => {
      let irq_trait = Ident::new(format!("Irq{}", struct_name));
      quote! {
        impl #irq_trait for #struct_name {}
      }
    }
  };

  Ok((
    quote! {
      #(#attrs)*
      pub struct #struct_name;

      impl ThreadNumber for #struct_name {
        const THREAD_NUMBER: usize = #index;
      }

      #interrupt
    },
    quote! {
      #field_name: Some(ThreadToken::<#thread_local, #struct_name>::handler)
    },
    quote! {
      #thread_local::new(#index)
    },
    quote! {
      #(#attrs)*
      pub #field_name: ThreadToken<#thread_local, #struct_name>
    },
    quote! {
      #field_name: ThreadToken::<#thread_local, #struct_name>::new()
    },
  ))
}
