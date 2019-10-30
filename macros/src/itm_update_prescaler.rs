use drone_config::Config;
use drone_macros_core::compile_error;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Expr, IntSuffix, LitInt,
};

struct ItmUpdatePrescaler {
    hclk: Expr,
}

impl Parse for ItmUpdatePrescaler {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let hclk = input.parse()?;
        Ok(Self { hclk })
    }
}

pub fn proc_macro(input: TokenStream) -> TokenStream {
    let ItmUpdatePrescaler { hclk } = parse_macro_input!(input as ItmUpdatePrescaler);
    let config = match Config::read_from_cargo_manifest_dir() {
        Ok(config) => config,
        Err(err) => compile_error!("{}", err),
    };
    if let Some(probe) = config.probe {
        if let Some(itm) = probe.itm {
            let baud_rate =
                LitInt::new(u64::from(itm.baud_rate), IntSuffix::None, Span::call_site());
            quote!(::drone_cortex_m::itm::update_prescaler(#hclk / #baud_rate - 1)).into()
        } else {
            compile_error!("Missing `probe.itm` section in `Drone.toml`");
        }
    } else {
        compile_error!("Missing `probe` section in `Drone.toml`");
    }
}
