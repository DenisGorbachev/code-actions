use anyhow::anyhow;
use heck::ToSnakeCase;

use crate::types::outcome::Outcome;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::extensions::std::string::ensure_suffix;
use crate::types::label::LabelSlice;

pub fn get_relative_path_anchor_filename(anchor: &Utf8Path, filename: &str) -> Outcome<Utf8PathBuf> {
    let dir = get_dir_from_anchor(anchor)?;
    let new_path = dir.join(filename);
    Ok(new_path)
}

pub fn get_relative_path_anchor_subdir_filename(anchor: &Utf8Path, subdir: &str, filename: &str) -> Outcome<Utf8PathBuf> {
    let dir = get_dir_from_anchor(anchor)?;
    let new_path = dir.join(subdir).join(filename);
    Ok(new_path)
}

pub fn get_relative_path_anchor_stem_extension(anchor: &Utf8Path, stem: &str, extension: &str) -> Outcome<Utf8PathBuf> {
    let filename = format!("{stem}.{extension}");
    get_relative_path_anchor_filename(anchor, &filename)
}

pub fn get_relative_path_anchor_stem_rs(anchor: &Utf8Path, stem: &str) -> Outcome<Utf8PathBuf> {
    get_relative_path_anchor_stem_extension(anchor, stem, "rs")
}

pub fn get_relative_path_anchor_label_rs(anchor: &Utf8Path, label: &LabelSlice) -> Outcome<Utf8PathBuf> {
    let stem = label.to_snake_case();
    get_relative_path_anchor_stem_extension(anchor, &stem, "rs")
}

pub fn get_relative_path_anchor_subdir_stem_rs(anchor: &Utf8Path, subdir: &str, stem: &str) -> Outcome<Utf8PathBuf> {
    let filename = format!("{stem}.rs");
    get_relative_path_anchor_subdir_filename(anchor, subdir, &filename)
}

pub fn get_relative_path_anchor_subdir_label_rs(anchor: &Utf8Path, subdir: &str, label: &LabelSlice) -> Outcome<Utf8PathBuf> {
    let filename = format!("{}.rs", label.to_snake_case());
    get_relative_path_anchor_subdir_filename(anchor, subdir, &filename)
}

pub fn get_relative_path_anchor_subdir_label(anchor: &Utf8Path, subdir: &str, label: &str) -> Outcome<Utf8PathBuf> {
    let stem = label.to_snake_case();
    get_relative_path_anchor_subdir_stem_rs(anchor, subdir, &stem)
}

pub fn get_relative_path_anchor_subdir_name_suffix(anchor: &Utf8Path, subdir: &str, name: &str, suffix: &str) -> Outcome<Utf8PathBuf> {
    let name = ensure_suffix(name, suffix);
    get_relative_path_anchor_subdir_label(anchor, subdir, &name)
}

pub fn get_dir_from_anchor(anchor: &Utf8Path) -> Outcome<Utf8PathBuf> {
    if anchor.is_dir() {
        Ok(anchor.to_path_buf())
    } else {
        let parent_dir = anchor
            .parent()
            .ok_or_else(|| anyhow!("Anchor has no parent directory"))?;
        let anchor_stem = anchor
            .file_stem()
            .ok_or_else(|| anyhow!("Anchor has no file stem"))?;
        Ok(parent_dir.join(anchor_stem))
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn must_get_relative_filename() -> Outcome {
        let parent: &Utf8Path = "/some/struct.rs".as_ref();
        let subdir = "errors";
        let stem = "NotFound";
        let suffix = "Error";
        let out = get_relative_path_anchor_subdir_name_suffix(parent, subdir, stem, suffix)?;
        assert_eq!(out, "/some/struct/errors/not_found_error.rs".into());
        Ok(())
    }

    #[test]
    fn must_get_dir_from_anchor_if_dir() -> Outcome {
        let anchor: &Utf8Path = "src/traits".as_ref();
        let result = get_dir_from_anchor(anchor);
        let dir = assert_matches!(result, Ok(dir) => dir);
        assert_eq!(dir, anchor.to_path_buf());
        Ok(())
    }

    #[test]
    fn must_get_dir_from_anchor_if_file() -> Outcome {
        let anchor: &Utf8Path = "src/traits/display.rs".as_ref();
        let result = get_dir_from_anchor(anchor);
        let dir = assert_matches!(result, Ok(dir) => dir);
        assert_eq!(dir, "src/traits/display".into());
        Ok(())
    }
}
