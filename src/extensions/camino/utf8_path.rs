use std::ffi::OsStr;
use std::iter::Map;
use std::path::Path;

use anyhow::Context;
use camino::Utf8Ancestors;
use derive_more::{Deref, Display};

use crate::types::outcome::Outcome;

use crate::constants::{CARGO_TOML_FILE_NAME, SRC_DIR_NAME};
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::extensions::camino::utf8_path_ext::Utf8PathExt;
use crate::extensions::tempfile::temp_dir::TempDir;
use standard_traits::{Contains, Join};

pub mod try_from_temp_dir;

#[derive(Deref, Display, Ord, PartialOrd, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct Utf8Path(camino::Utf8Path);

impl Utf8Path {
    pub fn new(s: &(impl AsRef<str> + ?Sized)) -> &Self {
        unsafe { &*(camino::Utf8Path::new(s) as *const camino::Utf8Path as *const Utf8Path) }
    }

    pub fn ancestors(&self) -> Map<Utf8Ancestors<'_>, fn(&camino::Utf8Path) -> &Utf8Path> {
        self.0.ancestors().map(Utf8Path::new)
    }

    pub fn parent(&self) -> Option<&Self> {
        self.0.parent().map(Self::new)
    }

    pub fn join(&self, path: impl AsRef<Self>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.join(&path.as_ref().0))
    }

    pub fn to_path_buf(&self) -> Utf8PathBuf {
        Utf8PathBuf(self.0.to_path_buf())
    }

    pub fn find_src_root(&self) -> Option<&Utf8Path> {
        self.ancestors()
            .find(|it| it.join(CARGO_TOML_FILE_NAME).exists() && it.join(SRC_DIR_NAME).exists())
    }

    pub fn get_src_root(&self) -> Outcome<&Utf8Path> {
        self.find_src_root()
            .with_context(|| format!("Could not find the package root (a directory containing {} and {}), starting from {}", CARGO_TOML_FILE_NAME, SRC_DIR_NAME, self))
    }

    pub fn ancestors_up_to<'a>(&'a self, ancestor: &'a Self) -> impl Iterator<Item = &'a Self> {
        self.0
            .ancestors()
            .map(Utf8Path::new)
            .take_while(move |it| *it != ancestor)
    }

    pub fn parents_up_to<'a>(&'a self, parent: &'a Self) -> impl Iterator<Item = &'a Self> {
        self.0
            .parents()
            .map(Utf8Path::new)
            .take_while(move |it| *it != parent)
    }

    pub fn parents(&self) -> impl Iterator<Item = &Self> {
        self.0.parents().map(Utf8Path::new)
    }
}

pub fn get_utf8_path_ref_from_temp_dir(dir: &TempDir) -> Option<&Utf8Path> {
    let camino_path = camino::Utf8Path::from_path(dir.path())?;
    Some(Utf8Path::new(camino_path))
}

pub fn find_dir_containing_filename(path: &camino::Utf8Path, filename: impl AsRef<camino::Utf8Path>) -> Option<&camino::Utf8Path> {
    let filename = filename.as_ref();
    path.ancestors().find(|it| it.join(filename).exists())
}

impl ToOwned for Utf8Path {
    type Owned = Utf8PathBuf;

    fn to_owned(&self) -> Self::Owned {
        self.to_path_buf()
    }
}

impl AsRef<Utf8Path> for Utf8Path {
    fn as_ref(&self) -> &Utf8Path {
        self
    }
}

impl AsRef<Utf8Path> for str {
    fn as_ref(&self) -> &Utf8Path {
        Utf8Path::new(self)
    }
}

impl AsRef<Utf8Path> for String {
    fn as_ref(&self) -> &Utf8Path {
        Utf8Path::new(self)
    }
}

impl AsRef<Path> for Utf8Path {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<str> for Utf8Path {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<OsStr> for Utf8Path {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl Contains<Utf8Path> for &Utf8Path {
    fn contains(&self, other: &Utf8Path) -> bool {
        self.as_std_path().contains(other.as_std_path())
    }
}

impl<'a> Join<&'a Utf8Path> for &Utf8Path {
    type Output = Utf8PathBuf;

    fn join(self, rhs: &'a Utf8Path) -> Self::Output {
        self.join(rhs)
    }
}

impl<'a> Join<&'a str> for &Utf8Path {
    type Output = Utf8PathBuf;

    fn join(self, rhs: &'a str) -> Self::Output {
        self.join(rhs)
    }
}
