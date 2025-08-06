use crate::extensions::camino::utf8_path::Utf8Path;
use crate::types::outcome::Outcome;
use fs_err::read_to_string;

pub fn rename_declarations_path(parent: &Utf8Path, child_module_name_old: &str, child_module_name_new: &str) -> Outcome {
    let contents = read_to_string(parent)?;
    let contents = contents.replace(&format!("mod {child_module_name_old}"), &format!("mod {child_module_name_new}"));
    let contents = contents.replace(&format!("use {child_module_name_old}"), &format!("use {child_module_name_new}"));
    fs_err::write(parent, contents)?;
    Ok(())
}
