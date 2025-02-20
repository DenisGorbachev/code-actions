use std::borrow::Borrow;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use derive_more::{Deref, DerefMut, Display};
use derive_new::new;

use crate::extensions::camino::utf8_path::Utf8Path;
use standard_traits::Contains;

pub mod clap_value_parser;
pub mod from_str;
pub mod from_utf8_path;
pub mod from_utf8_path_buf;
pub mod into_path_buf;
pub mod try_from_os_str;
pub mod try_from_path_buf;

#[derive(new, Deref, DerefMut, Display, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Utf8PathBuf(pub camino::Utf8PathBuf);

impl Utf8PathBuf {
    pub fn as_path(&self) -> &Utf8Path {
        Utf8Path::new(self.0.as_path())
    }

    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
        Utf8Path::join(self.as_path(), path)
    }
}

impl Borrow<Utf8Path> for Utf8PathBuf {
    fn borrow(&self) -> &Utf8Path {
        self.as_path()
    }
}

impl AsRef<Utf8Path> for Utf8PathBuf {
    fn as_ref(&self) -> &Utf8Path {
        self.as_path()
    }
}

impl AsRef<Path> for Utf8PathBuf {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<str> for Utf8PathBuf {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<OsStr> for Utf8PathBuf {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl From<Utf8PathBuf> for PathBuf {
    fn from(value: Utf8PathBuf) -> Self {
        value.0.into_std_path_buf()
    }
}

impl TryFrom<&Path> for Utf8PathBuf {
    type Error = <camino::Utf8PathBuf as TryFrom<PathBuf>>::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let inner = camino::Utf8PathBuf::try_from(value.to_path_buf())?;
        Ok(Self::from(inner))
    }
}

impl Contains<Utf8Path> for Utf8PathBuf {
    fn contains(&self, other: &Utf8Path) -> bool {
        self.as_std_path().contains(other.as_std_path())
    }
}

impl Contains<Utf8PathBuf> for Utf8PathBuf {
    fn contains(&self, other: &Utf8PathBuf) -> bool {
        self.as_std_path().contains(other.as_std_path())
    }
}
