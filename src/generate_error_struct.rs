use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Using `fmt_derive::Display` because it formats the error using the Debug impl (which includes the error name & all fields)
pub fn get_error_struct_token_stream(name: Ident) -> TokenStream {
    quote! {
        use derive_more::{Error, From, Into};
        use derive_new::new;
        use fmt_derive::Display;

        #[derive(new, Error, Display, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
        pub struct #name {}

        impl #name {}
    }
}
