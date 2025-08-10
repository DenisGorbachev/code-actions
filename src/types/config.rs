use derive_more::Error;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use fmt_derive::Display;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct CodeActionsConfig {
    pub extra_derives: FxHashMap<String, Vec<String>>,
    pub extra_use_statements: FxHashMap<String, Vec<String>>,
}

#[derive(Error, Display, Debug)]
pub enum LoadConfigError {
    FigmentError(Box<figment::Error>),
    IoError(std::io::Error),
}

impl From<std::io::Error> for LoadConfigError {
    fn from(error: std::io::Error) -> Self {
        LoadConfigError::IoError(error)
    }
}

impl From<figment::Error> for LoadConfigError {
    fn from(error: figment::Error) -> Self {
        LoadConfigError::FigmentError(Box::new(error))
    }
}

impl CodeActionsConfig {
    pub fn load_from_anchor<P: AsRef<Path>>(anchor_path: P) -> Result<Self, LoadConfigError> {
        let mut figment = Figment::new();

        // Walk up from anchor to workspace root, collecting config files
        let anchor_path = anchor_path.as_ref();
        let mut current_path = if anchor_path.is_file() { anchor_path.parent().unwrap_or(anchor_path) } else { anchor_path };

        let mut config_paths = Vec::new();

        // Collect all config files from anchor to workspace root
        loop {
            let config_file = current_path.join("code-actions.toml");
            if config_file.exists() {
                config_paths.push(config_file);
            }

            // Check if we've reached workspace root (contains Cargo.toml)
            let cargo_toml = current_path.join("Cargo.toml");
            if cargo_toml.exists() {
                break;
            }

            // Move up to parent directory
            if let Some(parent) = current_path.parent() {
                current_path = parent;
            } else {
                break;
            }
        }

        // Reverse to apply configs from workspace root down to anchor
        config_paths.reverse();

        // Load configs in order (workspace root first, anchor last)
        for config_path in config_paths {
            figment = figment.adjoin(Toml::file(&config_path));
        }

        // Merge environment variables with prefix CODE_ACTIONS_
        figment = figment.adjoin(Env::prefixed("CODE_ACTIONS_"));

        Ok(figment.extract()?)
    }

    pub fn get_extra_derives_for_name(&self, type_name: &str) -> Vec<String> {
        let mut derives = Vec::new();

        // Match patterns in order of specificity (more specific patterns override less specific)
        for (pattern, pattern_derives) in &self.extra_derives {
            if Self::matches_pattern(type_name, pattern) {
                derives.extend(pattern_derives.clone());
            }
        }

        // Remove duplicates while preserving order
        let mut seen = std::collections::HashSet::new();
        derives.retain(|derive| seen.insert(derive.clone()));

        derives
    }

    pub fn get_extra_use_statements_for_name(&self, type_name: &str) -> Vec<String> {
        let mut use_statements = Vec::new();

        // Match patterns in order of specificity (more specific patterns override less specific)
        for (pattern, pattern_uses) in &self.extra_use_statements {
            if Self::matches_pattern(type_name, pattern) {
                use_statements.extend(pattern_uses.clone());
            }
        }

        // Remove duplicates while preserving order
        let mut seen = std::collections::HashSet::new();
        use_statements.retain(|use_stmt| seen.insert(use_stmt.clone()));

        use_statements
    }

    fn matches_pattern(type_name: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        if pattern.starts_with('*') && pattern.ends_with('*') {
            let middle = &pattern[1..pattern.len() - 1];
            return type_name.contains(middle);
        }

        if let Some(suffix) = pattern.strip_prefix('*') {
            return type_name.ends_with(suffix);
        }

        if let Some(prefix) = pattern.strip_suffix('*') {
            return type_name.starts_with(prefix);
        }

        type_name == pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_load_config_with_extra_derives() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create workspace root with Cargo.toml
        let workspace_root = temp_path.join("workspace");
        fs::create_dir_all(&workspace_root).unwrap();
        fs::write(workspace_root.join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();

        // Create workspace config
        let workspace_config = r#"
[extra_derives]
"*Error" = ["Clone", "Debug"]
"*" = ["Debug"]
        "#;
        fs::write(workspace_root.join("code-actions.toml"), workspace_config).unwrap();

        // Create nested directory with its own config
        let nested_dir = workspace_root.join("src").join("nested");
        fs::create_dir_all(&nested_dir).unwrap();

        let nested_config = r#"
[extra_derives]
"*Error" = ["Clone", "Debug", "PartialEq"]
"User*" = ["Serialize", "Deserialize"]

[extra_use_statements]
"*Error" = ["serde::{Serialize, Deserialize}"]
"User*" = ["std::fmt"]
        "#;
        fs::write(nested_dir.join("code-actions.toml"), nested_config).unwrap();

        // Load config from nested directory
        let config = CodeActionsConfig::load_from_anchor(&nested_dir).unwrap();

        // Verify merged configuration
        assert!(config.extra_derives.contains_key("*Error"));
        assert!(config.extra_derives.contains_key("*"));
        assert!(config.extra_derives.contains_key("User*"));

        // With adjoin, arrays should be concatenated
        let error_derives = &config.extra_derives["*Error"];
        assert!(error_derives.contains(&"Clone".to_string()));
        assert!(error_derives.contains(&"Debug".to_string()));
        assert!(error_derives.contains(&"PartialEq".to_string()));

        // Test extra use statements
        assert!(config.extra_use_statements.contains_key("*Error"));
        assert!(config.extra_use_statements.contains_key("User*"));

        let error_uses = &config.extra_use_statements["*Error"];
        assert!(error_uses.contains(&"serde::{Serialize, Deserialize}".to_string()));

        let user_uses = &config.extra_use_statements["User*"];
        assert!(user_uses.contains(&"std::fmt".to_string()));
    }

    #[test]
    fn test_extra_derives_generation() {
        use crate::generate_struct::get_regular_struct_token_stream_with_config;
        use quote::format_ident;

        // Create a sample config
        let mut config = CodeActionsConfig::default();
        config
            .extra_derives
            .insert("UserStruct".to_string(), vec!["Serialize".to_string(), "Deserialize".to_string()]);
        config
            .extra_use_statements
            .insert("UserStruct".to_string(), vec!["serde::{Serialize, Deserialize}".to_string()]);

        let struct_name = format_ident!("UserStruct");
        let token_stream = get_regular_struct_token_stream_with_config(struct_name, &config, "UserStruct");

        let code_string = token_stream.to_string();

        // Should contain the extra derive
        assert!(code_string.contains("Serialize"));
        assert!(code_string.contains("Deserialize"));

        // Should contain the extra use statement
        assert!(code_string.contains("serde"));
    }
}
