use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;

impl From<camino::Utf8PathBuf> for Utf8PathBuf {
    fn from(value: camino::Utf8PathBuf) -> Self {
        Self(value)
    }
}
