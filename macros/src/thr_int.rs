use inflector::Inflector;
use proc_macro::TokenStream;
use syn::{
  parse::{Parse, ParseStream, Result},
  Attribute, Ident, LitInt, Visibility,
};

struct ThrInt {
  attrs: Vec<Attribute>,
  vis: Visibility,
  ident: Ident,
  number: LitInt,
}

impl Parse for ThrInt {
  fn parse(input: ParseStream) -> Result<Self> {
    let attrs = input.call(Attribute::parse_outer)?;
    let vis = input.parse()?;
    input.parse::<Token![trait]>()?;
    let ident = input.parse()?;
    input.parse::<Token![:]>()?;
    let number = input.parse()?;
    input.parse::<Token![;]>()?;
    Ok(Self {
      attrs,
      vis,
      ident,
      number,
    })
  }
}

pub fn proc_macro(input: TokenStream) -> TokenStream {
  let ThrInt {
    attrs,
    vis,
    ident,
    number,
  } = parse_macro_input!(input as ThrInt);
  let int_name = format!("INT_{}", ident);
  let name_ident = new_ident!("{}", int_name.to_pascal_case());
  let number_ident = new_ident!("Int{}", number.value());

  let expanded = quote! {
    #(#attrs)*
    #[marker]
    #vis trait #number_ident<T: ThrTag>: IntToken<T> {}

    #[allow(unused_imports)]
    #vis use self::#number_ident as #name_ident;
  };
  expanded.into()
}
