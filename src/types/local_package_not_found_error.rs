use derive_more::Error;
use derive_new::new;
use fmt_derive::Display;

use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;

#[derive(new, Error, Display, Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
pub struct LocalPackageNotFoundError {
    path: Utf8PathBuf,
}

impl LocalPackageNotFoundError {}
