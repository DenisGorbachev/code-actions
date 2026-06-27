use camino::Utf8PathBuf as CaminoUtf8PathBuf;

use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;

impl From<CaminoUtf8PathBuf> for Utf8PathBuf {
    fn from(value: CaminoUtf8PathBuf) -> Self {
        Self(value)
    }
}
