use derive_more::Error;
use derive_new::new;
use fmt_derive::Display;

#[derive(new, Error, Display, Eq, PartialEq, Hash, Clone, Debug)]
pub struct GetTableFromItemError;
