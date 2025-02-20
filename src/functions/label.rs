use crate::extensions::camino::utf8_path::Utf8Path;
use crate::types::label::{Label, LabelSlice};
use heck::{ToSnakeCase, ToUpperCamelCase};
use not_found_error::{NotFoundError, Require};
use proc_macro2::{Ident, Span};

pub fn to_ident(label: &LabelSlice) -> Ident {
    Ident::new(&label.to_upper_camel_case(), Span::call_site())
}

pub fn to_stem(label: &LabelSlice) -> String {
    label.to_snake_case()
}

pub fn opt_from_utf8_path(path: &Utf8Path) -> Option<&LabelSlice> {
    path.file_stem()
}

pub fn try_from_utf8_path(path: &Utf8Path) -> Result<Label, NotFoundError<Label>> {
    path.file_stem().map(ToOwned::to_owned).require()
}
