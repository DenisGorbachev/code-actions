use subtype::subtype;

subtype!(
    #[derive(Clone, Debug)]
    pub struct ModuleTokenStream(proc_macro2::TokenStream);
);
