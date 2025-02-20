use clap::ValueEnum;
use proc_macro2::{Ident, TokenStream};

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
    pub fn function(&self) -> impl FnOnce(Ident) -> TokenStream {
        match self {
            Empty => get_empty_module_token_stream,
            RegularStruct => get_regular_struct_token_stream,
            UnitStruct => get_unit_struct_token_stream,
            NewtypeStruct => get_newtype_wrapper_struct_token_stream,
            SubtypeStruct => get_subtype_struct_token_stream,
            SigilStruct => get_sigil_struct_token_stream,
            ClapStruct => get_clap_struct_token_stream,
            ErrorStruct => get_error_struct_token_stream,
            RegularEnum => get_regular_enum_token_stream,
            PlainEnum => get_plain_enum_token_stream,
            ClapEnum => get_clap_enum_token_stream,
            ErrorEnum => get_error_enum_token_stream,
            TypeAlias => get_type_alias_token_stream,
            Trait => get_trait_token_stream,
            Fn => get_fn_token_stream,
        }
    }
}

impl ToModuleTokenStream for ModuleTemplate {
    fn to_module_token_stream(&self, ident: Ident) -> TokenStream {
        self.function()(ident)
    }
}
