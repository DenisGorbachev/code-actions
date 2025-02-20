use derive_more::Display;
use derive_more::Error;
use derive_new::new;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::extensions::std::path::file_stem::FileStem;

#[derive(new, Error, Display, Debug)]
pub struct TryFromUtf8PathError {
    pub path_buf: Utf8PathBuf,
}

impl<'a> TryFrom<&'a Utf8Path> for FileStem<'a> {
    type Error = TryFromUtf8PathError;

    fn try_from(value: &'a Utf8Path) -> Result<Self, Self::Error> {
        value
            .file_stem()
            .map(FileStem)
            .ok_or_else(|| TryFromUtf8PathError::new(value.to_path_buf()))
    }
}
