use crate::types::type_name::TypeName;
use quote::ToTokens;

impl ToTokens for TypeName {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}
