use std::path::PathBuf;

use derive_more::Error;
use derive_new::new;
use fmt_derive::Display;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::tempfile::temp_dir::TempDir;

#[derive(new, Error, Display, Debug)]
pub struct TryFromTempDirError {
    pub path_buf: PathBuf,
}

impl<'a> TryFrom<&'a TempDir> for &'a Utf8Path {
    type Error = TryFromTempDirError;

    fn try_from(value: &'a TempDir) -> Result<Self, Self::Error> {
        camino::Utf8Path::from_path(value.path())
            .map(Utf8Path::new)
            .ok_or_else(|| TryFromTempDirError::new(value.as_ref().to_path_buf()))
    }
}
