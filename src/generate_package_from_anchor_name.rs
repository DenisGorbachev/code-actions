use duct::cmd;

use crate::types::outcome::Outcome;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::std::fs::truncate;
use crate::generate_module::generate_module_with_dir_from_parent_dir_and_stem;
use crate::traits::cargo_info::CargoInfo;

pub fn generate_package_from_anchor_name(anchor: &Utf8Path, name: &str, args: &[&str]) -> Outcome {
    let workspace_root = anchor.get_workspace_root()?;
    let new_package_root = workspace_root.join(name);
    let new_package_root_src = new_package_root.join("src");
    let new_package_root_path = new_package_root.as_path();
    let default_modules = &["types", "functions"];
    cmd(
        "cargo",
        std::iter::once("new")
            .chain(std::iter::once(name))
            .chain(args.iter().cloned()),
    )
    .dir(workspace_root)
    .run()?;
    if args.contains(&"--lib") {
        let lib_rs = new_package_root_src.join("lib.rs");
        truncate(lib_rs)?;
    }
    default_modules
        .iter()
        .map(|module| generate_module_with_dir_from_parent_dir_and_stem(new_package_root_path, *module))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(())
}
