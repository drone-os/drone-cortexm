use drone_macros_core::parse_own_name;
use failure::{err_msg, Error};
use inflector::Inflector;
use proc_macro::TokenStream;
use quote::Tokens;
use syn::{parse_token_trees, Ident, IntTy, Lit, Token, TokenTree};

pub(crate) fn interrupt(input: TokenStream) -> Result<Tokens, Error> {
  let input = parse_token_trees(&input.to_string()).map_err(err_msg)?;
  let mut input = input.into_iter();
  let (attrs, name) = parse_own_name(&mut input)?;
  let name =
    name.ok_or_else(|| format_err!("Unexpected end of macro invokation"))?;
  let number = match input.next() {
    Some(TokenTree::Token(Token::Literal(Lit::Int(
      number,
      IntTy::Unsuffixed,
    )))) => match input.next() {
      Some(TokenTree::Token(Token::Semi)) => number,
      token => {
        Err(format_err!("Invalid token after `{}`: {:?}", number, token))?
      }
    },
    token => Err(format_err!("Invalid token: {:?}", token))?,
  };

  let irq_name = format!("IRQ_{}", name);
  let trait_name = Ident::new(irq_name.to_pascal_case());
  let trait_number = Ident::new(format!("Irq{}", number));

  Ok(quote! {
    #(#attrs)*
    pub trait #trait_number<T: Thread>: ThreadInterrupt<T> {}
    pub use self::#trait_number as #trait_name;
  })
}
