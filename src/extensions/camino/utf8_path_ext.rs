use std::iter::Skip;

use camino::{Utf8Ancestors, Utf8Path};

pub trait Utf8PathExt {
    // type Iter: Iterator<Item = Self>;

    fn parents(&self) -> Skip<Utf8Ancestors<'_>>;

    // fn parents_up_to<'a>(&'a self, parent: &'a Self) -> Self::Iter;

    // fn find_dir_containing_filename(&self, filename: &str) -> Option<&Self>;
}

impl Utf8PathExt for Utf8Path {
    fn parents(&self) -> Skip<Utf8Ancestors<'_>> {
        self.ancestors().skip(1)
    }
}
