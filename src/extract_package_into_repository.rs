use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::types::outcome::Outcome;
use xshell::{cmd, Cmd, Shell};

pub fn extract_package_into_repository(sh: Shell, mut confirm: impl FnMut(&str) -> Result<bool, xshell::Error>, source: Utf8PathBuf, target: Utf8PathBuf) -> Outcome {
    let target_src = target.join("src");
    if target_src.try_exists()? {
        confirm_run(cmd!(sh, "rm -r {target_src}"), &mut confirm, |cmd| cmd.run_interactive())?;
    }
    confirm_run(cmd!(sh, "mv {source}/src"), &mut confirm, |cmd| cmd.run_interactive())?;
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

pub fn confirm_run<T: Default, E>(cmd: Cmd, confirm: impl FnOnce(&str) -> Result<bool, E>, run: impl FnOnce(Cmd) -> Result<T, E>) -> Result<T, E> {
    let should_run = confirm(&cmd.to_string())?;
    if should_run {
        run(cmd)
    } else {
        Ok(T::default())
    }
}
