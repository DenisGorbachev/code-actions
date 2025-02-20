use std::fmt::Formatter;

use quote::IdentFragment;

use crate::types::type_name::TypeName;

impl IdentFragment for TypeName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
