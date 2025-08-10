use fs_err::File;
use std::path::Path;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::functions::format::{format_cargo_fmt, format_token_stream_prettyplease};
use crate::functions::label::{to_ident, to_stem, try_from_utf8_path};
use crate::generate_modules::{append, create_dir_all_for_file, generate_modules, overwrite};
use crate::get_relative_path::get_relative_path_anchor_stem_rs;
use crate::traits::cargo_info::CargoInfo;
use crate::traits::to_module_token_stream::ToModuleTokenStream;
use crate::types::config::CodeActionsConfig;
use crate::types::module_template::ModuleTemplate;
use crate::types::outcome::Outcome;
use anyhow::ensure;
use proc_macro2::TokenStream;

pub fn generate_module_file<FileContents, GetFileContents>(path: impl AsRef<Utf8Path>, get_file_contents: GetFileContents) -> Outcome<File>
where
    FileContents: AsRef<[u8]>,
    GetFileContents: FnOnce(&Utf8Path) -> Outcome<FileContents>,
{
    let path_ref = path.as_ref();
    let contents = get_file_contents(path_ref)?;
    create_module_file(path, contents)
}

pub fn get_module_file_from_label(label: &str, module_template: ModuleTemplate) -> Outcome<String> {
    let token_stream = module_template.to_module_token_stream(to_ident(label));
    let string = format_token_stream_prettyplease(token_stream)?;
    Ok(string)
}

pub fn create_module_file_from_anchor_label(anchor: &Utf8Path, label: &str, module_template: ModuleTemplate) -> Outcome<File> {
    let path = get_relative_path_anchor_stem_rs(anchor, to_stem(label))?;
    let manifest_path_buf = path.as_path().get_package_or_workspace_manifest()?;

    // Try to load config, use default if not found
    let config = CodeActionsConfig::load_from_anchor(anchor).unwrap_or_default();
    let token_stream = module_template.to_module_token_stream_with_config(to_ident(label), &config);

    create_module_file_from_stream(path, manifest_path_buf, token_stream)
}

pub fn append_to_module_file_from_path(path: &Utf8Path, module_template: ModuleTemplate) -> Outcome<File> {
    let manifest_path_buf = path.get_package_or_workspace_manifest()?;
    let label = try_from_utf8_path(path)?;

    // Try to load config, use default if not found
    let config = CodeActionsConfig::load_from_anchor(path).unwrap_or_default();
    let token_stream = module_template.to_module_token_stream_with_config(to_ident(&label), &config);

    append_to_module_file_from_stream(path, manifest_path_buf, token_stream)
}

pub fn create_module_file_from_stream(path: impl AsRef<Utf8Path>, manifest_path: impl AsRef<Path>, stream: TokenStream) -> Outcome<File> {
    let contents = format_token_stream_prettyplease(stream)?;
    let file = create_module_file(path.as_ref(), contents)?;
    format_cargo_fmt(manifest_path)?;
    Ok(file)
}

pub fn append_to_module_file_from_stream(path: impl AsRef<Utf8Path>, manifest_path: impl AsRef<Path>, stream: TokenStream) -> Outcome<File> {
    let contents = format_token_stream_prettyplease(stream)?;
    let file = append_to_module_file(path.as_ref(), contents)?;
    format_cargo_fmt(manifest_path)?;
    Ok(file)
}

pub fn create_module_file(path: impl AsRef<Utf8Path>, contents: impl AsRef<[u8]>) -> Outcome<File> {
    let path = path.as_ref();
    ensure!(!path.exists(), "File already exists: {}", path);
    create_dir_all_for_file(path)?;
    generate_modules(path)?;
    overwrite(path, contents)
}

// // TODO: The file has been changed on disk; maybe it's better not to return it
// pub fn create_and_format_module_file(path: impl AsRef<Utf8Path>, contents: impl AsRef<[u8]>) -> Outcome<File> {
//     let path = path.as_ref();
//     let file = create_module_file(path, contents)?;
//     let project_root = ProjectRoot::try_from_anchor(path)?;
//     format(path, project_root)?;
//     Ok(file)
// }

pub fn append_to_module_file(path: impl AsRef<Utf8Path>, contents: impl AsRef<[u8]>) -> Outcome<File> {
    let path = path.as_ref();
    create_dir_all_for_file(path)?;
    generate_modules(path)?;
    append(path, contents)
}

// // TODO: The file has been changed on disk; maybe it's better not to return it
// pub fn extend_and_format_module_file(path: impl AsRef<Utf8Path>, contents: impl AsRef<[u8]>) -> Outcome<File> {
//     let path = path.as_ref();
//     let file = extend_module_file(path, contents)?;
//     let project_root = ProjectRoot::try_from_anchor(path)?;
//     format(path, project_root)?;
//     Ok(file)
// }
