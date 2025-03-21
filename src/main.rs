use clap::{value_parser, Parser, Subcommand};
use stub_macro::stub;
use time::OffsetDateTime;

use code_actions::types::module_template::ModuleTemplate;
use code_actions::types::outcome::Outcome;

use code_actions::add_dependency::{add_global_dependency_from_version, add_local_dependency_for_package_from_name, remove_workspace_and_package_dependency};
use code_actions::clean_external_path_deps::clean_external_path_deps;
use code_actions::extensions::camino::utf8_path::Utf8Path;
use code_actions::extensions::camino::utf8_path_buf::Utf8PathBuf;
use code_actions::fix_imports;
use code_actions::fix_impossible_derives::fix_impossible_derives;
use code_actions::fix_name::fix_name;
use code_actions::functions::get_impl_file_contents::generate_impl_from_anchor_trait_path;
use code_actions::generate_file::{append_to_module_file_from_path, create_module_file_from_anchor_label, get_module_file_from_label};
use code_actions::generate_freewrite_file_from_anchor::generate_freewrite_file_from_anchor;
use code_actions::generate_module::{generate_module_from_anchor_subdir_label, generate_module_from_path};
use code_actions::generate_package_from_anchor_name::generate_package_from_anchor_name;
use code_actions::get_freewrite_path_from_anchor_path::get_freewrite_path_from_anchor;
use code_actions::get_relative_path::get_relative_path_anchor_subdir_name_suffix;
use code_actions::remove_module_by_path::remove_module_by_path;
use code_actions::traits::discard::Discard;
use code_actions::types::label::Label;

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
                anchor,
            } => fix_name(anchor.as_ref()),
            FixImpossibleDerives {
                anchor,
            } => fix_impossible_derives(anchor.as_ref()),
            FixMulti {
                anchor,
            } => {
                // Run `fix_impossible_derives` first, because fix_name would change the file name
                fix_impossible_derives(anchor.as_ref())?;
                fix_name(anchor.as_ref())?;
                Ok(())
            }
            FixImports {
                yes,
                anchor,
            } => fix_imports(anchor.as_ref(), yes),
            CleanExternalPathDeps {
                yes,
                anchor,
            } => clean_external_path_deps(anchor.as_ref(), !yes),
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
        anchor: Utf8PathBuf,
    },
    FixImpossibleDerives {
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
    },
    /// Fix name and impossible derives
    FixMulti {
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
    },
    FixImports {
        #[arg(long)]
        yes: bool,
        #[arg(value_parser = value_parser!(Utf8PathBuf))]
        anchor: Utf8PathBuf,
    },
    CleanExternalPathDeps {
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
