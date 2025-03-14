use cargo_metadata::{CompilerMessage, Message};
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn get_clippy_messages(workspace_root: impl AsRef<Path>) -> io::Result<impl Iterator<Item = io::Result<Message>>> {
    let mut cmd = Command::new("cargo");
    cmd.args(["clippy", "--message-format=json"]);
    cmd.current_dir(&workspace_root);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::null());

    // Spawn the command
    let child = cmd.spawn()?;

    // Get the stdout of the child process
    let stdout = child
        .stdout
        .ok_or_else(|| io::Error::other("Failed to get child stdout"))?;

    dbg!(&stdout);
    // Create a BufReader
    let reader = BufReader::new(stdout);

    // Return an iterator that processes each line
    let iter = reader
        .lines()
        // .inspect(|line_result| {
        //     dbg!(&line_result);
        // })
        .map(move |line_result| line_result.and_then(|line| serde_json::from_str(&line).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))));

    Ok(iter)
}

pub fn get_clippy_compiler_messages(workspace_root: impl AsRef<Path>) -> io::Result<impl Iterator<Item = CompilerMessage>> {
    let messages = get_clippy_messages(workspace_root)?;
    let compiler_messages = messages.filter_map(|message_result| {
        message_result.ok().and_then(|message| match message {
            Message::CompilerMessage(compiler_message) => Some(compiler_message),
            _ => None,
        })
    });
    Ok(compiler_messages)
}
