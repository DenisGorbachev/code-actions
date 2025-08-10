use derive_more::Error;
use derive_new::new;
use fmt_derive::Display;

#[derive(new, Error, Display, Eq, PartialEq, Clone, Debug)]
#[display("Empty regex pattern not allowed")]
pub struct CodeActionsConfigValidateError;
