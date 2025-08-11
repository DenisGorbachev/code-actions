use crate::functions::code_generation_helpers::{create_derive_attribute_from_syn_path, create_use_statements_from_syn_use_tree};
use crate::types::config::Config;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Using `fmt_derive::Display` because it formats the error using the Debug impl (which includes the error name & all fields)
pub fn get_error_struct_token_stream(name: Ident, config: &Config) -> TokenStream {
    get_error_struct_token_stream_with_config(name, config)
}

pub fn get_error_struct_token_stream_with_config(name: Ident, config: &Config) -> TokenStream {
    let type_name = name.to_string();
    let extra_derives = config.get_extra_derives_for_name(&type_name);
    let derive_attr = create_derive_attribute_from_syn_path(extra_derives.iter());

    let extra_uses = config.get_extra_use_statements_for_name(&type_name);
    let extra_use_statements = create_use_statements_from_syn_use_tree(extra_uses);

    quote! {
        #extra_use_statements

        #derive_attr
        pub struct #name {}

        impl #name {}
    }
}
