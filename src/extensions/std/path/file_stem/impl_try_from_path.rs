use crate::extensions::std::path::file_stem::FileStem;
use derive_more::Error;
use derive_new::new;
use fmt_derive::Display;
use std::path::{Path, PathBuf};

#[derive(new, Error, Display, Debug)]
pub struct TryFromPathError {
    pub path_buf: PathBuf,
}

impl<'a> TryFrom<&'a Path> for FileStem<'a> {
    type Error = TryFromPathError;

    fn try_from(path: &'a Path) -> Result<Self, Self::Error> {
        let os_str = path.file_stem().ok_or_else(|| TryFromPathError {
            path_buf: path.to_path_buf(),
        })?;
        let str = os_str.to_str().ok_or_else(|| TryFromPathError {
            path_buf: path.to_path_buf(),
        })?;
        Ok(Self(str))
    }
}
