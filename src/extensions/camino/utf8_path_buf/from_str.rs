use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;

impl From<&str> for Utf8PathBuf {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
