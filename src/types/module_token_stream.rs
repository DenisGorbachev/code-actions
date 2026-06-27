use proc_macro2::TokenStream;
use subtype::subtype;

subtype!(
    #[derive(Clone, Debug)]
    pub struct ModuleTokenStream(TokenStream);
);
