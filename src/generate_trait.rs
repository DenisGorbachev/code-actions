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
    let content = get_trait_token_stream(name);
    Ok(format_token_stream_prettyplease(content)?)
}

pub fn get_trait_token_stream(trait_name: Ident) -> TokenStream {
    let config = Config::default();
    let type_name = trait_name.to_string();
    get_trait_token_stream_with_config(trait_name, &config, &type_name)
}

pub fn get_trait_token_stream_with_config(trait_name: Ident, config: &Config, type_name: &str) -> TokenStream {
    let method_name = trait_name.to_snake_case();
    let extra_uses = config.get_extra_use_statements_for_name(type_name);
    let extra_use_statements = create_use_statements(&extra_uses);

    quote! {
        #extra_use_statements

        pub trait #trait_name {
            type Output;

            fn #method_name(&self) -> Self::Output;
        }
    }
}

fn create_use_statements(use_statements: &[String]) -> TokenStream {
    if use_statements.is_empty() {
        return quote! {};
    }

    let mut tokens = TokenStream::new();
    for use_stmt in use_statements {
        let use_tokens = use_stmt
            .parse::<TokenStream>()
            .unwrap_or_else(|_| quote! { compile_error!(concat!("Invalid use statement: ", #use_stmt)); });
        tokens.extend(quote! { use #use_tokens; });
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::functions::label::to_ident;

    #[test]
    fn must_get_trait_token_stream() {
        let stream = get_trait_token_stream(to_ident("MyTrait"));
        let contents = format_token_stream_prettyplease(stream).unwrap();
        assert_eq!(contents, "pub trait MyTrait {\n    type Output;\n    fn my_trait(&self) -> Self::Output;\n}\n");
    }
}
