use crate::functions::code_generation_helpers::create_use_statements_from_syn_use_tree;
use crate::types::config::Config;
use crate::types::outcome::Outcome;
use fs_err::File;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::std::path::file_stem::FileStem;
use crate::extensions::syn::IdentExt;
use crate::functions::format::format_token_stream_prettyplease;
use crate::generate_file::generate_module_file;
use crate::get_relative_path::get_relative_path_anchor_label_rs;
use crate::types::type_name::TypeName;

pub fn generate_trait_from_anchor_label(anchor: &Utf8Path, label: &str) -> Outcome<File> {
    let path = get_relative_path_anchor_label_rs(anchor, label)?;
    generate_trait_from_path(path)
}

pub fn generate_trait_from_path(path: impl AsRef<Utf8Path>) -> Outcome<File> {
    generate_module_file(path, get_trait_file_contents)
}

pub fn get_trait_file_contents(path: &Utf8Path) -> Outcome<String> {
    let stem = FileStem::try_from(path)?;
    let type_name = TypeName::from(*stem);
    let name = format_ident!("{}", &type_name);
    let config = Config::try_from(path.as_std_path())?;
    let content = get_trait_token_stream(name, &config);
    Ok(format_token_stream_prettyplease(content)?)
}

pub fn get_trait_token_stream(trait_name: Ident, config: &Config) -> TokenStream {
    get_trait_token_stream_with_config(trait_name, config)
}

pub fn get_trait_token_stream_with_config(trait_name: Ident, config: &Config) -> TokenStream {
    let type_name = trait_name.to_string();
    let method_name = trait_name.to_snake_case();
    let extra_uses = config.get_extra_use_statements_for_name(&type_name);
    let extra_use_statements = create_use_statements_from_syn_use_tree(extra_uses);

    quote! {
        #extra_use_statements

        pub trait #trait_name {
            type Output;

            fn #method_name(&self) -> Self::Output;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::functions::label::to_ident;

    #[test]
    fn must_get_trait_token_stream() {
        let config = Config::default();
        let stream = get_trait_token_stream(to_ident("MyTrait"), &config);
        let contents = format_token_stream_prettyplease(stream).unwrap();
        assert_eq!(contents, "pub trait MyTrait {\n    type Output;\n    fn my_trait(&self) -> Self::Output;\n}\n");
    }
}
