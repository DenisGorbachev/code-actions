use camino::Utf8PathBuf;
use derive_more::{Display, Error};
use derive_new::new;

use crate::extensions::std::path::file_stem::FileStem;

#[derive(new, Error, Display, Debug)]
pub struct TryFromUtf8PathBufError {
    pub path_buf: Utf8PathBuf,
}

impl<'a> TryFrom<&'a Utf8PathBuf> for FileStem<'a> {
    type Error = TryFromUtf8PathBufError;

    fn try_from(value: &'a Utf8PathBuf) -> Result<Self, Self::Error> {
        value
            .file_stem()
            .map(FileStem)
            .ok_or_else(|| TryFromUtf8PathBufError::new(value.clone()))
    }
}
