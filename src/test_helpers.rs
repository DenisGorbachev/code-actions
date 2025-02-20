use crate::constants::{CARGO_TOML_FILE_NAME, LIB_FILE_NAME, MAIN_FILE_NAME, SRC_DIR_NAME};
use crate::extensions::std::fs::{create_file_all, CreateFileAllError};
use crate::extensions::tempfile::temp_dir::TempDir;
use crate::types::outcome::Outcome;
use anyhow::Context;
use cargo_toml::{Inheritable, Resolver};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

type Manifest = cargo_toml::Manifest<()>;
type Package = cargo_toml::Package<()>;
type Workspace = cargo_toml::Workspace<()>;

const WORKSPACE: &str = "foo_workspace";
const PACKAGE: &str = "bar_package";
const CARGO: &str = CARGO_TOML_FILE_NAME;
const MAIN: &str = MAIN_FILE_NAME;
const LIB: &str = LIB_FILE_NAME;

pub fn get_workspace_path(root: &impl AsRef<Path>) -> PathBuf {
    root.as_ref().join(WORKSPACE)
}

pub fn get_package_path(root: &impl AsRef<Path>) -> PathBuf {
    get_workspace_path(root).join(PACKAGE)
}

pub fn get_src_path(root: &impl AsRef<Path>) -> PathBuf {
    get_package_path(root).join(SRC_DIR_NAME)
}

// fn get_workspace_cargo_toml_path(root: &impl AsRef<Path>) -> PathBuf {
//     get_workspace_path(root).join(CARGO)
// }
//
// fn get_package_cargo_toml_path(root: &impl AsRef<Path>) -> PathBuf {
//     get_package_path(root).join(CARGO)
// }

pub fn get_main_rs_path(root: &impl AsRef<Path>) -> PathBuf {
    get_src_path(root).join(MAIN)
}

pub fn get_lib_rs_path(root: &impl AsRef<Path>) -> PathBuf {
    get_src_path(root).join(LIB)
}

fn create_workspace_cargo_toml(path: impl AsRef<Path>, members: Vec<String>) -> Outcome<File> {
    let workspace = Workspace {
        members,
        resolver: Some(Resolver::V2),
        ..Default::default()
    };
    let manifest = Manifest {
        workspace: Some(workspace),
        ..Default::default()
    };
    let manifest_string = toml::ser::to_string_pretty(&manifest)?;
    let mut file = create_file_all(path.as_ref().join(CARGO))?;
    file.write_all(manifest_string.as_bytes())?;
    Ok(file)
}

fn create_package_cargo_toml(path: impl AsRef<Path>, name: String) -> Outcome<File> {
    let mut package = Package::default();
    package.name = name;
    package.version = Inheritable::Set("0.1.0".into());
    let manifest = Manifest {
        package: Some(package),
        ..Default::default()
    };
    let manifest_string = toml::ser::to_string_pretty(&manifest)?;
    let mut file = create_file_all(path.as_ref().join(CARGO))?;
    file.write_all(manifest_string.as_bytes())?;
    Ok(file)
}

fn create_main_rs(root: &impl AsRef<Path>) -> anyhow::Result<File, CreateFileAllError> {
    create_file_all(get_main_rs_path(root))
}

pub fn create_lib_rs(root: &impl AsRef<Path>) -> anyhow::Result<File, CreateFileAllError> {
    create_file_all(get_lib_rs_path(root))
}

pub fn get_temp_root() -> Outcome<TempDir> {
    let root = tempdir().context("Could not get `temp_dir`")?;
    create_workspace_cargo_toml(get_workspace_path(&root), vec![PACKAGE.to_string()])?;
    create_package_cargo_toml(get_package_path(&root), PACKAGE.to_string())?;
    Ok(TempDir(root))
}

pub fn get_temp_bin_root() -> Outcome<TempDir> {
    let temp_dir = get_temp_root()?;
    create_main_rs(&temp_dir)?;
    Ok(temp_dir)
}

pub fn get_temp_lib_root() -> Outcome<TempDir> {
    let temp_dir = get_temp_root()?;
    create_lib_rs(&temp_dir)?;
    Ok(temp_dir)
}
