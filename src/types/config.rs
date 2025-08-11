use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::errors::{ConfigCompileRegexPatternsError, ConfigLoadFromAnchorError, ConfigMatchesEmptyError};

/// Configuration for code actions
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Config {
    /// List of extra configuration rules for matching types
    pub extra: Vec<ExtraConfig>,
}

/// Configuration rule for applying extra derives and use statements to matching types
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ExtraConfig {
    /// Regex pattern to match type names against
    pub matches: String,
    /// Additional use statements to include when this pattern matches
    #[serde(default)]
    pub r#use: Vec<String>,
    /// Additional derive macros to apply when this pattern matches
    #[serde(default)]
    pub derive: Vec<String>,
    /// Compiled regex pattern (not serialized)
    #[serde(skip)]
    regex: Option<Regex>,
}

impl TryFrom<&Path> for Config {
    type Error = ConfigLoadFromAnchorError;

    fn try_from(anchor_path: &Path) -> Result<Self, Self::Error> {
        let start_path = if anchor_path.is_file() { anchor_path.parent().unwrap_or(anchor_path) } else { anchor_path };

        let config_paths = Self::collect_config_paths(start_path);
        let mut figment = Figment::new();

        // If no config files exist, return default
        if config_paths.is_empty() {
            return Ok(Config::default());
        }

        for config_path in config_paths.into_iter().rev() {
            figment = figment.adjoin(Toml::file(&config_path));
        }

        figment = figment.admerge(Env::prefixed("CODE_ACTIONS_"));
        let mut config: Config = figment
            .extract()
            .map_err(|e| ConfigLoadFromAnchorError::new(anchor_path.to_path_buf(), e.into()))?;

        config
            .validate()
            .map_err(|e| ConfigLoadFromAnchorError::new(anchor_path.to_path_buf(), e.into()))?;
        config
            .compile_regex_patterns()
            .map_err(|e| ConfigLoadFromAnchorError::new(anchor_path.to_path_buf(), e.into()))?;

        Ok(config)
    }
}

impl Config {
    fn collect_config_paths(start_path: &Path) -> Vec<PathBuf> {
        let mut config_paths = Vec::new();
        let mut current_path = start_path;

        loop {
            let config_file = current_path.join("code-actions.toml");
            if config_file.exists() {
                config_paths.push(config_file);
            }

            if current_path.join("Cargo.toml").exists() {
                break;
            }

            if let Some(parent) = current_path.parent() {
                current_path = parent;
            } else {
                break;
            }
        }

        config_paths
    }

    /// Compile regex patterns for all extra configs with validation
    fn compile_regex_patterns(&mut self) -> Result<(), ConfigCompileRegexPatternsError> {
        for extra in &mut self.extra {
            if extra.matches.is_empty() {
                return Err(ConfigCompileRegexPatternsError::EmptyPattern);
            }

            extra.regex = Some(Regex::new(&extra.matches)?);
        }
        Ok(())
    }

    /// Get extra derive macros for a given type name
    ///
    /// Returns all matching derive macros from configuration rules that match the type name,
    /// with duplicates removed while preserving order.
    pub fn get_extra_derives_for_name(&self, type_name: &impl AsRef<str>) -> Vec<String> {
        self.collect_matching_items(type_name, |extra| &extra.derive)
    }

    /// Get extra use statements for a given type name
    ///
    /// Returns all matching use statements from configuration rules that match the type name,
    /// with duplicates removed while preserving order.
    pub fn get_extra_use_statements_for_name(&self, type_name: &impl AsRef<str>) -> Vec<String> {
        self.collect_matching_items(type_name, |extra| &extra.r#use)
    }

    /// Check if any configuration rules match the given type name
    pub fn has_matches_for_name(&self, type_name: &str) -> bool {
        self.extra.iter().any(|extra| {
            extra
                .regex
                .as_ref()
                .is_some_and(|regex| regex.is_match(type_name))
        })
    }

    /// Validate configuration rules
    fn validate(&self) -> Result<(), ConfigMatchesEmptyError> {
        for extra in &self.extra {
            if extra.matches.is_empty() {
                return Err(ConfigMatchesEmptyError::new());
            }
        }
        Ok(())
    }

    fn collect_matching_items<F>(&self, type_name: &impl AsRef<str>, getter: F) -> Vec<String>
    where
        F: Fn(&ExtraConfig) -> &[String],
    {
        let items: Vec<String> = self
            .extra
            .iter()
            .filter_map(|extra| {
                extra
                    .regex
                    .as_ref()
                    .filter(|regex| regex.is_match(type_name.as_ref()))
                    .map(|_| getter(extra))
            })
            .flat_map(|items| items.iter().cloned())
            .collect();

        Self::deduplicate_preserving_order(items)
    }

    fn deduplicate_preserving_order(items: Vec<String>) -> Vec<String> {
        let mut seen = HashSet::with_capacity(items.len());
        items
            .into_iter()
            .filter(|item| seen.insert(item.clone()))
            .collect()
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
[[extra]]
matches = ".*Error$"
derive = ["Clone", "Debug"]

[[extra]]
matches = ".*"
derive = ["Debug"]
        "#;
        fs::write(workspace_root.join("code-actions.toml"), workspace_config).unwrap();

        // Create nested directory with its own config
        let nested_dir = workspace_root.join("src").join("nested");
        fs::create_dir_all(&nested_dir).unwrap();

        let nested_config = r#"
[[extra]]
matches = ".*Error$"
derive = ["Clone", "Debug", "PartialEq"]
use = ["serde::{Serialize, Deserialize}"]

[[extra]]
matches = "User.*"
derive = ["Serialize", "Deserialize"]
use = ["std::fmt"]
        "#;
        fs::write(nested_dir.join("code-actions.toml"), nested_config).unwrap();

        // Load config from nested directory
        let config = Config::try_from(nested_dir.as_path()).unwrap();

        // Verify merged configuration - should have configurations from both files
        assert!(!config.extra.is_empty());

        // Test that patterns work correctly
        let error_derives = config.get_extra_derives_for_name(&"MyError");
        assert!(error_derives.contains(&"Clone".to_string()));
        assert!(error_derives.contains(&"Debug".to_string()));
        assert!(error_derives.contains(&"PartialEq".to_string())); // From nested config

        let user_derives = config.get_extra_derives_for_name(&"UserStruct");
        assert!(user_derives.contains(&"Serialize".to_string()));
        assert!(user_derives.contains(&"Deserialize".to_string()));
        assert!(user_derives.contains(&"Debug".to_string())); // From wildcard pattern

        // Test extra use statements
        let error_uses = config.get_extra_use_statements_for_name(&"MyError");
        assert!(error_uses.contains(&"serde::{Serialize, Deserialize}".to_string()));

        let user_uses = config.get_extra_use_statements_for_name(&"UserStruct");
        assert!(user_uses.contains(&"std::fmt".to_string()));
    }

    #[test]
    fn test_extra_derives_generation() {
        use crate::generate_struct::get_regular_struct_token_stream_with_config;
        use quote::format_ident;

        // Create a sample config
        let mut config = Config {
            extra: vec![ExtraConfig {
                matches: "UserStruct".to_string(),
                derive: vec!["Serialize".to_string(), "Deserialize".to_string()],
                r#use: vec!["serde::{Serialize, Deserialize}".to_string()],
                regex: None,
            }],
        };
        config.compile_regex_patterns().unwrap();

        let struct_name = format_ident!("UserStruct");
        let token_stream = get_regular_struct_token_stream_with_config(struct_name, &config, "UserStruct");

        let code_string = token_stream.to_string();

        // Should contain the extra derive
        assert!(code_string.contains("Serialize"));
        assert!(code_string.contains("Deserialize"));

        // Should contain the extra use statement
        assert!(code_string.contains("serde"));
    }

    #[test]
    fn test_regex_matching() {
        let mut config = Config {
            extra: vec![
                ExtraConfig {
                    matches: "User.*".to_string(),
                    derive: vec!["Serialize".to_string()],
                    r#use: vec!["serde::Serialize".to_string()],
                    regex: None,
                },
                ExtraConfig {
                    matches: ".*Error$".to_string(),
                    derive: vec!["Error".to_string()],
                    r#use: vec![],
                    regex: None,
                },
                ExtraConfig {
                    matches: ".*".to_string(),
                    derive: vec!["Debug".to_string()],
                    r#use: vec![],
                    regex: None,
                },
            ],
        };
        config.compile_regex_patterns().unwrap();

        // Test User.* pattern
        let user_derives = config.get_extra_derives_for_name(&"UserStruct");
        assert!(user_derives.contains(&"Serialize".to_string()));
        assert!(user_derives.contains(&"Debug".to_string())); // From wildcard
        let user_uses = config.get_extra_use_statements_for_name(&"UserStruct");
        assert!(user_uses.contains(&"serde::Serialize".to_string()));

        // Test .*Error$ pattern
        let error_derives = config.get_extra_derives_for_name(&"MyError");
        assert!(error_derives.contains(&"Error".to_string()));
        assert!(error_derives.contains(&"Debug".to_string())); // From wildcard

        // Test that UserError matches both patterns
        let user_error_derives = config.get_extra_derives_for_name(&"UserError");
        assert!(user_error_derives.contains(&"Serialize".to_string())); // From User.*
        assert!(user_error_derives.contains(&"Error".to_string())); // From .*Error$
        assert!(user_error_derives.contains(&"Debug".to_string())); // From wildcard

        // Test non-matching name
        let other_derives = config.get_extra_derives_for_name(&"SomeStruct");
        assert!(!other_derives.contains(&"Serialize".to_string()));
        assert!(!other_derives.contains(&"Error".to_string()));
        assert!(other_derives.contains(&"Debug".to_string())); // From wildcard only
    }
}
