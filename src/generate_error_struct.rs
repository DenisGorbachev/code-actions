use crate::functions::code_generation_helpers::{create_derive_attribute, create_use_statements, merge_derives};
use crate::types::config::Config;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Using `fmt_derive::Display` because it formats the error using the Debug impl (which includes the error name & all fields)
pub fn get_error_struct_token_stream(name: Ident, config: &Config) -> TokenStream {
    get_error_struct_token_stream_with_config(name, config)
}

pub fn get_error_struct_token_stream_with_config(name: Ident, config: &Config) -> TokenStream {
    let type_name = name.to_string();
    let base_derives = &[
        "new",
        "Error",
        "Display",
        "From",
        "Into",
        "Ord",
        "PartialOrd",
        "Eq",
        "PartialEq",
        "Hash",
        "Clone",
        "Debug",
    ];
    let extra_derives = config.get_extra_derives_for_name(&type_name);
    let all_derives = merge_derives(base_derives, &extra_derives);
    let derive_attr = create_derive_attribute(&all_derives);

    let extra_uses = config.get_extra_use_statements_for_name(&type_name);
    let extra_use_statements = create_use_statements(&extra_uses);

    quote! {
        use derive_more::{Error, From, Into};
        use derive_new::new;
        use fmt_derive::Display;
        #extra_use_statements

        #derive_attr
        pub struct #name {}

        impl #name {}
    }
}
