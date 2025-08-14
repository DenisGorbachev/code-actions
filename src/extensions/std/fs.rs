use std::fs::{File, OpenOptions, create_dir_all, read_to_string, write};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

use derive_more::Error;
use fmt_derive::Display;

pub mod file_ext;

#[derive(Error, Display, Debug)]
pub enum CreateFileAllError {
    PathParent { path: PathBuf },
    CreateDirAll { source: io::Error, path: PathBuf },
    CreateFile { source: io::Error, path: PathBuf },
}

/// TODO: handle the case where path is root (currently it returns PathParent error)
pub fn create_file_all<P: AsRef<Path>>(path_like: P) -> Result<File, CreateFileAllError> {
    let path = path_like.as_ref();
    let parent = path
        .parent()
        .ok_or_else(|| CreateFileAllError::PathParent {
            path: path.to_owned(),
        })?;
    create_dir_all(parent).map_err(|source| CreateFileAllError::CreateDirAll {
        source,
        path: parent.to_owned(),
    })?;
    File::create(path).map_err(|source| CreateFileAllError::CreateFile {
        source,
        path: path.to_owned(),
    })
}

pub fn modify_file_contents<InnerError, OuterError, Modify>(path: impl AsRef<Path>, modify: Modify) -> Result<(), OuterError>
where
    Modify: FnOnce(String) -> Result<String, InnerError>,
    OuterError: From<std::io::Error> + From<InnerError>,
{
    let contents_old = read_to_string(path.as_ref())?;
    let contents_new = modify(contents_old)?;
    write(path, contents_new).map_err(From::from)
}

pub fn truncate<P: AsRef<Path>>(path: P) -> io::Result<()> {
    OpenOptions::new().write(true).open(path)?.set_len(0)
}

pub fn write_all_to_file_if_not_exists<P: AsRef<Path>, B: AsRef<[u8]>>(filename: P, buf: B) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true) // This option ensures the file must not already exist
        .open(filename)?;

    file.write_all(buf.as_ref())
}

pub fn find_replace_all<'a, 'b>(path: impl AsRef<Path>, iter: impl IntoIterator<Item = (&'a str, &'b str)>) -> io::Result<()> {
    let mut contents = fs_err::read_to_string(path.as_ref())?;
    for (from, to) in iter.into_iter() {
        contents = contents.replace(from, to);
    }
    fs_err::write(path.as_ref(), contents)?;
    Ok(())
}
