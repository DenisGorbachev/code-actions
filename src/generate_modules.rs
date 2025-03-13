use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;

use anyhow::Context;
use fs_err::{create_dir_all, File, OpenOptions};

use crate::types::outcome::Outcome;

use crate::constants::SRC_DIR_NAME;
use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::std::fs::file_ext::FileExt;
use crate::extensions::std::path::file_stem::FileStem;
use crate::primary_module::get_primary_module_path;

pub fn get_str_from_option_os_str<'a>(input: Option<&'a OsStr>, name: &str) -> &'a str {
    let os_str = input.unwrap_or_else(|| panic!("{} should be present", name));
    os_str
        .to_str()
        .unwrap_or_else(|| panic!("{} should be convertible to str", name))
}

pub fn generate_modules(path: &Utf8Path) -> Outcome {
    // dbg!(&path);
    let root = path.get_src_root()?;
    let src = root.join(SRC_DIR_NAME);
    let mut child = path;
    let parents = path.parents_up_to(src.as_path());

    for parent in parents {
        // dbg!(&parent);
        let mut module_file_path = parent.to_path_buf();
        module_file_path.set_extension("rs");
        // dbg!(&module_file_path);
        let mut module_file = open_file_for_appending(&module_file_path)?;
        let module_declarations = get_module_declarations(&mut module_file, child)?;

        module_file
            .append_if_not_contains(&module_declarations)
            .with_context(|| format!("Could not append module declaration to file: '{}'", &module_file_path))?;

        child = parent;
    }

    let primary_module_file_path = get_primary_module_path(src.clone())?;
    let mut primary_module_file = open_file_for_appending(&primary_module_file_path)?;
    let module_declarations = get_module_declarations(&mut primary_module_file, child)?;
    primary_module_file
        .append_if_not_contains(&module_declarations)
        .with_context(|| format!("Could not append module declaration to file: '{}'", &primary_module_file_path.display()))?;

    Ok(())
}

pub fn create_dir_all_for_file(path: &Utf8Path) -> Outcome {
    let parent = path
        .parent()
        .with_context(|| format!("Could not get parent from path: {}", &path))?;
    create_dir_all(parent)?;
    Ok(())
}

pub fn open_file_for_appending(path: impl AsRef<Path>) -> Outcome<File> {
    let path = path.as_ref();
    OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(path)
        .with_context(|| format!("Could not open file for appending: '{}'", path.display()))
}

pub fn open_file_for_overwriting(path: impl AsRef<Path>) -> Outcome<File> {
    let path = path.as_ref();
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .with_context(|| format!("Could not open file for overwriting: '{}'", path.display()))
}

pub fn append(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Outcome<File> {
    let mut file = open_file_for_appending(path)?;
    file.write_all(contents.as_ref())?;
    Ok(file)
}

pub fn overwrite(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Outcome<File> {
    let mut file = open_file_for_overwriting(path)?;
    file.write_all(contents.as_ref())?;
    Ok(file)
}

pub fn get_module_declarations(file: &mut File, path: &Utf8Path) -> Outcome<Vec<String>> {
    if file.contains("pub mod")? {
        get_pub_mod_declarations(path)
    } else {
        get_mod_pub_use_declarations(path)
    }
}

// TODO: Don't generate `pub use` declarations for modules that contain only macros
pub fn get_mod_pub_use_declarations(path: &Utf8Path) -> Outcome<Vec<String>> {
    let file_stem = FileStem::try_from(path)?;
    Ok(vec![
        format!("mod {};", &file_stem),
        format!("pub use {}::*;", &file_stem),
    ])
}

pub fn get_pub_mod_declarations(path: &Utf8Path) -> Outcome<Vec<String>> {
    let file_stem = FileStem::try_from(path)?;
    Ok(vec![format!("pub mod {};", &file_stem)])
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::types::outcome::Outcome;

    use crate::extensions::camino::utf8_path::Utf8Path;
    use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
    use crate::extensions::std::path;
    use crate::generate_modules::{create_dir_all_for_file, generate_modules};
    use crate::test_helpers::{get_src_path, get_temp_bin_root};

    #[test]
    fn test_empty_directory() -> io::Result<()> {
        // call fn create_modules with path = tmp + "some/deep/struct.rs"
        // assert that /some.rs exists
        // assert that /some/deep.rs exists
        // assert that /some/deep/struct.rs exists
        // assert that /some.rs contains a "pub mod deep;" line
        // assert that /some/deep.rs contains a "pub mod struct;" line
        // assert that /some/deep/struct.rs contains "struct Struct {}"
        // assert that /some/deep/struct.rs contains "#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]"
        // call tmp.close()
        Ok(())
    }

    #[test]
    fn test_path_outside_of_package() -> io::Result<()> {
        // generate a path for which is_inside_package returns false
        // call fn create_modules with this path
        // assert that create_modules returns an error
        Ok(())
    }

    /// It must not generate a module file in the workspace, only in the package
    /// It must attach the top-level module to either lib.rs or main.rs (or return an error if neither exists)
    #[test]
    fn test_cargo_workspace() {}

    /// It must not write any files if there is an error (e.g. neither lib.rs nor main.rs exist)
    #[test]
    fn test_no_modifications_on_error() {}

    /// In particular, must not add duplicate `pub mod $name;` lines
    #[test]
    fn must_not_add_duplicate_lines() -> Outcome {
        let root = get_temp_bin_root()?;
        let root_path: &Utf8Path = (&root).try_into()?;
        let path_buf: Utf8PathBuf = get_src_path(&root).join("some/deep/struct.rs").try_into()?;
        let path = path_buf.as_ref();
        create_dir_all_for_file(path)?;
        generate_modules(path)?;
        generate_modules(path)?;
        assert_eq!(path::file_with_duplicate_lines(root_path), None);
        Ok(())
    }
}
