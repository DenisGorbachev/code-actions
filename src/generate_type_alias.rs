use crate::functions::code_generation_helpers::create_use_statements;
use crate::types::config::Config;
use fs_err::File;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::types::outcome::Outcome;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::std::path::file_stem::FileStem;
use crate::functions::format::format_token_stream_prettyplease;
use crate::generate_file::generate_module_file;
use crate::get_relative_path::get_relative_path_anchor_label_rs;
use crate::types::label::LabelSlice;
use crate::types::type_name::TypeName;

pub fn generate_type_alias_from_path(path: impl AsRef<Utf8Path>) -> Outcome<File> {
    generate_module_file(path, get_type_alias_file_contents)
}

pub fn generate_type_alias_from_anchor_label(anchor: &Utf8Path, label: &LabelSlice) -> Outcome<File> {
    let path = get_relative_path_anchor_label_rs(anchor, label)?;
    generate_type_alias_from_path(path)
}

pub fn get_type_alias_file_contents(path: &Utf8Path) -> Outcome<String> {
    let stem = FileStem::try_from(path)?;
    let type_name = TypeName::from(*stem);
    let name = format_ident!("{}", &type_name);
    let config = Config::try_from(path.as_std_path())?;
    let content = get_type_alias_token_stream(name, &config);
    Ok(format_token_stream_prettyplease(content)?)
}

pub fn get_type_alias_token_stream(name: Ident, config: &Config) -> TokenStream {
    get_type_alias_token_stream_with_config(name, config)
}

pub fn get_type_alias_token_stream_with_config(name: Ident, config: &Config) -> TokenStream {
    let type_name = name.to_string();
    let extra_uses = config.get_extra_use_statements_for_name(&type_name);
    let extra_use_statements = create_use_statements(&extra_uses);

    quote! {
        #extra_use_statements

        pub type #name = ();
    }
}
