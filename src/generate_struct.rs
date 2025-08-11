use crate::types::outcome::Outcome;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use stub_macro::stub;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::std::path::file_stem::FileStem;
use crate::functions::code_generation_helpers::{create_derive_attribute_from_syn_path, create_use_statements_from_syn_use_tree};
use crate::functions::format::format_token_stream_prettyplease;
use crate::types::config::Config;
use crate::types::type_name::TypeName;

pub fn get_struct_file_contents(path: &Utf8Path, config: &Config) -> Outcome<String> {
    let stem = FileStem::try_from(path)?;
    let type_name = TypeName::from(*stem);
    let name = format_ident!("{}", &type_name);

    let content = get_regular_struct_token_stream_with_config(name, config);
    Ok(format_token_stream_prettyplease(content)?)
}

/// `Ord, PartialOrd` is useful for generic structs
pub fn get_regular_struct_token_stream(name: Ident, config: &Config) -> TokenStream {
    get_regular_struct_token_stream_with_config(name, config)
}

pub fn get_regular_struct_token_stream_with_config(name: Ident, config: &Config) -> TokenStream {
    let type_name = name.to_string();
    let extra_derives = config.get_extra_derives_for_name(&type_name);
    let derive_attr = create_derive_attribute_from_syn_path(extra_derives.iter());

    let extra_uses = config.get_extra_use_statements_for_name(&type_name);
    let extra_use_statements = create_use_statements_from_syn_use_tree(extra_uses);

    quote! {
        #extra_use_statements

        #derive_attr
        pub struct #name {}

        impl #name {}
    }
}

pub fn get_unit_struct_token_stream<'d>(name: Ident, use_statements: impl IntoIterator<Item = syn::UseTree>, derives: impl IntoIterator<Item = &'d syn::Path>) -> TokenStream {
    let use_statements_tokens = create_use_statements_from_syn_use_tree(use_statements);
    let derive_attr = create_derive_attribute_from_syn_path(derives);
    quote! {
        #use_statements_tokens

        #derive_attr
        pub struct #name;
    }
}

/// Currently equivalent to unit struct, but may change in the future
pub fn get_sigil_struct_token_stream(name: Ident, _config: &Config) -> TokenStream {
    // let _matches = stub!(());
    let use_statements = stub!(Vec<syn::UseTree>);
    let derives = stub!(Vec<&syn::Path>);
    get_unit_struct_token_stream(name, use_statements, derives)
}

pub fn get_clap_struct_token_stream(name: Ident, config: &Config) -> TokenStream {
    get_clap_struct_token_stream_with_config(name, config)
}

pub fn get_clap_struct_token_stream_with_config(name: Ident, config: &Config) -> TokenStream {
    let type_name = name.to_string();
    let extra_derives = config.get_extra_derives_for_name(&type_name);
    let derive_attr = create_derive_attribute_from_syn_path(extra_derives.iter());

    let extra_uses = config.get_extra_use_statements_for_name(&type_name);
    let extra_use_statements = create_use_statements_from_syn_use_tree(extra_uses);

    quote! {
        #extra_use_statements

        #derive_attr
        pub struct #name {}

        impl #name {
            pub async fn run(self, stdout: &mut impl Write, stderr: &mut impl Write) -> Result<(), ()> {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::config::Config;
    use anyhow::Context;
    use assert_matches::assert_matches;
    use derive_getters::{Dissolve, Getters};
    use derive_new::new;
    use fs_err::{create_dir_all, remove_file};

    use crate::assertions::assert_file_contains::assert_file_contains;
    use crate::types::outcome::Outcome;

    use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
    use crate::extensions::std::fs::create_file_all;
    use crate::extensions::tempfile::temp_dir::TempDir;
    use crate::generate_file::create_module_file_from_anchor_label;
    use crate::get_relative_path::get_relative_path_anchor_label_rs;
    use crate::test_helpers;
    use crate::test_helpers::get_src_path;
    use crate::types::anchor::Anchor;
    use crate::types::label::Label;
    use crate::types::module_template::ModuleTemplate;

    use ModuleTemplate::*;

    #[test]
    fn test_existing_struct_path() -> Outcome {
        let root = test_helpers::get_temp_bin_root()?;
        let (anchor, label) = get_struct_anchor_label_from_temp_dir(&root)?.dissolve();
        let path = get_relative_path_anchor_label_rs(anchor.as_ref(), label.as_ref())?;
        create_file_all(path.as_path())?;
        let config = Config::default();
        let result = create_module_file_from_anchor_label(anchor.as_ref(), label.as_ref(), RegularStruct, &config);
        assert_matches!(result, Err(ref err) if format!("{err:?}").contains("already exists"));
        Ok(())
    }

    #[test]
    fn test_existing_dir() -> Outcome {
        let (_root, anchor, label) = get_struct_path_buf()?.dissolve();
        create_dir_all(anchor.as_path())?;
        let config = Config::default();
        create_module_file_from_anchor_label(anchor.as_ref(), label.as_ref(), RegularStruct, &config)?;
        Ok(())
    }

    #[test]
    fn test_regular_case() -> Outcome {
        let chest = generate_struct()?;
        assert_file_contains(&chest.path()?, "Struct")?;
        assert_file_contains(&test_helpers::get_main_rs_path(chest.root()), "mod some;")?;
        assert_file_contains(&test_helpers::get_main_rs_path(chest.root()), "pub use some::*;")?;
        Ok(())
    }

    #[test]
    fn test_lib_case() -> Outcome {
        let (root, anchor, label) = get_struct_path_buf()
            .context("Could not get `path_buf`")?
            .dissolve();
        let path = get_relative_path_anchor_label_rs(anchor.as_ref(), label.as_ref())?;
        remove_file(test_helpers::get_main_rs_path(&root))?;
        test_helpers::create_lib_rs(&root)?;
        let config = Config::default();
        create_module_file_from_anchor_label(anchor.as_path(), label.as_str(), RegularStruct, &config)?;
        assert_file_contains(&path, "Struct")?;
        assert_file_contains(&test_helpers::get_lib_rs_path(&root), "mod some;")?;
        assert_file_contains(&test_helpers::get_lib_rs_path(&root), "pub use some::*;")?;
        Ok(())
    }

    fn get_struct_anchor_label_from_temp_dir(dir: &TempDir) -> Outcome<AnchorLabel> {
        let anchor = get_src_path(&dir).join("some/deep").try_into()?;
        let label = String::from("my_struct");
        Ok(AnchorLabel::new(anchor, label))
    }

    fn get_struct_path_buf() -> Outcome<RootAnchorLabel> {
        let root = test_helpers::get_temp_bin_root().context("Could not get `root`")?;
        get_struct_anchor_label_from_temp_dir(&root).map(|anchor_label| RootAnchorLabel::from_anchor_label(root, anchor_label))
    }

    fn generate_struct() -> Outcome<RootAnchorLabel> {
        let chest = get_struct_path_buf().context("Could not get `path_buf`")?;
        let config = Config::default();
        create_module_file_from_anchor_label(chest.anchor().as_path(), chest.label().as_ref(), RegularStruct, &config).context("create_struct error")?;
        Ok(chest)
    }

    #[derive(new, Getters, Dissolve, Eq, PartialEq, Clone, Debug)]
    pub struct AnchorLabel {
        anchor: Anchor,
        label: Label,
    }

    #[derive(new, Getters, Dissolve, Debug)]
    pub struct RootAnchorLabel {
        root: TempDir,
        anchor: Anchor,
        label: Label,
    }

    impl RootAnchorLabel {
        pub fn from_anchor_label(root: TempDir, anchor_label: AnchorLabel) -> Self {
            let (anchor, label) = anchor_label.dissolve();
            Self {
                root,
                anchor,
                label,
            }
        }

        pub fn path(&self) -> Outcome<Utf8PathBuf> {
            get_relative_path_anchor_label_rs(self.anchor.as_path(), self.label.as_str())
        }
    }
}
