use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse, Attribute, Ident, LitInt, Visibility};
use syn::synom::Synom;

struct Interrupt {
  attrs: Vec<Attribute>,
  vis: Visibility,
  ident: Ident,
  number: LitInt,
}

impl Synom for Interrupt {
  named!(parse -> Self, do_parse!(
    attrs: many0!(Attribute::parse_outer) >>
    vis: syn!(Visibility) >>
    keyword!(trait) >>
    ident: syn!(Ident) >>
    punct!(:) >>
    number: syn!(LitInt) >>
    punct!(;) >>
    (Interrupt { attrs, vis, ident, number })
  ));
}

pub fn proc_macro(input: TokenStream) -> TokenStream {
  let call_site = Span::call_site();
  let Interrupt {
    attrs,
    vis,
    ident,
    number,
  } = parse::<Interrupt>(input).unwrap();
  let irq_name = format!("IRQ_{}", ident);
  let name_ident = Ident::new(&irq_name.to_pascal_case(), call_site);
  let number_ident = Ident::new(&format!("Irq{}", number.value()), call_site);
  let expanded = quote_spanned! { call_site =>
    #(#attrs)*
    #vis trait #number_ident<T: ThreadTag>: IrqToken<T> {}

    #[allow(unused_imports)]
    #vis use self::#number_ident as #name_ident;
  };
  expanded.into()
}
