use crate::traits::is_internal::IsInternal;
use derive_builder::Builder;
use derive_getters::{Dissolve, Getters};
use derive_new::new;
use std::cmp::Ordering;
use std::path::Path;
use toml_edit::{Formatted, InlineTable, Key, Value};

#[derive(new, Getters, Dissolve, Builder, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug, serde::Serialize, serde::Deserialize)]
#[builder(default, setter(into, strip_option), derive(Debug))]
pub struct Dependency {
    version: Option<String>,
    path: Option<String>,
    workspace: Option<bool>,
    optional: Option<bool>,
}

impl Dependency {}

impl DependencyBuilder {
    pub fn optional_maybe(&mut self, optional: bool) -> &mut Self {
        if optional {
            self.optional = Some(Some(true))
        } else {
            self.optional = Some(None)
        }
        self
    }
}

impl From<Dependency> for InlineTable {
    fn from(value: Dependency) -> Self {
        let Dependency {
            version,
            path,
            workspace,
            optional,
        } = value;
        let mut spec = InlineTable::new();
        if let Some(version) = version {
            spec.insert("version", Value::String(Formatted::new(version)));
        }
        if let Some(optional) = optional {
            spec.insert("optional", Value::Boolean(Formatted::new(optional)));
        }
        if let Some(workspace) = workspace {
            spec.insert("workspace", Value::Boolean(Formatted::new(workspace)));
        }
        if let Some(path) = path {
            spec.insert("path", Value::String(Formatted::new(path)));
        }
        spec.sort_values_by(workspace_first);
        spec
    }
}

pub fn workspace_first(a_key: &Key, _a_value: &Value, _b_key: &Key, _b_value: &Value) -> Ordering {
    if a_key == "workspace" {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

impl IsInternal for Dependency {
    fn is_internal(&self, manifest_root: impl AsRef<Path>, project_root: impl AsRef<Path>) -> bool {
        match &self.path {
            None => true,
            Some(path) => path.is_internal(manifest_root, project_root),
        }
    }
}
