use crate::functions::code_generation_helpers::{create_derive_attribute, create_use_statements, merge_derives};
use crate::types::config::Config;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn get_regular_enum_token_stream(name: Ident, config: &Config) -> TokenStream {
    let type_name = name.to_string();
    get_regular_enum_token_stream_with_config(name, config, &type_name)
}

pub fn get_regular_enum_token_stream_with_config(name: Ident, config: &Config, type_name: &impl AsRef<str>) -> TokenStream {
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
pub fn get_plain_enum_token_stream(name: Ident, config: &Config) -> TokenStream {
    let type_name = name.to_string();
    get_plain_enum_token_stream_with_config(name, config, &type_name)
}

pub fn get_plain_enum_token_stream_with_config(name: Ident, config: &Config, type_name: &impl AsRef<str>) -> TokenStream {
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

pub fn get_clap_enum_token_stream(name: Ident, config: &Config) -> TokenStream {
    let type_name = name.to_string();
    get_clap_enum_token_stream_with_config(name, config, &type_name)
}

pub fn get_clap_enum_token_stream_with_config(name: Ident, config: &Config, type_name: &impl AsRef<str>) -> TokenStream {
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
