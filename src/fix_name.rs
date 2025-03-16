use crate::constants::SRC_DIR_NAME;
use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::functions::parent_candidates::parent_candidates;
use crate::traits::rename_module::RenameModule;
use crate::types::outcome::Outcome;
use anyhow::Context;
use heck::ToSnakeCase;
use prettyplease::unparse;
use proc_macro2::Ident;
use syn_more::{maybe_ident_for_item, parse_main_item_from_path, SynFrom};

/// This function simply renames the file and the `mod` declaration in the parent module. It doesn't rename all references.
// TODO: Ensure that every reference is renamed, too (can only do that with rust-analyzer)
pub fn fix_name(path: &Utf8Path) -> Outcome {
    let root = path.get_src_root()?;
    let src = root.join(SRC_DIR_NAME);
    let ident = main_ident(path)?;
    let module_name_new = ident.to_string().to_snake_case();
    let file_name = format!("{}.rs", module_name_new);
    let path_new = Utf8PathBuf::from(path.with_file_name(file_name));
    if !path_new.as_path().eq(path) {
        let module_name_old = path
            .file_stem()
            .expect("module path should have a file stem");
        let parent = parent_candidates(path, src.as_path()).next();
        fs_err::rename(path, path_new)?;
        if let Some(parent) = parent {
            let mut file = syn::File::syn_from(parent.as_path().as_std_path())?;
            file.rename_module(module_name_old, &module_name_new)?;
            fs_err::write(parent.as_path(), unparse(&file))?;
        }
    }
    Ok(())
}

pub fn main_ident(anchor: &Utf8Path) -> Outcome<Ident> {
    let item = parse_main_item_from_path(anchor)?.with_context(|| format!("Main item not found in \"{}\"", anchor))?;
    let item_ident = maybe_ident_for_item(item).context("Expected the main item to have an ident")?;
    Ok(item_ident)
}
