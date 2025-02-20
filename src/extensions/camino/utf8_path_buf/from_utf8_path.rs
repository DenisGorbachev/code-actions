use camino::Utf8Path;

use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;

impl<'a> From<&'a Utf8Path> for Utf8PathBuf {
    fn from(value: &'a Utf8Path) -> Self {
        Self(value.to_path_buf())
    }
}
