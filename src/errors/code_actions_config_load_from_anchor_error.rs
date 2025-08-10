use camino::Utf8PathBuf;
use derive_more::{Error, From};
use derive_new::new;
use fmt_derive::Display;

use super::{CodeActionsConfigCompileRegexPatternsError, CodeActionsConfigValidateError};

#[derive(new, Error, Display, From, Clone, Debug)]
pub struct CodeActionsConfigLoadFromAnchorError {
    #[error(not(source))]
    pub anchor_path: Utf8PathBuf,
    pub reason: CodeActionsConfigLoadFromAnchorErrorReason,
}

#[derive(Error, Display, From, Clone, Debug)]
pub enum CodeActionsConfigLoadFromAnchorErrorReason {
    FigmentExtract(#[error(not(source))] Box<figment::Error>),
    Validate(CodeActionsConfigValidateError),
    CompileRegexPatterns(CodeActionsConfigCompileRegexPatternsError),
}

impl From<figment::Error> for CodeActionsConfigLoadFromAnchorErrorReason {
    fn from(error: figment::Error) -> Self {
        Self::FigmentExtract(Box::new(error))
    }
}
