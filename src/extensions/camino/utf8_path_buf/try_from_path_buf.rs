use std::path::PathBuf;

use derive_more::Error;
use fmt_derive::Display;

use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;

#[derive(Error, Display, Debug)]
pub struct TryFromPathBufError;

impl TryFrom<PathBuf> for Utf8PathBuf {
    type Error = TryFromPathBufError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        camino::Utf8PathBuf::from_path_buf(value)
            .map(Utf8PathBuf)
            .map_err(|_| TryFromPathBufError)
    }
}
