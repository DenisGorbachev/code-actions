use derive_more::{Error, From};
use fmt_derive::Display;

#[derive(Error, Display, From, Clone, Debug)]
pub enum ConfigCompileRegexPatternsError {
    #[display("Empty regex pattern not allowed")]
    EmptyPattern,
    #[display("Invalid regex pattern: {_0}")]
    RegexError(#[from] regex::Error),
}
