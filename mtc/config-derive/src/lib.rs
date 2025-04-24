use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr, parse_macro_input};

#[proc_macro_derive(Configuration, attributes(config))]
pub fn derive_configuration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Parse attributes for configuration options
    let mut config_name: Option<String> = None;
    let mut format: Option<String> = None;
    for attr in &input.attrs {
        if attr.path().is_ident("config") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    config_name = Some(s.value());
                    Ok(())
                } else if meta.path.is_ident("format") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    let v = s.value();
                    format = match v.as_str() {
                        "json" => Some("json".to_string()),
                        "toml" => Some("toml".to_string()),
                        "yaml" => Some("yaml".to_string()),
                        _ => None,
                    };

                    Ok(())
                } else {
                    Err(meta.error("unsupported repr"))
                }
            });
        }
    }

    // Generate config_name implementation
    let config_name_impl = if let Some(name_str) = config_name {
        quote! {
            fn config_name() -> String {
                String::from(#name_str)
            }
        }
    } else {
        quote! {
            fn config_name() -> String {
                stringify!(#name).to_lowercase()
            }
        }
    };

    // Generate format implementation if specified
    let format_impl = if let Some(format_tokens) = format {
        quote! {
            fn format() -> mtc_config::ConfigFormat {
                #format_tokens.into()
            }
        }
    } else {
        quote! {}
    };

    // Generate the implementation
    let expanded = quote! {
        impl mtc_config::Configuration for #name {
            #config_name_impl
            #format_impl
        }
    };

    TokenStream::from(expanded)
}
