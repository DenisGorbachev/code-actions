use toml_edit::{Item, Table};

use crate::types::get_table_from_item_error::GetTableFromItemError;

pub fn get_table_from_item(item: &Item) -> Result<&Table, GetTableFromItemError> {
    item.as_table().ok_or(GetTableFromItemError)
}
