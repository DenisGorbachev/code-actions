use derive_getters::{Dissolve, Getters};
use derive_new::new;
use toml_edit::{Item, Value};

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::traits::cargo_info::CargoInfo;
use crate::types::project_root::ProjectRoot;
use crate::types::toml_file::TomlFile;

#[derive(new, Getters, Dissolve, Default, Clone, Debug)]
pub struct PackageInfo {
    pub package_manifest: TomlFile,
    pub workspace_manifest: Option<TomlFile>,
}

impl PackageInfo {
    pub fn as_refs(&self) -> (&TomlFile, Option<&TomlFile>) {
        (&self.package_manifest, self.workspace_manifest.as_ref())
    }

    pub fn project_manifest(&self) -> &TomlFile {
        self.workspace_manifest
            .as_ref()
            .unwrap_or(&self.package_manifest)
    }

    pub fn project_root(&self) -> Option<ProjectRoot> {
        self.project_manifest()
            .path()
            .parent()
            .map(ProjectRoot::from)
    }
}

impl TryFrom<&Utf8Path> for PackageInfo {
    type Error = anyhow::Error;

    fn try_from(anchor: &Utf8Path) -> Result<Self, Self::Error> {
        let package_root_path = anchor.get_package_root()?;
        let package_manifest_path_buf = package_root_path.to_manifest();
        let package_manifest = TomlFile::try_from(package_manifest_path_buf)?;

        let workspace_root_path_buf_opt = if let Some(workspace) = package_manifest
            .get("package")
            .and_then(|package| package.get("workspace"))
        {
            match workspace {
                Item::Value(Value::String(value)) => Some(Utf8PathBuf::from(value.to_string().as_str())),
                _ => None,
            }
        } else if package_manifest.get("workspace").is_some() {
            None
        } else {
            // TODO: Traverse up to the current working directory
            package_manifest
                .path()
                .parent()
                .and_then(Utf8Path::parent)
                .and_then(CargoInfo::find_package_root)
                .map(Utf8Path::to_path_buf)
        };

        // Check if the package is excluded from the workspace
        let workspace_manifest = if let Some(workspace_root_path_buf) = workspace_root_path_buf_opt {
            let workspace_manifest_path_buf: Utf8PathBuf = workspace_root_path_buf.as_path().to_manifest();
            let workspace_manifest = TomlFile::try_from(workspace_manifest_path_buf)?;
            if let Some(Item::Table(workspace_table)) = workspace_manifest.get("workspace") {
                if let Some(Item::Value(Value::Array(exclude_array))) = workspace_table.get("exclude") {
                    if !exclude_array
                        .iter()
                        .any(|item| item.as_str().map(|it| workspace_root_path_buf.join(it)) == Some(package_root_path.to_path_buf()))
                    {
                        Some(workspace_manifest)
                    } else {
                        None
                    }
                } else {
                    Some(workspace_manifest)
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(PackageInfo::new(package_manifest, workspace_manifest))
    }
}

impl TryFrom<Utf8PathBuf> for PackageInfo {
    type Error = anyhow::Error;

    fn try_from(anchor: Utf8PathBuf) -> Result<Self, Self::Error> {
        TryFrom::try_from(anchor.as_path())
    }
}
