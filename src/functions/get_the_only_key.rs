use derive_more::Error;
use derive_new::new;
use fmt_derive::Display;

#[derive(new, Error, Display, Eq, PartialEq, Hash, Clone, Debug)]
pub struct IteratorMustContainAtLeastOneItemError;

pub fn get_first_item<Item>(mut iter: impl Iterator<Item = Item>) -> Result<Item, IteratorMustContainAtLeastOneItemError> {
    iter.next().ok_or(IteratorMustContainAtLeastOneItemError)
}
