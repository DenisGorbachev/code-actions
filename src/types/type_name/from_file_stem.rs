use crate::extensions::std::path::file_stem::FileStem;
use crate::types::type_name::TypeName;

impl<'a> From<&FileStem<'a>> for TypeName {
    fn from(value: &FileStem<'a>) -> Self {
        Self(value.to_string())
    }
}
