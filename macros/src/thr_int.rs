use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Attribute, Ident, LitInt, Visibility};
use syn::synom::Synom;

struct ThrInt {
  attrs: Vec<Attribute>,
  vis: Visibility,
  ident: Ident,
  number: LitInt,
}

impl Synom for ThrInt {
  named!(parse -> Self, do_parse!(
    attrs: many0!(Attribute::parse_outer) >>
    vis: syn!(Visibility) >>
    keyword!(trait) >>
    ident: syn!(Ident) >>
    punct!(:) >>
    number: syn!(LitInt) >>
    punct!(;) >>
    (ThrInt { attrs, vis, ident, number })
  ));
}

pub fn proc_macro(input: TokenStream) -> TokenStream {
  let call_site = Span::call_site();
  let ThrInt {
    attrs,
    vis,
    ident,
    number,
  } = try_parse!(call_site, input);
  let int_name = format!("INT_{}", ident);
  let name_ident = Ident::new(&int_name.to_pascal_case(), call_site);
  let number_ident = Ident::new(&format!("Int{}", number.value()), call_site);
  let expanded = quote_spanned! { call_site =>
    #(#attrs)*
    #vis trait #number_ident<T: ThrTag>: IntToken<T> {}

    #[allow(unused_imports)]
    #vis use self::#number_ident as #name_ident;
  };
  expanded.into()
}
