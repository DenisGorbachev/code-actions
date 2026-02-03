use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::extensions::syn::IdentExt;

pub fn get_fn_token_stream(name: Ident) -> TokenStream {
    let name = name.to_snake_case();
    quote! {
        pub fn #name() {
            todo!()
        }
    }
}
