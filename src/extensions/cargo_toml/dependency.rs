use crate::traits::is_internal::IsInternal;
use crate::traits::is_internal_opt::IsInternalOpt;
use cargo_toml::Dependency;
use std::path::Path;
use tracing::instrument;

impl IsInternalOpt for Dependency {
    #[instrument(fields(manifest_root = %manifest_root.as_ref().display(), project_root = %project_root.as_ref().display()))]
    fn is_internal_opt(&self, manifest_root: impl AsRef<Path>, project_root: impl AsRef<Path>) -> Option<bool> {
        let detail = self.detail()?;
        let path = detail.path.as_ref()?;
        dbg!(detail);
        dbg!(path);
        dbg!(manifest_root.as_ref());
        dbg!(project_root.as_ref());
        let is_internal = path.is_internal(manifest_root, project_root);
        dbg!(is_internal);
        if path.contains("syn") {
            panic!();
        }
        Some(is_internal)
    }
}

impl IsInternal for Dependency {
    fn is_internal(&self, manifest_root: impl AsRef<Path>, project_root: impl AsRef<Path>) -> bool {
        self.is_internal_opt(manifest_root, project_root)
            .unwrap_or(true)
    }
}
