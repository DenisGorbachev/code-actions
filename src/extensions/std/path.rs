use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use anyhow::Context;
use rustc_hash::FxHashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use walkdir::WalkDir;

use crate::types::outcome::Outcome;

pub mod file_stem;

pub fn get_file_stem(path: &Path) -> Outcome<&OsStr> {
    path.file_stem()
        .context("Path should have a valid file stem")
}

pub fn get_file_stem_str(path: &Path) -> Outcome<&str> {
    let stem = get_file_stem(path)?;
    stem.to_str()
        .context("File stem should be convertible to str")
}

pub fn has_duplicate_lines<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut lines_seen = FxHashSet::default();

    for line in reader.lines() {
        let line = line?;
        if !lines_seen.insert(line) {
            // A duplicate line was found.
            return Ok(true);
        }
    }

    // No duplicates found.
    Ok(false)
}

// This function finds the first file with duplicate lines within a directory.
pub fn file_with_duplicate_lines<P: AsRef<Path>>(root: P) -> Option<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok()) // Filter out any errors during directory traversal.
        .filter(|e| e.file_type().is_file()) // Ensure we're looking at files only.
        .find_map(|entry| {
            let path = entry.path();
            match has_duplicate_lines(path) {
                Ok(true) => Some(path.to_path_buf()), // Return the path if it has duplicate lines.
                _ => None,                            // Skip files without duplicate lines or upon encountering an error.
            }
        })
}
