use heck::ToUpperCamelCase;

use crate::types::type_name::TypeName;

impl From<&str> for TypeName {
    fn from(value: &str) -> Self {
        Self(value.to_upper_camel_case())
    }
}
