use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::types::outcome::Outcome;
use derive_more::Error;
use std::io;
use xshell::{cmd, Cmd, Shell};

pub fn extract_package_into_repository(sh: Shell, source: Utf8PathBuf, target: Utf8PathBuf) -> Outcome {
    let target_src = target.join("src");
    if target_src.try_exists()? {
        confirm_run(cmd!(sh, "rm -r {target_src}"), confirm, |cmd| cmd.run_interactive())?;
    }
    confirm_run(cmd!(sh, "mv {source}/src"), confirm, |cmd| cmd.run_interactive())?;
    let tasks = [
        "Move dependencies",
        "Remove package from workspace.members in Cargo.toml",
        &format!("ff {target}"),
        &format!("ff {source}"),
    ];
    for task in tasks {
        confirm(task)?;
    }
    Ok(())
}

pub fn confirm_run<T: Default, ConfirmErr, RunErr>(cmd: Cmd, confirm: impl FnOnce(&str) -> Result<bool, ConfirmErr>, run: impl FnOnce(Cmd) -> Result<T, RunErr>) -> Result<T, ConfirmRunError<ConfirmErr, RunErr>> {
    let should_run = confirm(&cmd.to_string()).map_err(ConfirmRunError::Confirm)?;
    if should_run {
        run(cmd).map_err(ConfirmRunError::Run)
    } else {
        Ok(T::default())
    }
}

use dialoguer::Confirm;
use fmt_derive::Display;

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
