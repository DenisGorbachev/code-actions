use std::path::Path;

use crate::extensions::camino::utf8_path_old::Utf8PathOld;

impl AsRef<Path> for Utf8PathOld {
    fn as_ref(&self) -> &Path {
        AsRef::as_ref(&self.0)
    }
}
