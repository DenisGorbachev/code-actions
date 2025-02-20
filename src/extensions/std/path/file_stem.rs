use derive_more::{Deref, Display};

pub mod impl_try_from_path;
pub mod try_from_camino_utf8_path;
pub mod try_from_camino_utf8_path_buf;
pub mod try_from_utf8_path;

#[derive(Deref, Display, Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
pub struct FileStem<'a>(pub &'a str);
