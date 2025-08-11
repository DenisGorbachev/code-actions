use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn merge_derives(base_derives: &[&str], extra_derives: &[String]) -> Vec<String> {
    let mut all_derives: Vec<String> = base_derives.iter().map(|s| s.to_string()).collect();

    for derive in extra_derives {
        if !all_derives.contains(derive) {
            all_derives.push(derive.clone());
        }
    }

    all_derives
}

pub fn create_derive_attribute(derives: &[String]) -> TokenStream {
    if derives.is_empty() {
        return quote! {};
    }

    let derive_idents: Vec<_> = derives.iter().map(|d| format_ident!("{}", d)).collect();

    quote! { #[derive(#(#derive_idents),*)] }
}

pub fn create_use_statements(use_statements: &[String]) -> TokenStream {
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
