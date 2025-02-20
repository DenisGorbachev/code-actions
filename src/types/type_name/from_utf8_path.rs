use crate::extensions::camino::utf8_path::Utf8Path;
use crate::types::type_name::TypeName;

impl From<&Utf8Path> for TypeName {
    fn from(value: &Utf8Path) -> Self {
        Self::from(value.as_str())
    }
}
