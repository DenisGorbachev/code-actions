use std::fmt::Debug;
use std::path::Path;
use std::{fs, io};

pub fn assert_file_contains<P: AsRef<Path> + Debug>(path: &P, pattern: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    assert!(contents.contains(pattern), "File {:?} does not contain {:?}", path, pattern);
    Ok(())
}
