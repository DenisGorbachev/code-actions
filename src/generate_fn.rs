use crate::functions::code_generation_helpers::create_use_statements;
use crate::types::config::Config;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::extensions::syn::IdentExt;

pub fn get_fn_token_stream(name: Ident, config: &Config) -> TokenStream {
    get_fn_token_stream_with_config(name, config)
}

pub fn get_fn_token_stream_with_config(name: Ident, config: &Config) -> TokenStream {
    let type_name = name.to_string();
    let snake_name = name.to_snake_case();
    let extra_uses = config.get_extra_use_statements_for_name(&type_name);
    let extra_use_statements = create_use_statements(&extra_uses);

    quote! {
        #extra_use_statements

        pub fn #snake_name() {
            todo!()
        }
    }
}
