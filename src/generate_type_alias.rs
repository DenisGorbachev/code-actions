use fs_err::File;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::types::outcome::Outcome;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::std::path::file_stem::FileStem;
use crate::functions::format::format_token_stream_prettyplease;
use crate::generate_file::generate_module_file;
use crate::get_relative_path::get_relative_path_anchor_label_rs;
use crate::types::label::LabelSlice;
use crate::types::type_name::TypeName;

pub fn generate_type_alias_from_path(path: impl AsRef<Utf8Path>) -> Outcome<File> {
    generate_module_file(path, get_type_alias_file_contents)
}

pub fn generate_type_alias_from_anchor_label(anchor: &Utf8Path, label: &LabelSlice) -> Outcome<File> {
    let path = get_relative_path_anchor_label_rs(anchor, label)?;
    generate_type_alias_from_path(path)
}

pub fn get_type_alias_file_contents(path: &Utf8Path) -> Outcome<String> {
    let stem = FileStem::try_from(path)?;
    let type_name = TypeName::from(*stem);
    let name = format_ident!("{}", &type_name);
    let content = get_type_alias_token_stream(name);
    Ok(format_token_stream_prettyplease(content)?)
}

pub fn get_type_alias_token_stream(name: Ident) -> TokenStream {
    quote! {
        pub type #name = ();
    }
}
