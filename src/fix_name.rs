use anyhow::Context;
use heck::ToSnakeCase;
use proc_macro2::Ident;

use crate::types::outcome::Outcome;
use syn_more::{maybe_ident_for_item, parse_main_item_from_path};

use crate::extensions::camino::utf8_path::Utf8Path;

pub fn fix_name(path: &Utf8Path) -> Outcome {
    let ident = main_ident(path)?;
    let file_name = ident.to_string().to_snake_case();
    let path_new = path.with_file_name(file_name);
    // TODO: Ensure that every reference is renamed, too
    // TODO: Can only do that with rust-analyzer
    fs_err::rename(path, path_new)?;
    Ok(())
}

pub fn main_ident(anchor: &Utf8Path) -> Outcome<Ident> {
    let item = parse_main_item_from_path(anchor)?.with_context(|| format!("Main item not found in \"{}\"", anchor))?;
    let item_ident = maybe_ident_for_item(item).context("Expected the main item to have an ident")?;
    Ok(item_ident)
}
