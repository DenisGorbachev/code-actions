use derive_more::{Deref, Display};

pub mod from_camino_utf8_path;
pub mod from_file_stem;
pub mod from_str;
pub mod from_utf8_path;
pub mod from_utf8_path_buf;
pub mod ident_fragment;
pub mod to_tokens;

#[derive(Deref, Display, Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
pub struct TypeName(pub String);

impl TypeName {}
