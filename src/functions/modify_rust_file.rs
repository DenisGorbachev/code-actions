use crate::extensions::std::fs::modify_file_contents;
use crate::functions::format::format_cargo_fmt;
use crate::types::outcome::Outcome;
use prettyplease::unparse;
use std::path::Path;
use syn::parse_file;

pub fn modify_rust_file<Modify>(path: impl AsRef<Path>, modify: Modify) -> Outcome
where
    Modify: FnOnce(syn::File) -> Outcome<syn::File>,
{
    // TODO: This function does not preserve regular comments starting with "//"
    modify_file_contents(path, |string| -> Outcome<String> {
        let file = parse_file(&string)?;
        let file = modify(file)?;
        Ok(unparse(&file))
    })
}

pub fn modify_and_format_rust_file<Modify>(path: impl AsRef<Path>, manifest_path: impl AsRef<Path>, modify: Modify) -> Outcome
where
    Modify: FnOnce(syn::File) -> Outcome<syn::File>,
{
    // TODO: This function does not preserve regular comments starting with "//" (use a parser from rust-analyzer instead of syn?)
    // TODO: Return an error if the file contains "//"
    modify_rust_file(path.as_ref(), modify)?;
    format_cargo_fmt(manifest_path)?;
    Ok(())
}
