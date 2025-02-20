use crate::traits::cargo_info::CargoInfo;
use crate::types::outcome::Outcome;
use duct::cmd;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use std::io;
use std::path::Path;
use std::process::Output;

/// Formats the file at `path` with `rustfmt`
#[deprecated(note = "`format_cargo_fmt` is better because it invokes rustfmt with a Rust edition specified in Cargo.toml")]
pub fn format_rustfmt(path: impl AsRef<Path>, work_dir: impl AsRef<Path>) -> io::Result<Output> {
    cmd!("rustfmt", path.as_ref()).dir(work_dir.as_ref()).run()
}

/// Formats the package with `cargo fmt`
pub fn format_cargo_fmt(manifest_path: impl AsRef<Path>) -> io::Result<Output> {
    cmd!("cargo", "fmt", "--all", "--manifest-path", manifest_path.as_ref()).run()
}

pub fn format_cargo_fmt_by_path(path: impl AsRef<Path>) -> Outcome<Output> {
    let manifest_path = path.as_ref().get_package_or_workspace_manifest()?;
    let output = format_cargo_fmt(manifest_path)?;
    Ok(output)
}

pub fn format_token_stream_rustfmt(tokens: TokenStream) -> io::Result<Output> {
    cmd!("rustfmt").stdin_bytes(tokens.to_string()).run()
}

pub fn format_token_stream_prettyplease(tokens: TokenStream) -> syn::Result<String> {
    // NOTE: using parse_file instead of parse2 because tokens may contain multiple Items
    let file = syn::parse_file(&tokens.to_string())?;
    let string = unparse(&file).replace("#[newline]", "");
    Ok(string)
}
