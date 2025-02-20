use heck::ToSnakeCase;
use syn::Ident;

pub trait IdentExt: Sized {
    fn clone_with_name<F: FnOnce(String) -> String>(&self, f: F) -> Self;

    fn to_snake_case(&self) -> Self {
        self.clone_with_name(|name| name.to_snake_case())
    }
}

impl IdentExt for Ident {
    fn clone_with_name<F: FnOnce(String) -> String>(&self, f: F) -> Self {
        let name = f(self.to_string());
        let span = self.span();
        Ident::new(&name, span)
    }
}
