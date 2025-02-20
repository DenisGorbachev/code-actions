use standard_traits::Contains;
use std::path::Path;

pub trait IsInternal {
    fn is_internal(&self, manifest_root: impl AsRef<Path>, project_root: impl AsRef<Path>) -> bool;
}

impl IsInternal for String {
    fn is_internal(&self, manifest_root: impl AsRef<Path>, project_root: impl AsRef<Path>) -> bool {
        let relative_path = manifest_root.as_ref().join(self);
        let canonical_path = relative_path
            .canonicalize()
            .unwrap_or_else(|e| panic!("it should be possible to make the path '{}' canonical (encountered {})", relative_path.display(), e));
        project_root.as_ref().contains(canonical_path.as_path())
    }
}
