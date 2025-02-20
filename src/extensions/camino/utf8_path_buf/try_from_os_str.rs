use std::ffi::OsStr;

use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;

pub struct TryFromOsStrError;

impl TryFrom<&OsStr> for Utf8PathBuf {
    type Error = std::str::Utf8Error;

    fn try_from(value: &OsStr) -> Result<Self, Self::Error> {
        let s: &str = value.try_into()?;
        Ok(s.into())
    }
}
