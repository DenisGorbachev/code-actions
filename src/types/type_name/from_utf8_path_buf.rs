use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::types::type_name::TypeName;

impl From<&Utf8PathBuf> for TypeName {
    fn from(value: &Utf8PathBuf) -> Self {
        TypeName::from(value.as_str())
    }
}
