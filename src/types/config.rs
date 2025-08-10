use derive_more::{Error, From};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use fmt_derive::Display;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, path::Path};

/// Configuration for code actions with extra derives and use statements
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct CodeActionsConfig {
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

#[derive(Error, Display, From, Debug)]
pub enum LoadConfigError {
    #[display("Configuration error: {_0}")]
    FigmentError(Box<figment::Error>),
    #[display("IO error while reading config: {_0}")]
    #[from]
    IoError(std::io::Error),
    #[display("Invalid regex pattern: {_0}")]
    #[from]
    RegexError(regex::Error),
    #[display("Validation error: {_0}")]
    #[error(ignore)]
    ValidationError(String),
}

impl From<figment::Error> for LoadConfigError {
    fn from(error: figment::Error) -> Self {
        LoadConfigError::FigmentError(Box::new(error))
    }
}

impl LoadConfigError {
    pub fn is_config_not_found(&self) -> bool {
        matches!(self, LoadConfigError::FigmentError(_))
    }

    pub fn is_regex_error(&self) -> bool {
        matches!(self, LoadConfigError::RegexError(_))
    }
}

impl CodeActionsConfig {
    /// Load configuration from anchor path, walking up to workspace root
    ///
    /// This method searches for `code-actions.toml` files from the anchor path
    /// up to the workspace root (directory containing `Cargo.toml`), merging
    /// configurations using the `adjoin` strategy.
    ///
    /// # Arguments
    /// * `anchor_path` - Starting path (file or directory) to begin search from
    ///
    /// # Returns
    /// * `Ok(CodeActionsConfig)` - Merged configuration with compiled regex patterns
    /// * `Err(LoadConfigError)` - If configuration loading or regex compilation fails
    ///
    /// # Example
    /// ```no_run
    /// use code_actions::types::config::CodeActionsConfig;
    ///
    /// let config = CodeActionsConfig::load_from_anchor("./src/lib.rs")?;
    /// let derives = config.get_extra_derives_for_name("UserError");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn load_from_anchor<P: AsRef<Path>>(anchor_path: P) -> Result<Self, LoadConfigError> {
        let anchor_path = anchor_path.as_ref();
        let start_path = if anchor_path.is_file() { anchor_path.parent().unwrap_or(anchor_path) } else { anchor_path };

        let config_paths = Self::collect_config_paths(start_path);
        let mut figment = Figment::new();

        for config_path in config_paths.into_iter().rev() {
            figment = figment.adjoin(Toml::file(&config_path));
        }

        figment = figment.adjoin(Env::prefixed("CODE_ACTIONS_"));
        let mut config: CodeActionsConfig = figment.extract()?;

        // Validate configuration before compiling regex
        config.validate()?;
        config.compile_regex_patterns()?;

        Ok(config)
    }

    fn collect_config_paths(start_path: &Path) -> Vec<std::path::PathBuf> {
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
    fn compile_regex_patterns(&mut self) -> Result<(), LoadConfigError> {
        for extra in &mut self.extra {
            if extra.matches.is_empty() {
                return Err(LoadConfigError::ValidationError("Empty regex pattern not allowed".to_string()));
            }

            extra.regex = Some(Regex::new(&extra.matches)?);
        }
        Ok(())
    }

    /// Get extra derive macros for a given type name
    ///
    /// Returns all matching derive macros from configuration rules that match the type name,
    /// with duplicates removed while preserving order.
    pub fn get_extra_derives_for_name(&self, type_name: &str) -> Vec<String> {
        self.collect_matching_items(type_name, |extra| &extra.derive)
    }

    /// Get extra use statements for a given type name
    ///
    /// Returns all matching use statements from configuration rules that match the type name,
    /// with duplicates removed while preserving order.
    pub fn get_extra_use_statements_for_name(&self, type_name: &str) -> Vec<String> {
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
    fn validate(&self) -> Result<(), LoadConfigError> {
        for extra in &self.extra {
            // Check for empty patterns
            if extra.matches.is_empty() {
                return Err(LoadConfigError::ValidationError("Empty regex pattern not allowed".to_string()));
            }

            // Check for potentially dangerous patterns
            if extra.matches == ".*" && !extra.derive.is_empty() {
                // Warn about overly broad patterns, but don't fail
                tracing::warn!("Broad pattern '.*' will apply {:?} to all types", extra.derive);
            }
        }
        Ok(())
    }

    fn collect_matching_items<F>(&self, type_name: &str, getter: F) -> Vec<String>
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
                    .filter(|regex| regex.is_match(type_name))
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
        let config = CodeActionsConfig::load_from_anchor(&nested_dir).unwrap();

        // Verify merged configuration - should have configurations from both files
        assert!(!config.extra.is_empty());

        // Test that patterns work correctly
        let error_derives = config.get_extra_derives_for_name("MyError");
        assert!(error_derives.contains(&"Clone".to_string()));
        assert!(error_derives.contains(&"Debug".to_string()));
        assert!(error_derives.contains(&"PartialEq".to_string())); // From nested config

        let user_derives = config.get_extra_derives_for_name("UserStruct");
        assert!(user_derives.contains(&"Serialize".to_string()));
        assert!(user_derives.contains(&"Deserialize".to_string()));
        assert!(user_derives.contains(&"Debug".to_string())); // From wildcard pattern

        // Test extra use statements
        let error_uses = config.get_extra_use_statements_for_name("MyError");
        assert!(error_uses.contains(&"serde::{Serialize, Deserialize}".to_string()));

        let user_uses = config.get_extra_use_statements_for_name("UserStruct");
        assert!(user_uses.contains(&"std::fmt".to_string()));
    }

    #[test]
    fn test_extra_derives_generation() {
        use crate::generate_struct::get_regular_struct_token_stream_with_config;
        use quote::format_ident;

        // Create a sample config
        let config = CodeActionsConfig {
            extra: vec![ExtraConfig {
                matches: "UserStruct".to_string(),
                derive: vec!["Serialize".to_string(), "Deserialize".to_string()],
                r#use: vec!["serde::{Serialize, Deserialize}".to_string()],
                regex: Some(Regex::new("UserStruct").unwrap()),
            }],
        };

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
        let mut config = CodeActionsConfig {
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

        // Compile regex patterns
        config.compile_regex_patterns().unwrap();

        // Test User.* pattern
        let user_derives = config.get_extra_derives_for_name("UserStruct");
        assert!(user_derives.contains(&"Serialize".to_string()));
        assert!(user_derives.contains(&"Debug".to_string())); // From wildcard
        let user_uses = config.get_extra_use_statements_for_name("UserStruct");
        assert!(user_uses.contains(&"serde::Serialize".to_string()));

        // Test .*Error$ pattern
        let error_derives = config.get_extra_derives_for_name("MyError");
        assert!(error_derives.contains(&"Error".to_string()));
        assert!(error_derives.contains(&"Debug".to_string())); // From wildcard

        // Test that UserError matches both patterns
        let user_error_derives = config.get_extra_derives_for_name("UserError");
        assert!(user_error_derives.contains(&"Serialize".to_string())); // From User.*
        assert!(user_error_derives.contains(&"Error".to_string())); // From .*Error$
        assert!(user_error_derives.contains(&"Debug".to_string())); // From wildcard

        // Test non-matching name
        let other_derives = config.get_extra_derives_for_name("SomeStruct");
        assert!(!other_derives.contains(&"Serialize".to_string()));
        assert!(!other_derives.contains(&"Error".to_string()));
        assert!(other_derives.contains(&"Debug".to_string())); // From wildcard only
    }
}
