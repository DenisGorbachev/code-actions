use derive_getters::{Dissolve, Getters};
use derive_more::{Deref, DerefMut};
use derive_new::new;
use fs_err::{read_to_string, write};
use std::io;
use toml_edit::DocumentMut;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;

#[derive(new, Deref, DerefMut, Getters, Dissolve, Default, Clone, Debug)]
pub struct TomlFile {
    path_buf: Utf8PathBuf,
    #[deref]
    #[deref_mut]
    doc: DocumentMut,
}

impl TomlFile {
    pub fn modify<M>(&mut self, mutator: M) -> Result<(), TomlFileIoError>
    where
        M: FnOnce(&mut DocumentMut),
    {
        mutator(&mut self.doc);
        write(&self.path_buf, self.doc.to_string())?;
        Ok(())
    }

    pub fn path(&self) -> &Utf8Path {
        self.path_buf.as_path()
    }
}

impl TryFrom<&Utf8Path> for TomlFile {
    type Error = <Self as TryFrom<Utf8PathBuf>>::Error;

    fn try_from(value: &Utf8Path) -> Result<Self, Self::Error> {
        Self::try_from(value.to_path_buf())
    }
}

impl TryFrom<Utf8PathBuf> for TomlFile {
    type Error = TomlFileIoError;

    fn try_from(value: Utf8PathBuf) -> Result<Self, Self::Error> {
        let contents = read_to_string(&value)?;
        let doc = contents.parse::<DocumentMut>()?;
        Ok(Self::new(value, doc))
    }
}

#[derive(derive_more::Error, fmt_derive::Display, derive_more::From, Debug)]
pub enum TomlFileIoError {
    TheIoError(io::Error),
    TheTomlEditError(toml_edit::TomlError),
}
