use std::fmt::{Formatter, Result as FmtResult};

use quote::IdentFragment;

use crate::types::type_name::TypeName;

impl IdentFragment for TypeName {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}
