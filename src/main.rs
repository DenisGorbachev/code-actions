use clap::{value_parser, Parser, Subcommand};
use stub_macro::stub;
use time::OffsetDateTime;

use crate::types::outcome::Outcome;
use types::module_template::ModuleTemplate;

use crate::add_dependency::{add_global_dependency_from_version, add_local_dependency_for_package_from_name, remove_workspace_and_package_dependency};
use crate::clean_external_path_deps::clean_external_path_deps;
use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::fix_name::fix_name;
use crate::functions::get_impl_file_contents::generate_impl_from_anchor_trait_path;
use crate::generate_file::{append_to_module_file_from_path, create_module_file_from_anchor_label, get_module_file_from_label};
use crate::generate_freewrite_file_from_anchor::generate_freewrite_file_from_anchor;
use crate::generate_module::{generate_module_from_anchor_subdir_label, generate_module_from_path};
use crate::generate_package_from_anchor_name::generate_package_from_anchor_name;
use crate::get_freewrite_path_from_anchor_path::get_freewrite_path_from_anchor;
use crate::get_relative_path::get_relative_path_anchor_subdir_name_suffix;
use crate::remove_module_by_path::remove_module_by_path;
use crate::types::label::Label;

pub mod add_dependency;
pub mod constants;
pub mod experiment;
pub mod extensions;
pub mod functions;
pub mod generate_enum;
pub mod generate_error_enum;
pub mod generate_error_struct;
pub mod generate_file;
pub mod generate_impl_from;
pub mod generate_module;
pub mod generate_modules;
pub mod generate_struct;
pub mod get_freewrite_path_from_anchor_path;
pub mod get_newtype_wrapper_struct_token_stream;
pub mod get_relative_path;
pub mod join_blocks;
pub mod primary_module;
pub mod test_helpers;
pub mod tests;
pub mod traits;
pub mod types;
pub mod utils;

#[derive(Parser)]
#[command(
    version,
    author = "Denis Gorbachev",
    about = "Utilities for writing Rust code quickly",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn run(self) -> Outcome {
        use Command::*;
        match self.command {
            AddDependency {
                command,
            } => {
                use AddDependencyCommand::*;
                match command {
                    Global {
                        anchor,
                        crate_version,
                        optional,
                    } => add_global_dependency_from_version(anchor.as_ref(), &crate_version, optional),
                    Local {
                        anchor,
                        crate_name,
                    } => add_local_dependency_for_package_from_name(anchor.as_ref(), &crate_name),
                }
            }
            RemoveDependency {
                anchor,
                crate_name,
            } => remove_workspace_and_package_dependency(anchor.as_ref(), &crate_name),
            Generate {
                command,
            } => {
                use GenerateCommand::*;
                match command {
                    Package {
                        anchor,
                        name,
                        args: _args,
                    } => generate_package_from_anchor_name(anchor.as_ref(), &name, stub!()),
                    ModuleFromPath {
                        path,
                    } => generate_module_from_path(path).discard(),
                    ModuleFromAnchorSubdirLabel {
                        anchor,
                        subdir,
                        label,
                    } => generate_module_from_anchor_subdir_label(anchor.as_ref(), &subdir, &label).discard(),
                    ModuleFromAnchorLabel {
                        anchor,
                        label,
                        template,
                    } => create_module_file_from_anchor_label(anchor.as_ref(), &label, template).discard(),
                    ImplFromAnchorTraitPath {
                        anchor,
                        trait_path,
                    } => generate_impl_from_anchor_trait_path(anchor.as_ref(), &trait_path).discard(),
                    FreewriteFileFromAnchor {
                        anchor,
                    } => generate_freewrite_file_from_anchor(anchor.as_ref()),
                }
            }
            Append {
                command,
            } => {
                use AppendCommand::*;
                match command {
                    ModuleFromPath {
                        path,
                        template,
                    } => append_to_module_file_from_path(path.as_ref(), template).discard(),
                }
            }
            Remove {
                command,
            } => {
                use RemoveCommand::*;
                match command {
                    ModuleByPath {
                        path,
                    } => remove_module_by_path(path.as_path()),
                }
            }
            FixName {
                path,
            } => fix_name(path.as_ref()),
            RemoveImpossibleDerives {
                anchor,
            } => remove_impossible_derives(anchor.as_ref()),
            CleanExternalPathDeps {
                yes,
                anchor,
            } => clean_external_path_deps(anchor.as_ref(), !yes),
            FixImports {
                yes,
                anchor,
            } => fix_imports(anchor.as_ref(), yes),
            Print {
                command,
            } => {
                use PrintCommand::*;
                match command {
                    Module {
                        label,
                        template,
                    } => {
                        let module = get_module_file_from_label(&label, template)?;
                        println!("{module}");
                        Ok(())
                    }
                    RelativePath {
                        parent,
                        subdir,
                        stem,
                        suffix,
                    } => {
                        let suffix = suffix.unwrap_or_default();
                        let path = Utf8Path::new(camino::Utf8Path::new(&parent));
                        let filename = get_relative_path_anchor_subdir_name_suffix(path, &subdir, &stem, &suffix)?;
                        println!("{}", filename);
                        Ok(())
                    }
                    FreewritePath {
                        anchor,
                    } => {
                        let now = OffsetDateTime::now_utc();
                        let freewrite_path = get_freewrite_path_from_anchor(now, anchor.as_ref())?;
                        println!("{}", freewrite_path);
                        Ok(())
                    }
                }
            }
        }
    }
}

#[derive(Subcommand)]
enum Command {
    AddDependency {
        #[command(subcommand)]
        command: AddDependencyCommand,
    },
    RemoveDependency {
        #[arg(value_name = "FILE", value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
        crate_name: String,
    },
    Generate {
        #[command(subcommand)]
        command: GenerateCommand,
    },
    Append {
        #[command(subcommand)]
        command: AppendCommand,
    },
    Remove {
        #[command(subcommand)]
        command: RemoveCommand,
    },
    FixName {
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        path: Utf8PathBuf,
    },
    RemoveImpossibleDerives {
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
    },
    CleanExternalPathDeps {
        #[arg(long)]
        yes: bool,
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
    },
    FixImports {
        #[arg(long)]
        yes: bool,
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
    },
    Print {
        #[command(subcommand)]
        command: PrintCommand,
    },
}

#[derive(Subcommand)]
enum AddDependencyCommand {
    Global {
        #[arg(long, short)]
        optional: bool,
        #[arg(value_name = "FILE", value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
        crate_version: String,
    },
    Local {
        #[arg(value_name = "FILE", value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
        crate_name: String,
    },
}

#[derive(Subcommand)]
enum GenerateCommand {
    Package {
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
        name: String,
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    ModuleFromPath {
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        path: Utf8PathBuf,
    },
    ModuleFromAnchorSubdirLabel {
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
        subdir: String,
        label: String,
    },
    ModuleFromAnchorLabel {
        template: ModuleTemplate,
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
        label: Label,
    },
    ImplFromAnchorTraitPath {
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
        trait_path: String,
    },
    FreewriteFileFromAnchor {
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
    },
}

#[derive(Subcommand)]
enum AppendCommand {
    ModuleFromPath {
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        path: Utf8PathBuf,
        #[arg(default_value_t, value_enum)]
        template: ModuleTemplate,
    },
}

#[derive(Subcommand)]
enum RemoveCommand {
    ModuleByPath {
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        path: Utf8PathBuf,
    },
}

#[derive(Subcommand)]
enum PrintCommand {
    Module {
        label: Label,
        #[arg(default_value_t, value_enum)]
        template: ModuleTemplate,
    },
    RelativePath {
        parent: String,
        subdir: String,
        stem: String,
        #[arg(short, long)]
        suffix: Option<String>,
    },
    FreewritePath {
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
    },
}

fn main() -> Outcome {
    // TODO: Use [Mason](https://pub.dev/packages/mason_cli) to generate files from templates
    init_tracing_subscriber();
    Cli::parse().run()
}

pub fn init_tracing_subscriber() {
    use tracing::level_filters::LevelFilter;
    use tracing_error::ErrorLayer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    let env_filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        // .with_max_level(tracing::Level::TRACE) // Set the maximum log level to TRACE
        .finish()
        .with(ErrorLayer::default());
    // dbg!(&subscriber);
    subscriber.init();
}

pub mod clean_external_path_deps;
pub mod extract_package_to_repository;
mod fix_imports;
pub mod fix_name;
pub mod generate_fn;
pub mod generate_freewrite_file_from_anchor;
pub mod generate_package_from_anchor_name;
pub mod generate_trait;
pub mod generate_type_alias;
pub mod get_subtype_struct_token_stream;
pub mod remove_module_by_path;
pub mod statics;

use crate::remove_impossible_derives::remove_impossible_derives;
use crate::traits::discard::Discard;
pub use fix_imports::*;

#[allow(dead_code)]
mod add_blank_lines;
#[cfg(test)]
mod assertions;
pub mod remove_impossible_derives;
