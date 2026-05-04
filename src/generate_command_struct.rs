use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn get_command_struct_token_stream(name: Ident) -> TokenStream {
    let label = format!("{name}RunError");
    let error_name = Ident::new(&label, Span::call_site());
    quote! {
        use clap::{Parser, value_parser};
        use errgonomic::handle;
        use std::path::PathBuf;
        use std::process::ExitCode;
        use thiserror::Error;
        use tokio::fs::read_to_string;

        #[derive(Parser, Clone, Debug)]
        pub struct #name {
            #[arg(short, long, value_parser = value_parser!(PathBuf))]
            path: PathBuf,
        }

        impl #name {
            pub async fn run(self) -> Result<ExitCode, #error_name> {
                use #error_name::*;
                let Self {
                    path,
                } = self;
                let contents = handle!(read_to_string(&path).await, ReadToStringFailed, path);
                println!("{contents}");
                Ok(ExitCode::SUCCESS)
            }
        }

        #[derive(Error, Debug)]
        pub enum #error_name {
            #[error("failed to read file at '{path}'")]
            ReadToStringFailed { source: std::io::Error, path: PathBuf },
        }
    }
}
