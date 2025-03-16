use crate::traits::rename_module::RenameModule;
use derive_more::Error;
use fmt_derive::Display;

impl RenameModule for syn::File {
    type Output = Result<Self, SynFileRenameModuleError>;

    fn rename_module(self, _module_name_old: &str, _module_name_new: &str) -> Self::Output {
        todo!()
    }
}

type ModuleName = String;

#[derive(Error, Display, Eq, PartialEq, Hash, Clone, Debug)]
pub enum SynFileRenameModuleError {
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
    use quote::ToTokens;
    use syn::parse_file;
    use SynFileRenameModuleError::*;

    const INPUT: &str = "
        mod foo;
        mod bar;
        mod foobar;

        pub use foo::zed::*;
        pub use foo::other;
        pub use bar::something;
        pub use foobar::*;

        pub struct Foo {}
    ";

    const FOO: &str = "foo";

    const BAR: &str = "bar";

    fn input() -> syn::File {
        parse_file(INPUT).unwrap()
    }

    /// It must rename the module if the name matches exactly
    /// It must not rename the module if the name starts with, but doesn't match exactly
    #[test]
    #[ignore]
    fn must_rename_module() {
        let input = input();
        let output = input
            .rename_module("foo", "qux")
            .expect("should rename successfully");
        let output_string_actual = output.to_token_stream().to_string();
        let output_string_expected = INPUT
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
