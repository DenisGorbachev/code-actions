//! There are two kinds of newtype structs: subtypes and wrappers
//! Subtypes constrain the inner value (so that not every inner value could be wrapped in a subtype)
//! Wrappers do not constrain the inner value (they exist solely for implementing foreign traits on foreign types)

use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// A subtype must derive `TryFrom` instead of `From` (to ensure that constraint is upheld)
/// A subtype must not derive `DerefMut` (to ensure that constraint is upheld)
/// A wrapper must derive `From` (to allow simple creation of a newtype value from the underlying value)
/// A wrapper must derive `DerefMut` (to allow simple mutation of the underlying value)
pub fn get_newtype_wrapper_struct_token_stream(name: Ident) -> TokenStream {
    quote! {
        use derive_more::{Deref, DerefMut, From, Into};
        use derive_new::new;

        #[derive(new, Deref, DerefMut, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
        #[repr(transparent)]
        pub struct #name {
            inner: ()
        }

        impl #name {}
    }
}
