use std::path::{Path, PathBuf};

use derive_more::Error;
use derive_new::new;
use fmt_derive::Display;

use crate::constants::*;

#[derive(new, Error, Display, Debug)]
pub struct PrimaryModuleNotFound {
    pub path_buf: PathBuf,
}

pub fn get_primary_module_path(src: impl AsRef<Path>) -> Result<PathBuf, PrimaryModuleNotFound> {
    let src = src.as_ref();
    for name in PRIMARY_FILE_NAMES {
        let primary_module = src.join(name);
        if primary_module.exists() {
            return Ok(primary_module);
        }
    }
    Err(PrimaryModuleNotFound::new(src.to_path_buf()))
}
