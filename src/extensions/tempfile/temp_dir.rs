use std::path::Path;

use derive_more::Deref;

#[derive(Deref, Debug)]
pub struct TempDir(pub tempfile::TempDir);

impl TempDir {}

impl AsRef<Path> for TempDir {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}
