use crate::constants::CARGO_TOML_FILE_NAME;
use crate::extensions::camino::utf8_path::Utf8Path;
use crate::traits::find_dir_containing_filename::FindDirContainingFilename;
use crate::types::project_root::ProjectRoot;
use not_found_error::{NotFoundError, OkOrNotFound};
use standard_traits::Join;
use std::path::Path;

pub trait CargoInfo: ToOwned
where
    for<'a, 'b> &'a Self: Join<&'b Self, Output = Self::Owned> + Join<&'b str, Output = Self::Owned>,
    str: AsRef<Self>,
{
    fn find_package_root(&self) -> Option<&Self>;

    fn find_workspace_root(&self) -> Option<&Self>;

    fn find_workspace_manifest(&self) -> Option<Self::Owned> {
        self.find_workspace_root()
            .map(|root| root.join(CARGO_TOML_FILE_NAME))
    }

    fn find_package_or_workspace_root(&self) -> Option<&Self> {
        self.find_package_root()
            .or_else(|| self.find_workspace_root())
    }

    fn find_workspace_or_package_root(&self) -> Option<&Self> {
        self.find_workspace_root()
            .or_else(|| self.find_package_root())
    }

    fn find_package_or_workspace_manifest(&self) -> Option<Self::Owned> {
        self.find_package_or_workspace_root().map(Self::to_manifest)
    }

    fn find_workspace_or_package_manifest(&self) -> Option<Self::Owned> {
        self.find_workspace_or_package_root().map(Self::to_manifest)
    }

    fn get_package_manifest(&self) -> Result<Self::Owned, NotFoundError<PackageRoot>> {
        self.get_package_root().map(Self::to_manifest)
    }

    fn get_workspace_manifest(&self) -> Result<Self::Owned, NotFoundError<WorkspaceRoot>>
    where
        str: AsRef<Self>,
    {
        self.get_workspace_root().map(Self::to_manifest)
    }

    fn get_package_root(&self) -> Result<&Self, NotFoundError<PackageRoot>> {
        self.find_package_root().ok_or_not_found()
    }

    fn get_workspace_root(&self) -> Result<&Self, NotFoundError<WorkspaceRoot>> {
        self.find_workspace_root().ok_or_not_found()
    }

    // TODO: migrate the codebase to newtypes
    fn get_project_root(&self) -> Result<&Self, NotFoundError<ProjectRoot>> {
        self.find_workspace_or_package_root().ok_or_not_found()
    }

    fn get_package_or_workspace_manifest(&self) -> Result<Self::Owned, NotFoundError<ManifestFile>> {
        self.find_package_or_workspace_manifest().ok_or_not_found()
    }

    fn get_workspace_or_package_manifest(&self) -> Result<Self::Owned, NotFoundError<ManifestFile>> {
        self.find_workspace_or_package_manifest().ok_or_not_found()
    }

    fn to_manifest(&self) -> Self::Owned
    where
        str: AsRef<Self>,
    {
        self.join(CARGO_TOML_FILE_NAME)
    }
}

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct PackageRoot;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct WorkspaceRoot;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct ManifestFile;

impl CargoInfo for Path {
    fn find_package_root(&self) -> Option<&Self> {
        self.find_dir_containing_filename(CARGO_TOML_FILE_NAME)
    }

    fn find_workspace_root(&self) -> Option<&Self> {
        let package_root = self.find_dir_containing_filename(CARGO_TOML_FILE_NAME)?;
        let package_root_parent = package_root.parent()?;
        package_root_parent.find_dir_containing_filename(CARGO_TOML_FILE_NAME)
    }
}

impl CargoInfo for Utf8Path {
    fn find_package_root(&self) -> Option<&Self> {
        self.find_dir_containing_filename(CARGO_TOML_FILE_NAME)
    }

    fn find_workspace_root(&self) -> Option<&Self> {
        let package_root = self.find_dir_containing_filename(CARGO_TOML_FILE_NAME)?;
        let package_root_parent = package_root.parent()?;
        package_root_parent.find_dir_containing_filename(CARGO_TOML_FILE_NAME)
    }
}
