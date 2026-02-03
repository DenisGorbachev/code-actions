use crate::types::outcome::Outcome;
use anyhow::Context;
use camino::{Utf8Path, Utf8PathBuf};
use std::io::BufRead;
use std::io::Write;
use std::{fs, io};

pub fn remove_module_by_path(path: &Utf8Path) -> Outcome {
    // Extract the file stem
    let file_stem = path.file_stem().context("Failed to get file stem")?;

    // Remove the module file
    fs::remove_file(path).with_context(|| format!("Failed to remove module file: {path}"))?;

    // Get the parent directory of the module file
    let parent_dir = path
        .parent()
        .context("Failed to get parent directory of the module file")?;

    // Find the parent module file (lib.rs or mod.rs in the parent directory)
    let parent_module_file = find_parent_module_file(parent_dir)?;

    // Remove the "mod" or "pub mod" line from the parent module file
    remove_mod_line_from_parent(&parent_module_file, file_stem)?;

    Ok(())
}

fn find_parent_module_file(parent_dir: &Utf8Path) -> Outcome<Utf8PathBuf> {
    let potential_files = vec![
        parent_dir.join("lib.rs"),
        parent_dir.join("mod.rs"),
        parent_dir.with_extension("rs"),
    ];

    for file in potential_files {
        if file.exists() {
            return Ok(file);
        }
    }

    Err(anyhow::anyhow!("Parent module file not found"))
}

fn remove_mod_line_from_parent(parent_module_file: &Utf8Path, file_stem: &str) -> Outcome {
    let file = fs::File::open(parent_module_file).with_context(|| format!("Failed to open parent module file: {parent_module_file}"))?;
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !is_filtered(&line, file_stem) {
            lines.push(line);
        }
    }

    let mut file = fs::File::create(parent_module_file).with_context(|| format!("Failed to write to parent module file: {parent_module_file}"))?;
    for line in lines {
        writeln!(file, "{line}")?;
    }

    Ok(())
}

pub fn is_filtered(line: &str, str: &str) -> bool {
    let is_filtered_mod = is_mod(line) && line.trim_end_matches(";").ends_with(str);
    let is_filtered_use = is_use(line)
        && line
            .trim_end_matches(";")
            .trim_end_matches("::*")
            .ends_with(str);
    is_filtered_mod || is_filtered_use
}

pub fn is_mod(line: &str) -> bool {
    starts_with_trimmed(line, "mod")
}

pub fn is_use(line: &str) -> bool {
    starts_with_trimmed(line, "use")
}

pub fn starts_with_trimmed(line: &str, start: &str) -> bool {
    line.trim() // trim optional whitespace
        .trim_start_matches("pub")
        .trim() // trim optional whitespace
        .trim_start_matches("(crate)")
        .trim() // trim optional whitespace
        .starts_with(start)
}
