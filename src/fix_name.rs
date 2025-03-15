use crate::types::outcome::Outcome;
use anyhow::Context;
use fs_err::read_to_string;
use heck::ToSnakeCase;
use proc_macro2::Ident;
use stub_macro::stub;
use syn_more::{maybe_ident_for_item, parse_main_item_from_path};

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::functions::parent_candidates::parent_candidates;

/// This function simply renames the file and the `mod` declaration in the parent module. It doesn't rename all references.
// TODO: Ensure that every reference is renamed, too (can only do that with rust-analyzer)
pub fn fix_name(path: &Utf8Path) -> Outcome {
    let src = stub!();
    let ident = main_ident(path)?;
    let module_name_new = ident.to_string().to_snake_case();
    let file_name = format!("{}.rs", module_name_new);
    let path_new = Utf8PathBuf::from(path.with_file_name(file_name));
    if !path_new.as_path().eq(path) {
        let module_name_old = path
            .file_stem()
            .expect("module path should have a file stem");
        let parent = parent_candidates(path, src).next();
        fs_err::rename(path, path_new)?;
        if let Some(parent) = parent {
            rename_declarations(parent.as_path(), module_name_old, &module_name_new)?
        }
    }
    Ok(())
}

pub fn rename_declarations(parent: &Utf8Path, child_module_name_old: &str, child_module_name_new: &str) -> Outcome {
    let contents = read_to_string(parent)?;
    let contents = contents.replace(&format!("mod {}", child_module_name_old), &format!("mod {}", child_module_name_new));
    let contents = contents.replace(&format!("use {}", child_module_name_old), &format!("use {}", child_module_name_new));
    fs_err::write(parent, contents)?;
    Ok(())
}

pub fn main_ident(anchor: &Utf8Path) -> Outcome<Ident> {
    let item = parse_main_item_from_path(anchor)?.with_context(|| format!("Main item not found in \"{}\"", anchor))?;
    let item_ident = maybe_ident_for_item(item).context("Expected the main item to have an ident")?;
    Ok(item_ident)
}
