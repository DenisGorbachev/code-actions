use crate::types::config::Config;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn get_regular_enum_token_stream(name: Ident) -> TokenStream {
    let config = Config::default();
    let type_name = name.to_string();
    get_regular_enum_token_stream_with_config(name, &config, &type_name)
}

pub fn get_regular_enum_token_stream_with_config(name: Ident, config: &Config, type_name: &str) -> TokenStream {
    let base_derives = &[
        "From",
        "Ord",
        "PartialOrd",
        "Eq",
        "PartialEq",
        "Hash",
        "Clone",
        "Copy",
        "Debug",
    ];
    let extra_derives = config.get_extra_derives_for_name(type_name);
    let all_derives = merge_derives(base_derives, &extra_derives);
    let derive_attr = create_derive_attribute(&all_derives);

    let extra_uses = config.get_extra_use_statements_for_name(type_name);
    let extra_use_statements = create_use_statements(&extra_uses);

    quote! {
        use derive_more::From;
        #extra_use_statements

        #derive_attr
        pub enum #name {}

        impl #name {}
    }
}

/// Plain enums are those enums that contain only constant variants without arguments
/// The use statement ("use #name::*") is useful for functions that match on enum variants
pub fn get_plain_enum_token_stream(name: Ident) -> TokenStream {
    let config = Config::default();
    let type_name = name.to_string();
    get_plain_enum_token_stream_with_config(name, &config, &type_name)
}

pub fn get_plain_enum_token_stream_with_config(name: Ident, config: &Config, type_name: &str) -> TokenStream {
    let base_derives = &[
        "Display",
        "Ord",
        "PartialOrd",
        "Eq",
        "PartialEq",
        "Hash",
        "Clone",
        "Copy",
        "Debug",
    ];
    let extra_derives = config.get_extra_derives_for_name(type_name);
    let all_derives = merge_derives(base_derives, &extra_derives);
    let derive_attr = create_derive_attribute(&all_derives);

    let extra_uses = config.get_extra_use_statements_for_name(type_name);
    let extra_use_statements = create_use_statements(&extra_uses);

    quote! {
        use strum::Display;
        #[allow(unused_imports)]
        use #name::*;
        #extra_use_statements

        #derive_attr
        pub enum #name {}

        impl #name {}
    }
}

pub fn get_clap_enum_token_stream(name: Ident) -> TokenStream {
    let config = Config::default();
    let type_name = name.to_string();
    get_clap_enum_token_stream_with_config(name, &config, &type_name)
}

pub fn get_clap_enum_token_stream_with_config(name: Ident, config: &Config, type_name: &str) -> TokenStream {
    let base_derives = &["Parser", "Clone", "Debug"];
    let extra_derives = config.get_extra_derives_for_name(type_name);
    let all_derives = merge_derives(base_derives, &extra_derives);
    let derive_attr = create_derive_attribute(&all_derives);

    let extra_uses = config.get_extra_use_statements_for_name(type_name);
    let extra_use_statements = create_use_statements(&extra_uses);

    quote! {
        use std::io::Write;
        use clap::Parser;
        #extra_use_statements

        #derive_attr
        pub enum #name {
            Placeholder
        }

        impl #name {
            pub async fn run(self, stdout: &mut impl Write, stderr: &mut impl Write) -> Result<(), ()> {
                use #name::*;
                match self {
                    Placeholder => todo!()
                }
            }
        }
    }
}

fn merge_derives(base_derives: &[&str], extra_derives: &[String]) -> Vec<String> {
    let mut all_derives: Vec<String> = base_derives.iter().map(|s| s.to_string()).collect();

    for derive in extra_derives {
        if !all_derives.contains(derive) {
            all_derives.push(derive.clone());
        }
    }

    all_derives
}

fn create_derive_attribute(derives: &[String]) -> TokenStream {
    if derives.is_empty() {
        return quote! {};
    }

    let derive_idents: Vec<_> = derives.iter().map(|d| format_ident!("{}", d)).collect();

    quote! { #[derive(#(#derive_idents),*)] }
}

fn create_use_statements(use_statements: &[String]) -> TokenStream {
    if use_statements.is_empty() {
        return quote! {};
    }

    let mut tokens = TokenStream::new();
    for use_stmt in use_statements {
        let use_tokens = use_stmt
            .parse::<TokenStream>()
            .unwrap_or_else(|_| quote! { compile_error!(concat!("Invalid use statement: ", #use_stmt)); });
        tokens.extend(quote! { use #use_tokens; });
    }

    tokens
}
