use std::path::Path;

use derive_more::Deref;
use tempfile::TempDir as TempfileTempDir;

#[derive(Deref, Debug)]
pub struct TempDir(pub TempfileTempDir);

impl TempDir {}

impl AsRef<Path> for TempDir {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}
