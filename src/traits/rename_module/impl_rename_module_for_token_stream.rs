use crate::traits::rename_module::RenameModule;
use derive_more::Error;
use fmt_derive::Display;
use proc_macro2::TokenStream;

impl RenameModule for TokenStream {
    type Output = Result<Self, TokenStreamRenameModuleError>;

    fn rename_module(self, _module_name_old: &str, _module_name_new: &str) -> Self::Output {
        todo!()
    }
}

type ModuleName = String;

#[derive(Error, Display, Eq, PartialEq, Hash, Clone, Debug)]
pub enum TokenStreamRenameModuleError {
    #[display("A module with the specified name already exists: {_0}")]
    #[error(ignore)]
    NameAlreadyExists(ModuleName),

    #[display("New name is equal to old name: {_0}")]
    #[error(ignore)]
    NewNameIsEqualToOldName(ModuleName),
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use proc_macro2::TokenStream;
    use quote::quote;
    use TokenStreamRenameModuleError::*;

    fn input() -> TokenStream {
        quote! {
            mod foo;
            mod bar;
            mod foobar;

            pub use foo::zed::*;
            pub use foo::other;
            pub use bar::something;
            pub use foobar::*;

            pub struct Foo {}
        }
    }

    const FOO: &str = "foo";
    const BAR: &str = "bar";

    /// It must rename the module if the name matches exactly
    /// It must not rename the module if the name starts with, but doesn't match exactly
    #[test]
    #[ignore]
    fn must_rename_module() {
        let input = input();
        let input_string = input.to_string();
        let output = input
            .rename_module("foo", "qux")
            .expect("should rename successfully");
        let output_string_actual = output.to_string();
        let output_string_expected = input_string
            .replace("mod foo;", "mod qux;")
            .replace("pub use foo::zed::*;", "pub use qux::zed::*;")
            .replace("pub use foo::other;", "pub use qux::other;");
        // NOTE: It must not rename "foobar" to "quxbar"
        assert_eq!(output_string_actual, output_string_expected);
    }

    #[test]
    #[ignore]
    fn must_return_error_if_names_are_equal() {
        assert_eq!(input().rename_module(FOO, FOO).unwrap_err(), NewNameIsEqualToOldName(FOO.into()));
    }

    #[test]
    #[ignore]
    fn must_not_rename_if_name_already_exists() {
        assert_eq!(input().rename_module(FOO, BAR).unwrap_err(), NameAlreadyExists(BAR.into()))
    }
}
