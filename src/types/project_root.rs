use crate::constants::CARGO_TOML_FILE_NAME;
use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::traits::cargo_info::CargoInfo;
use not_found_error::NotFoundError;
use std::path::PathBuf;
use subtype::subtype_path_buf;

subtype_path_buf!(
    pub struct ProjectRoot(PathBuf);
);

impl From<&Utf8Path> for ProjectRoot {
    fn from(value: &Utf8Path) -> Self {
        Self::from(value.to_path_buf())
    }
}

impl From<Utf8PathBuf> for ProjectRoot {
    fn from(value: Utf8PathBuf) -> Self {
        Self::from(value.0.into_std_path_buf())
    }
}

impl ProjectRoot {
    pub fn try_from_anchor(anchor: &Utf8Path) -> Result<Self, NotFoundError<Self>> {
        anchor.get_project_root().map(Self::from)
    }

    pub fn manifest_path_buf(&self) -> PathBuf {
        self.join(CARGO_TOML_FILE_NAME)
    }
}
