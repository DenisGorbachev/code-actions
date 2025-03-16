use crate::traits::rename_module::RenameModule;
use derive_more::Error;
use fmt_derive::Display;

impl RenameModule for &mut syn::File {
    type Output = Result<(), SynFileRenameModuleError>;

    fn rename_module(self, module_name_old: &str, module_name_new: &str) -> Self::Output {
        // Check if new name equals old name
        if module_name_old == module_name_new {
            return Err(SynFileRenameModuleError::NewNameIsEqualToOldName(module_name_new.to_string()));
        }

        // Check if a module with the new name already exists
        for item in &self.items {
            if let syn::Item::Mod(module) = item {
                if module.ident == module_name_new {
                    return Err(SynFileRenameModuleError::NameAlreadyExists(module_name_new.to_string()));
                }
            }
        }

        // Rename the module declarations and use statements
        for item in &mut self.items {
            match item {
                // Handle module declarations
                syn::Item::Mod(module) => {
                    if module.ident == module_name_old {
                        module.ident = syn::Ident::new(module_name_new, module.ident.span());
                    }
                }
                // Handle use statements
                syn::Item::Use(use_item) => {
                    *use_item = rename_use_paths(use_item, module_name_old, module_name_new);
                }
                _ => {}
            }
        }

        Ok(())
    }
}

type ModuleName = String;

// Helper functions
fn rename_use_paths(use_item: &syn::ItemUse, old_name: &str, new_name: &str) -> syn::ItemUse {
    let mut new_use_item = use_item.clone();
    rename_use_tree(&mut new_use_item.tree, old_name, new_name);
    new_use_item
}

fn rename_use_tree(tree: &mut syn::UseTree, old_name: &str, new_name: &str) {
    match tree {
        // For path::to::module
        syn::UseTree::Path(path) => {
            if path.ident == old_name {
                path.ident = syn::Ident::new(new_name, path.ident.span());
            }
            rename_use_tree(&mut path.tree, old_name, new_name);
        }
        // For nested imports like path::{a, b, c}
        syn::UseTree::Group(group) => {
            for item in &mut group.items {
                rename_use_tree(item, old_name, new_name);
            }
        }
        // For glob imports like path::*
        syn::UseTree::Glob(_) => {}
        // For renamed imports like path::item as alias
        syn::UseTree::Rename(_) => {}
        // For single item imports like item
        syn::UseTree::Name(_) => {}
    }
}

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
    use prettyplease::unparse;
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
    fn must_rename_module() {
        let input = input();
        let mut output = input.clone();
        output
            .rename_module("foo", "qux")
            .expect("should rename successfully");
        let output_string_actual = unparse(&output);
        let output_string_expected = INPUT
            .trim()
            .replace("mod foo;", "mod qux;")
            .replace("pub use foo::zed::*;", "pub use qux::zed::*;")
            .replace("pub use foo::other;", "pub use qux::other;");
        // NOTE: It must not rename "foobar" to "quxbar"
        assert_eq!(output_string_actual.trim(), output_string_expected.trim());
    }

    #[test]
    fn must_return_error_if_names_are_equal() {
        assert_eq!(input().rename_module(FOO, FOO).unwrap_err(), NewNameIsEqualToOldName(FOO.into()));
    }

    #[test]
    fn must_not_rename_if_name_already_exists() {
        assert_eq!(input().rename_module(FOO, BAR).unwrap_err(), NameAlreadyExists(BAR.into()))
    }
}
