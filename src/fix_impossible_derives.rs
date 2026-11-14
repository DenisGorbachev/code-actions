use crate::extensions::camino::utf8_path::Utf8Path;
use crate::functions::filter_map_impossible_derives::filter_map_impossible_derives;
use crate::functions::get_clippy_messages::get_clippy_compiler_messages;
use crate::functions::modify_rust_file::modify_and_format_rust_file;
use crate::types::outcome::Outcome;
use crate::types::package_info::PackageInfo;
use anyhow::Context;
use itertools::Itertools;
use not_found_error::Require;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{Attribute, File, Meta, Path, Token};
use syn_more::{get_main_item_mut, get_struct_or_enum_attrs_mut};

pub type PunctuatedIdents = Punctuated<Ident, Token![,]>;

pub fn fix_impossible_derives(anchor: &Utf8Path) -> Outcome {
    let package_info = PackageInfo::try_from(anchor)?;
    let project_root = package_info.project_root().require()?;
    let manifest_path_buf = project_root.manifest_path_buf();
    loop {
        let compiler_messages = get_clippy_compiler_messages(project_root.as_path())?;
        let impossible_derives = filter_map_impossible_derives(compiler_messages).collect_vec();
        let mut changed = false;
        modify_and_format_rust_file(anchor, &manifest_path_buf, |mut file| -> Outcome<File> {
            let item = get_main_item_mut(&mut file).with_context(|| format!("Main item not found in \"{anchor}\""))?;
            let attributes = get_struct_or_enum_attrs_mut(item).context("Main item must be a struct or enum")?;
            changed = remove_derives_many(attributes, &impossible_derives);
            Ok(file)
        })?;
        if !changed {
            return Ok(());
        }
    }
}

pub fn filter_derives(attr: &mut Attribute, filter: &impl FilterOf<Ident>) -> bool {
    if let Meta::List(ref mut meta_list) = attr.meta
        && meta_list.path.is_ident("derive")
    {
        let result = meta_list.parse_args_with(PunctuatedIdents::parse_terminated);
        match result {
            Ok(punctuated) => {
                let previous_len = punctuated.len();
                let filtered = punctuated.into_iter().filter(|ident| filter.filter(ident));
                let punctuated_new = PunctuatedIdents::from_iter(filtered);
                let changed = punctuated_new.len() != previous_len;
                meta_list.tokens = punctuated_new.to_token_stream();
                return changed;
            }
            Err(_) => {
                // intentionally do nothing
            }
        }
    }
    false
}

pub fn remove_derives_many(attributes: &mut [Attribute], filter: &impl FilterOf<Ident>) -> bool {
    attributes
        .iter_mut()
        .fold(false, |changed, attr| filter_derives(attr, filter) | changed)
}

pub trait FilterOf<T> {
    fn filter(&self, value: &T) -> bool;
}

impl FilterOf<Path> for Vec<Ident> {
    fn filter(&self, path: &Path) -> bool {
        match path.get_ident() {
            None => true,
            Some(ident) => !self.contains(ident),
        }
    }
}

impl FilterOf<Ident> for Vec<Ident> {
    fn filter(&self, ident: &Ident) -> bool {
        !self.contains(ident)
    }
}

#[cfg(test)]
mod tests {
    use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
    use crate::fix_impossible_derives::fix_impossible_derives;
    use crate::test_helpers::{get_lib_rs_path, get_temp_lib_root};
    use crate::types::outcome::Outcome;
    use prettyplease::unparse;
    use quote::ToTokens;
    use standard_traits::Get;
    use std::fs;
    use syn::{File, Item, ItemStruct, parse_quote};

    // #[derive(Eq, PartialEq, Copy, Clone, Debug)]
    // pub struct ProjectDirectory(std::path::PathBuf);

    #[test]
    fn must_remove_impossible_copy() -> Outcome {
        let item_before: ItemStruct = parse_quote! {
            #[derive(Eq, PartialEq, Copy, Clone, Debug)]
            pub struct ProjectDirectory(std::path::PathBuf);
        };
        // the subsequent calls should remove Copy, because PathBuf doesn't implement Copy
        let item_after: ItemStruct = parse_quote! {
            #[derive(Eq, PartialEq, Clone, Debug)]
            pub struct ProjectDirectory(std::path::PathBuf);
        };
        assert_item_equal_after_remove_impossible_derive(item_before, item_after)
    }

    // #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
    // pub struct MyMap(std::collections::HashMap<String, String>);

    #[test]
    fn must_remove_impossible_ord() -> Outcome {
        let item_before: ItemStruct = parse_quote! {
            #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
            pub struct MyMap(std::collections::HashMap<String, String>);
        };
        let item_after: ItemStruct = parse_quote! {
            #[derive(Eq, PartialEq, Clone, Debug)]
            pub struct MyMap(std::collections::HashMap<String, String>);
        };
        assert_item_equal_after_remove_impossible_derive(item_before, item_after)
    }

    fn assert_item_equal_after_remove_impossible_derive(item_before: ItemStruct, item_after: ItemStruct) -> Outcome {
        let root = get_temp_lib_root()?;
        let lib_rs = get_lib_rs_path(&root);
        fs::write(&lib_rs, item_before.to_token_stream().to_string())?;
        let lib_rs_utf8 = Utf8PathBuf::try_from(lib_rs.as_path())?;
        fix_impossible_derives(lib_rs_utf8.as_ref())?;
        let item = Item::from(item_after);
        let file_actual = fs::read_to_string(&lib_rs)?;
        let file_expected = unparse(&item.get::<File>());
        assert_eq!(file_actual, file_expected);
        Ok(())
    }
}
