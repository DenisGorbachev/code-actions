use crate::types::outcome::Outcome;
use derive_more::{Display, Error};
use fs_err::File;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use stub_macro::stub;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::std::path::file_stem::FileStem;
use crate::functions::format::format_token_stream_prettyplease;
use crate::generate_file::generate_module_file;
use crate::types::type_name::TypeName;

pub fn generate_impl_from(path: impl AsRef<Utf8Path>) -> Outcome<File> {
    generate_module_file(path, get_impl_from_file_contents)
}

#[derive(Error, Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct MustSupportGenerics {}

pub fn get_get_impl_from_token_stream() -> Outcome {
    Err(MustSupportGenerics {})?;
    Ok(())
}

pub fn get_impl_from_file_contents(path: &Utf8Path) -> Outcome<String> {
    let stem = FileStem::try_from(path)?;
    let type_name = TypeName::from(*stem);
    let name = format_ident!("{}", &type_name);
    let content = get_impl_from_token_stream(stub!(), name);
    Ok(format_token_stream_prettyplease(content)?)
}

pub fn get_impl_from_token_stream(source: TokenStream, target: Ident) -> TokenStream {
    let _old = quote! {
        use #source;

        impl #source for #target {
            fn from(source: #source) -> Self {
                todo!()
            }
        }
    };
    todo!()
}
