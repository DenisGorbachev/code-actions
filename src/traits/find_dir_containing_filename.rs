use crate::extensions::camino::utf8_path::Utf8Path;
use std::path::Path;

pub trait FindDirContainingFilename {
    fn find_dir_containing_filename(&self, filename: impl AsRef<Self>) -> Option<&Self>;
}

impl FindDirContainingFilename for Path {
    fn find_dir_containing_filename(&self, filename: impl AsRef<Self>) -> Option<&Self> {
        let filename = filename.as_ref();
        self.ancestors().find(|it| it.join(filename).exists())
    }
}

impl FindDirContainingFilename for camino::Utf8Path {
    fn find_dir_containing_filename(&self, filename: impl AsRef<Self>) -> Option<&Self> {
        let filename = filename.as_ref();
        self.ancestors().find(|it| it.join(filename).exists())
    }
}

impl FindDirContainingFilename for Utf8Path {
    fn find_dir_containing_filename(&self, filename: impl AsRef<Self>) -> Option<&Self> {
        let filename = filename.as_ref();
        self.ancestors().find(|it| it.join(filename).exists())
    }
}
