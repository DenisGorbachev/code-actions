use crate::types::type_name::TypeName;
use proc_macro2::TokenStream;
use quote::ToTokens;

impl ToTokens for TypeName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}
