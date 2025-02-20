use crate::types::outcome::Outcome;
use fs_err::File;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::ops::Deref;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::std::path::file_stem::FileStem;
use crate::extensions::syn::IdentExt;
use crate::functions::format::format_token_stream_prettyplease;
use crate::generate_file::generate_module_file;
use crate::get_relative_path::get_relative_path_anchor_label_rs;
use crate::types::label::LabelSlice;

pub fn generate_fn_from_anchor_label(anchor: &Utf8Path, label: &LabelSlice) -> Outcome<File> {
    let path = get_relative_path_anchor_label_rs(anchor, label)?;
    generate_fn_from_path(path)
}

pub fn generate_fn_from_path(path: impl AsRef<Utf8Path>) -> Outcome<File> {
    generate_module_file(path, get_fn_file_contents)
}

pub fn get_fn_file_contents(path: &Utf8Path) -> Outcome<String> {
    let stem = FileStem::try_from(path)?;
    let name = format_ident!("{}", stem.deref());
    let content = get_fn_token_stream(name);
    Ok(format_token_stream_prettyplease(content)?)
}

pub fn get_fn_token_stream(name: Ident) -> TokenStream {
    let name = name.to_snake_case();
    quote! {
        pub fn #name() {
            todo!()
        }
    }
}
