use std::path::Path;

pub trait IsInternalOpt {
    fn is_internal_opt(&self, manifest_root: impl AsRef<Path>, project_root: impl AsRef<Path>) -> Option<bool>;
}
