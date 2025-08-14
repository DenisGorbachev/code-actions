use fs_err::{File, create_dir};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::types::outcome::Outcome;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::generate_file::generate_module_file;
use crate::get_relative_path::{get_relative_path_anchor_stem_rs, get_relative_path_anchor_subdir_label_rs, get_relative_path_anchor_subdir_name_suffix};
use crate::types::label::LabelSlice;

pub fn generate_module_from_anchor_subdir_name_suffix(anchor: &Utf8Path, subdir: &str, name: &str, suffix: &str) -> Outcome<File> {
    let path = get_relative_path_anchor_subdir_name_suffix(anchor, subdir, name, suffix)?;
    generate_module_from_path(path)
}

pub fn generate_module_from_anchor_subdir_label(anchor: &Utf8Path, subdir: &str, label: &LabelSlice) -> Outcome<File> {
    let path = get_relative_path_anchor_subdir_label_rs(anchor, subdir, label)?;
    generate_module_from_path(path)
}

pub fn generate_module_from_anchor_stem(anchor: &Utf8Path, stem: &str) -> Outcome<File> {
    let path = get_relative_path_anchor_stem_rs(anchor, stem)?;
    generate_module_from_path(path)
}

pub fn generate_module_from_path(path: impl AsRef<Utf8Path>) -> Outcome<File> {
    generate_module_file(path, get_module_file_contents)
}

pub fn generate_module_with_dir_from_parent_dir_and_stem(parent_dir: impl AsRef<Utf8Path>, stem: &str) -> Outcome {
    let parent_dir = parent_dir.as_ref();
    let filename = format!("{stem}.rs");
    let dirname = stem;
    let dir = parent_dir.join(dirname);
    let file = parent_dir.join(filename);
    create_dir(dir)?;
    generate_module_from_path(file)?;
    Ok(())
}

pub fn get_module_file_contents(_path: &Utf8Path) -> Outcome<String> {
    Ok(String::new())
}

pub fn get_empty_module_token_stream(_name: Ident) -> TokenStream {
    quote! {}
}
