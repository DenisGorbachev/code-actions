use proc_macro2::TokenStream;
use quote::quote;

pub fn create_derive_attribute_from_syn_path<'a>(derives: impl IntoIterator<Item = &'a syn::Path>) -> TokenStream {
    let derive_idents = derives.into_iter();
    quote! { #[derive(#(#derive_idents),*)] }
}

pub fn create_use_statements_from_syn_use_tree(use_statements: impl IntoIterator<Item = syn::UseTree>) -> TokenStream {
    let mut tokens = TokenStream::new();
    for use_tree in use_statements {
        tokens.extend(quote! { use #use_tree; });
    }
    tokens
}
