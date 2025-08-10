use derive_more::{Error, From};
use derive_new::new;
use fmt_derive::Display;
use std::path::PathBuf;

use super::{ConfigCompileRegexPatternsError, ConfigMatchesEmptyError};

#[derive(new, Error, Display, From, Clone, Debug)]
pub struct ConfigLoadFromAnchorError {
    #[error(not(source))]
    pub anchor_path: PathBuf,
    pub reason: ConfigLoadFromAnchorErrorReason,
}

#[derive(Error, Display, From, Clone, Debug)]
pub enum ConfigLoadFromAnchorErrorReason {
    FigmentExtract(#[error(not(source))] Box<figment::Error>),
    Validate(ConfigMatchesEmptyError),
    CompileRegexPatterns(ConfigCompileRegexPatternsError),
}

impl From<figment::Error> for ConfigLoadFromAnchorErrorReason {
    fn from(error: figment::Error) -> Self {
        Self::FigmentExtract(Box::new(error))
    }
}
