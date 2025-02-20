use std::io::{Read, Result, Seek, SeekFrom, Write};

use fs_err::File;
use itertools::Itertools;

pub trait FileExt {
    fn contains<S: AsRef<str>>(&mut self, s: S) -> Result<bool>;
    fn append_if_not_contains<S: AsRef<str>>(&mut self, lines: &[S]) -> Result<()>;
}

impl FileExt for File {
    fn contains<S: AsRef<str>>(&mut self, str: S) -> Result<bool> {
        let mut contents = String::new();
        self.read_to_string(&mut contents)?;
        Ok(contents.contains(str.as_ref()))
    }

    fn append_if_not_contains<S: AsRef<str>>(&mut self, new_lines: &[S]) -> Result<()> {
        self.seek(SeekFrom::Start(0))?;
        let mut string = String::new();
        self.read_to_string(&mut string)?;
        let existing_lines = string.lines().collect_vec();
        self.seek(SeekFrom::End(0))?;
        for line in new_lines {
            let line = line.as_ref();
            let line_exists = existing_lines
                .iter()
                .any(|existing| existing.trim() == line.trim());
            if !line_exists {
                writeln!(self, "{}", line)?;
            }
        }
        Ok(())
    }
}
