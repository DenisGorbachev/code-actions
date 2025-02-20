use proc_macro2::{Ident, TokenStream};

pub trait ToModuleTokenStream {
    fn to_module_token_stream(&self, ident: Ident) -> TokenStream;
}
