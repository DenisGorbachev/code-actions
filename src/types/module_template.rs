use clap::ValueEnum;
use proc_macro2::{Ident, TokenStream};
use stub_macro::stub;
use ModuleTemplate::*;

use crate::generate_enum::{get_clap_enum_token_stream, get_plain_enum_token_stream, get_regular_enum_token_stream};
use crate::generate_error_enum::get_error_enum_token_stream;
use crate::generate_error_struct::get_error_struct_token_stream;
use crate::generate_fn::get_fn_token_stream;
use crate::generate_module::get_empty_module_token_stream;
use crate::generate_struct::{get_clap_struct_token_stream, get_sigil_struct_token_stream};
use crate::generate_struct::{get_regular_struct_token_stream, get_unit_struct_token_stream};
use crate::generate_trait::get_trait_token_stream;
use crate::generate_type_alias::get_type_alias_token_stream;
use crate::get_newtype_wrapper_struct_token_stream::get_newtype_wrapper_struct_token_stream;
use crate::get_subtype_struct_token_stream::get_subtype_struct_token_stream;
use crate::traits::to_module_token_stream::ToModuleTokenStream;
use crate::types::config::Config;

#[derive(ValueEnum, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum ModuleTemplate {
    #[default]
    Empty,
    RegularStruct,
    UnitStruct,
    NewtypeStruct,
    SubtypeStruct,
    SigilStruct,
    ClapStruct,
    ErrorStruct,
    RegularEnum,
    PlainEnum,
    ClapEnum,
    ErrorEnum,
    TypeAlias,
    Trait,
    Fn,
}

impl ModuleTemplate {
    pub fn function<'a>(&self, config: &'a Config) -> Box<dyn FnOnce(Ident) -> TokenStream + 'a> {
        match self {
            Empty => Box::new(get_empty_module_token_stream),
            RegularStruct => Box::new(move |ident| get_regular_struct_token_stream(ident, config)),
            UnitStruct => Box::new(move |ident| get_unit_struct_token_stream(ident, stub!(Vec<syn::UseTree>), stub!(Vec<&syn::Path>))),
            NewtypeStruct => Box::new(get_newtype_wrapper_struct_token_stream),
            SubtypeStruct => Box::new(get_subtype_struct_token_stream),
            SigilStruct => Box::new(move |ident| get_sigil_struct_token_stream(ident, config)),
            ClapStruct => Box::new(move |ident| get_clap_struct_token_stream(ident, config)),
            ErrorStruct => Box::new(move |ident| get_error_struct_token_stream(ident, config)),
            RegularEnum => Box::new(move |ident| get_regular_enum_token_stream(ident, config)),
            PlainEnum => Box::new(move |ident| get_plain_enum_token_stream(ident, config)),
            ClapEnum => Box::new(move |ident| get_clap_enum_token_stream(ident, config)),
            ErrorEnum => Box::new(move |ident| get_error_enum_token_stream(ident, config)),
            TypeAlias => Box::new(move |ident| get_type_alias_token_stream(ident, config)),
            Trait => Box::new(move |ident| get_trait_token_stream(ident, config)),
            Fn => Box::new(move |ident| get_fn_token_stream(ident, config)),
        }
    }

    pub fn to_module_token_stream_with_config(&self, ident: Ident, config: &Config) -> TokenStream {
        use crate::generate_enum::{get_clap_enum_token_stream_with_config, get_plain_enum_token_stream_with_config, get_regular_enum_token_stream_with_config};
        use crate::generate_error_enum::get_error_enum_token_stream_with_config;
        use crate::generate_error_struct::get_error_struct_token_stream_with_config;
        use crate::generate_fn::get_fn_token_stream_with_config;
        use crate::generate_struct::{get_clap_struct_token_stream_with_config, get_regular_struct_token_stream_with_config};
        use crate::generate_trait::get_trait_token_stream_with_config;
        use crate::generate_type_alias::get_type_alias_token_stream_with_config;

        let type_name = ident.to_string();

        match self {
            Empty => get_empty_module_token_stream(ident),
            RegularStruct => get_regular_struct_token_stream_with_config(ident, config),
            UnitStruct => get_unit_struct_token_stream(ident, stub!(Vec<syn::UseTree>), stub!(Vec<&syn::Path>)),
            NewtypeStruct => get_newtype_wrapper_struct_token_stream(ident),
            SubtypeStruct => get_subtype_struct_token_stream(ident),
            SigilStruct => get_unit_struct_token_stream(ident, stub!(Vec<syn::UseTree>), stub!(Vec<&syn::Path>)), // Uses unit struct
            ClapStruct => get_clap_struct_token_stream_with_config(ident, config),
            ErrorStruct => get_error_struct_token_stream_with_config(ident, config),
            RegularEnum => get_regular_enum_token_stream_with_config(ident, config, &type_name),
            PlainEnum => get_plain_enum_token_stream_with_config(ident, config, &type_name),
            ClapEnum => get_clap_enum_token_stream_with_config(ident, config, &type_name),
            ErrorEnum => get_error_enum_token_stream_with_config(ident, config),
            TypeAlias => get_type_alias_token_stream_with_config(ident, config),
            Trait => get_trait_token_stream_with_config(ident, config),
            Fn => get_fn_token_stream_with_config(ident, config),
        }
    }
}

impl ToModuleTokenStream for ModuleTemplate {
    fn to_module_token_stream(&self, ident: Ident) -> TokenStream {
        // Use default config for compatibility with existing ToModuleTokenStream trait
        let config = Config::default();
        self.function(&config)(ident)
    }
}
