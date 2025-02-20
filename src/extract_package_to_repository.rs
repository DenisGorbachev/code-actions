use crate::types::outcome::Outcome;

pub fn extract_package_to_repository() -> Outcome {
    let _config_files = ["rustfmt.toml", "clippy.toml", "bacon.toml"];
    let _lefthook = "lefthook.yml"; // TODO: For binary packages, the lefthook.yml is different (no `cargo test --doc`)
    let _gitignore = ".gitignore";
    Ok(())
}
