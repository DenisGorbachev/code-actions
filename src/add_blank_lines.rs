use crate::extensions::camino::utf8_path::Utf8Path;
use crate::functions::format::format_cargo_fmt;
use crate::types::outcome::Outcome;
use crate::types::package_info::PackageInfo;
use quote::ToTokens;
use std::fs;
use syn::File;
use syn_more::SynFrom;

/// Insert blank lines between top-level items in a Rust source file.
///
/// `path` refers to a Rust source code file
/// `count` is the desired count of blank lines between items
///
/// Warning: this function does not preserve regular comments (starting with "//")
///
/// The resulting count of blank lines between items is always `count`. If there is more than `count` lines between items in the input `path`, it is normalized to exactly `count`.
/// The blank lines between non-top-level items are preserved.
/// The blank lines between sibling `use` and `mod` statements are removed (so that `use` and `mod` statements are grouped)
/// All other formatting is preserved.
/// A single newline at the end of file is added if not present.
pub fn insert_blank_lines(path: impl AsRef<Utf8Path>, count: usize) -> Outcome {
    let package_info = PackageInfo::try_from(path.as_ref())?;
    let file = File::syn_from(path.as_ref().as_std_path())?;
    let separator = get_separator(count);

    let mut output: Vec<String> = Vec::new();

    // Add shebang if present
    if let Some(shebang) = file.shebang {
        output.push(shebang);
    }
    output.extend(
        file.attrs
            .into_iter()
            .map(|attr| attr.into_token_stream().to_string()),
    );
    output.extend(
        file.items
            .into_iter()
            .map(|item| item.into_token_stream().to_string()),
    );

    // Convert the token stream back to a string
    let new_content = output.join(&separator);

    // Write the new content back to the file
    fs::write(path.as_ref(), new_content)?;

    // Format the file using rustfmt
    format_cargo_fmt(package_info.project_manifest().path())?;

    Ok(())
}

fn get_separator(count: usize) -> String {
    let mut separator = "\n".repeat(count);
    separator.push('\n');
    separator
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
    use crate::test_helpers::{get_src_path, get_temp_bin_root};
    use crate::types::outcome::Outcome;
    use indoc::indoc;
    use pretty_assertions::assert_str_eq;
    use std::fs;

    #[test]
    #[ignore]
    fn must_add_blank_lines() -> Outcome {
        let input = indoc! {"
            #!/bin/bash
            #![cfg(feature = \"my_feature\")]
            use subtype::subtype_string;
            use std::path::Path;
            fn filter(path: &Path) -> bool {
                fn internal_func() {}

                todo!()
            }
            subtype_string!(

                pub struct CommandName(String);

            );


            impl CommandName {}
        "};
        let output_expected = indoc! {"
            #!/bin/bash

            #![cfg(feature = \"my_feature\")]

            use subtype::subtype_string;
            use std::path::Path;

            fn filter(path: &Path) -> bool {
                fn internal_func() {}

                todo!()
            }

            subtype_string!(
                pub struct CommandName(String);
            );

            impl CommandName {}
        "};
        let output_actual = insert_blank_lines_in_temp_root(input)?;
        assert_str_eq!(output_expected, output_actual.as_str());
        Ok(())
    }

    fn insert_blank_lines_in_temp_root(input: &str) -> Outcome<String> {
        let root = get_temp_bin_root()?;
        let src = get_src_path(&root);
        let input_path = src.join("input.rs");
        let input_path_utf8 = Utf8PathBuf::try_from(input_path.as_path())?;
        fs::write(&input_path, input)?;
        insert_blank_lines(&input_path_utf8, 1)?;
        let output = fs::read_to_string(&input_path)?;
        Ok(output)
    }
}
