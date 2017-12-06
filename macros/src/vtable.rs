use failure::{err_msg, Error};
use inflector::Inflector;
use proc_macro::TokenStream;
use quote::Tokens;
use syn::{parse_token_trees, Ident, IntTy, Lit, Token, TokenTree};

pub(crate) fn vtable(input: TokenStream) -> Result<Tokens, Error> {
  let input = parse_token_trees(&input.to_string()).map_err(err_msg)?;
  let mut input = input.into_iter();
  let mut attributes = Vec::new();
  let mut thread_id = Vec::new();
  let mut thread_name = Vec::new();
  let mut thread_const = Vec::new();
  let mut thread_number = Vec::new();
  let mut thread_attributes = Vec::new();
  let mut thread_count = 0;
  'outer: loop {
    let mut inner_attributes = Vec::new();
    loop {
      match input.next() {
        Some(TokenTree::Token(Token::DocComment(ref string)))
          if string.starts_with("//!") =>
        {
          let string = string.trim_left_matches("//!");
          attributes.push(quote!(#[doc = #string]));
        }
        Some(TokenTree::Token(Token::DocComment(ref string)))
          if string.starts_with("///") =>
        {
          let string = string.trim_left_matches("///");
          inner_attributes.push(quote!(#[doc = #string]));
        }
        Some(TokenTree::Token(Token::Pound)) => match input.next() {
          Some(TokenTree::Token(Token::Not)) => match input.next() {
            Some(TokenTree::Delimited(delimited)) => {
              attributes.push(quote!(# #delimited))
            }
            token => {
              Err(format_err!("Invalid tokens after `#!`: {:?}", token))?
            }
          },
          Some(TokenTree::Delimited(delimited)) => {
            inner_attributes.push(quote!(# #delimited))
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
          thread_id.push(thread_count);
          thread_name.push(name);
          thread_const.push(None);
          thread_number.push(None);
          thread_attributes.push(inner_attributes);
          thread_count += 1;
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
          thread_id.push(thread_count);
          thread_name.push(Ident::new(name.as_ref().to_snake_case()));
          thread_const.push(Some(name));
          thread_number.push(Some(number));
          thread_attributes.push(inner_attributes);
          thread_count += 1;
          break;
        }
        None => break 'outer,
        token => Err(format_err!("Invalid token: {:?}", token))?,
      }
    }
  }
  let irq_number = thread_number
    .iter()
    .cloned()
    .filter_map(|x| x)
    .max()
    .map(|x| x + 1)
    .unwrap_or(0);
  let mut irq_name = (0..irq_number)
    .map(|n| Ident::new(format!("_irq{}", n)))
    .collect::<Vec<_>>();
  thread_number
    .iter()
    .zip(thread_name.iter())
    .filter_map(|(number, name)| number.map(|number| (number as usize, name)))
    .for_each(|(number, name)| {
      irq_name[number] = name.clone();
    });
  let thread_handler = thread_name
    .iter()
    .map(|name| Ident::new(format!("__{}_handler", name)))
    .collect::<Vec<_>>();
  let thread_count = Lit::Int(thread_count + 1, IntTy::Unsuffixed);
  let thread_id = thread_id
    .into_iter()
    .map(|id| Lit::Int(id + 1, IntTy::Unsuffixed))
    .collect::<Vec<_>>();
  let mut thread_id_with_reset = thread_id.clone();
  thread_id_with_reset.insert(0, Lit::Int(0, IntTy::Unsuffixed));
  let thread_id2 = thread_id.clone();
  let thread_name2 = thread_name.clone();
  let thread_handler2 = thread_handler.clone();
  let irq_name2 = irq_name.clone();

  let thread_number = thread_number
    .into_iter()
    .zip(thread_attributes.iter())
    .zip(thread_const.into_iter())
    .map(|((number, attributes), name)| {
      number
        .into_iter()
        .map(|number| {
          let number = Lit::Int(number, IntTy::Unsuffixed);
          quote! {
            #(#attributes)*
            pub const #name: usize = #number;
          }
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  Ok(quote! {
    use drone_cortex_m::vtable::{Handler, ResetHandler, Reserved};

    #(#attributes)*
    #[allow(dead_code)]
    pub struct VectorTable {
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

    impl VectorTable {
      /// Creates an empty `VectorTable` with `reset` handler.
      pub const fn new(reset: ResetHandler) -> VectorTable {
        VectorTable {
          #(
            #thread_name: Some(#thread_handler),
          )*
          ..VectorTable {
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
              #irq_name2: None,
            )*
          }
        }
      }
    }

    #[allow(dead_code)]
    static mut THREADS: [ThreadLocal; #thread_count] = [
      #(
        ThreadLocal::new(#thread_id_with_reset),
      )*
    ];

    #(
      #[doc(hidden)]
      pub unsafe extern "C" fn #thread_handler2() {
        const THREAD_ID: usize = #thread_id;
        THREADS.get_unchecked_mut(THREAD_ID).resume(THREAD_ID);
      }

      #(#thread_attributes)*
      #[inline(always)]
      pub fn #thread_name2() -> &'static ThreadLocal {
        unsafe { ThreadLocal::get_unchecked(#thread_id2) }
      }

      #(#thread_number)*
    )*
  })
}
