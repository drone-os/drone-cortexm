use proc_macro::TokenStream;
use syn;

pub(crate) fn vtable(input: TokenStream) -> TokenStream {
  let input = syn::parse_token_trees(&input.to_string()).unwrap();
  let mut input = input.into_iter();
  let mut attributes = Vec::new();
  let mut exception_name = Vec::new();
  let mut exception_number = Vec::new();
  let mut exception_attributes = Vec::new();
  'outer: loop {
    let mut inner_attributes = Vec::new();
    loop {
      match input.next() {
        Some(syn::TokenTree::Token(syn::Token::DocComment(string))) => {
          if string.starts_with("//!") {
            let string = string.trim_left_matches("//!");
            attributes.push(quote!(#[doc = #string]));
          } else {
            let string = string.trim_left_matches("///");
            inner_attributes.push(quote!(#[doc = #string]));
          }
        }
        Some(syn::TokenTree::Token(syn::Token::Pound)) => match input.next() {
          Some(syn::TokenTree::Token(syn::Token::Not)) => match input.next() {
            Some(syn::TokenTree::Delimited(delimited)) => {
              attributes.push(quote!(# #delimited))
            }
            token => panic!("Invalid tokens after `#!`: {:?}", token),
          },
          Some(syn::TokenTree::Delimited(delimited)) => {
            inner_attributes.push(quote!(# #delimited))
          }
          token => panic!("Invalid tokens after `#`: {:?}", token),
        },
        Some(syn::TokenTree::Token(syn::Token::Ident(name))) => {
          match input.next() {
            Some(syn::TokenTree::Token(syn::Token::Semi)) => (),
            token => panic!("Invalid token after `{}`: {:?}", name, token),
          }
          exception_attributes.push(inner_attributes);
          exception_number.push(None);
          exception_name.push(name);
          break;
        }
        Some(
          syn::TokenTree::Token(
            syn::Token::Literal(syn::Lit::Int(number, syn::IntTy::Unsuffixed)),
          ),
        ) => {
          match input.next() {
            Some(syn::TokenTree::Token(syn::Token::Colon)) => (),
            token => panic!("Invalid token after `{}`: {:?}", number, token),
          }
          let name = match input.next() {
            Some(syn::TokenTree::Token(syn::Token::Ident(name))) => name,
            token => panic!("Invalid token after `{}:`: {:?}", number, token),
          };
          match input.next() {
            Some(syn::TokenTree::Token(syn::Token::Semi)) => (),
            token => panic!("Invalid token after `{}`: {:?}", name, token),
          }
          exception_attributes.push(inner_attributes);
          exception_number.push(Some(number));
          exception_name.push(name);
          break;
        }
        None => {
          break 'outer;
        }
        token => panic!("Invalid token: {:?}", token),
      }
    }
  }
  let irq_number = exception_number
    .iter()
    .cloned()
    .filter_map(|x| x)
    .max()
    .map(|x| x + 1)
    .unwrap_or(0);
  let mut irq_name = (0..irq_number)
    .map(|n| syn::Ident::new(format!("_irq{}", n)))
    .collect::<Vec<_>>();
  exception_number
    .iter()
    .zip(exception_name.iter())
    .filter_map(|(number, name)| {
      number.map(|number| (number as usize, name))
    })
    .for_each(|(number, name)| {
      irq_name[number] = name.clone();
    });
  let exception_name2 = exception_name.clone();
  let exception_name3 = exception_name.clone();
  let exception_name4 = exception_name.clone();
  let exception_name5 = exception_name.clone();
  let exception_attributes2 = exception_attributes.clone();
  let irq_name2 = irq_name.clone();

  let output = quote! {
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
      /// Constructs a `VectorTable`.
      pub const fn new(reset: ResetHandler) -> VectorTable {
        VectorTable {
          #(
            #exception_name: Some(#exception_name2::handler),
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

    #(
      #(#exception_attributes)*
      pub mod #exception_name3 {
        use drone::routine::Routine;

        /// The routine chain.
        pub static mut ROUTINE: Routine = Routine::new();

        /// The routine handler.
        ///
        /// # Safety
        ///
        /// Should be called only by hardware.
        pub unsafe extern "C" fn handler() {
          ROUTINE.invoke();
        }
      }

      #(#exception_attributes2)*
      pub fn #exception_name4() -> &'static mut ::drone::routine::Routine {
        unsafe { &mut #exception_name5::ROUTINE }
      }
    )*
  };
  output.parse().unwrap()
}
