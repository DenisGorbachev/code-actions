use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn get_subtype_struct_token_stream(name: Ident) -> TokenStream {
    quote! {
        use subtype::subtype;

        #[newline]
        subtype!(
            #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
            pub struct #name(());
        );

        #[newline]
        impl #name {}

    }
}
