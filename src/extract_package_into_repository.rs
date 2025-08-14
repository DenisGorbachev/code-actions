use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::types::outcome::Outcome;
use anyhow::Context;
use derive_more::Error;
use std::io;
use xshell::{Cmd, Shell, cmd};

pub fn extract_package_into_repository(sh: Shell, source: Utf8PathBuf, target: Utf8PathBuf) -> Outcome {
    let source_src = source.join("src");
    let target_src = target.join("src");
    let target_info = PackageInfo::try_from(target.as_path())?;
    let target_name = target_info
        .name()
        .ok_or_not_found::<String>()
        .context(format!("Could not find package name in {target}"))?;
    let source_workspace_root = source.as_path().get_workspace_root()?;
    if target_src.try_exists()? {
        confirm_run(cmd!(sh, "rm -r {target_src}"), confirm, |cmd| cmd.run_interactive())?;
    }
    if source_src.try_exists()? {
        confirm_run(cmd!(sh, "mv {source_src} {target_src}"), confirm, |cmd| cmd.run_interactive())?;
    } else {
        println!("{source_src} does not exist; skipping");
    }
    let tasks = [
        "Move dependencies",
        "Remove package from workspace.members in Cargo.toml",
        &format!("ff {target}"),
        &format!("Add \"{target_name}\" dependency to {source_workspace_root}"),
        &format!("Fix the code in {source_workspace_root}"),
        &format!("ff {source_workspace_root}"),
    ];
    for task in tasks {
        confirm(task)?;
    }
    Ok(())
}

pub fn confirm_run<T: Default, ConfirmErr, RunErr>(cmd: Cmd, confirm: impl FnOnce(&str) -> Result<bool, ConfirmErr>, run: impl FnOnce(Cmd) -> Result<T, RunErr>) -> Result<T, ConfirmRunError<ConfirmErr, RunErr>> {
    let should_run = confirm(&cmd.to_string()).map_err(ConfirmRunError::Confirm)?;
    if should_run { run(cmd).map_err(ConfirmRunError::Run) } else { Ok(T::default()) }
}

use crate::traits::cargo_info::CargoInfo;
use crate::types::package_info::PackageInfo;
use dialoguer::Confirm;
use fmt_derive::Display;
use not_found_error::OkOrNotFound;

pub fn confirm(prompt: &str) -> io::Result<bool> {
    Confirm::new()
        .with_prompt(prompt)
        .interact()
        .map_err(to_io_error)
}

#[derive(Error, Display, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum ConfirmRunError<ConfirmErr, RunErr> {
    Confirm(ConfirmErr),
    Run(RunErr),
}

pub fn to_io_error(error: dialoguer::Error) -> io::Error {
    match error {
        dialoguer::Error::IO(out) => out,
    }
}
