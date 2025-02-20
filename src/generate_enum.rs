use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn get_regular_enum_token_stream(name: Ident) -> TokenStream {
    quote! {
        use derive_more::From;

        #[derive(From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
        pub enum #name {}

        impl #name {}
    }
}

/// Plain enums are those enums that contain only constant variants without arguments
/// The use statement ("use #name::*") is useful for functions that match on enum variants
pub fn get_plain_enum_token_stream(name: Ident) -> TokenStream {
    quote! {
        use strum::Display;
        #[allow(dead_code)]
        pub use #name::*;

        #[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
        pub enum #name {}

        impl #name {}
    }
}

pub fn get_clap_enum_token_stream(name: Ident) -> TokenStream {
    quote! {
        use std::io::Write;
        use clap::Parser;

        #[derive(Parser, Clone, Debug)]
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
