use time::OffsetDateTime;

use crate::types::outcome::Outcome;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::std::fs::write_all_to_file_if_not_exists;
use crate::get_relative_path::get_relative_path_anchor_stem_extension;
use crate::utils::{get_freewrite_file_content, get_freewrite_file_stem};

pub fn generate_freewrite_file_from_anchor(anchor: &Utf8Path) -> Outcome {
    let now = OffsetDateTime::now_utc();
    let stem = get_freewrite_file_stem(now)?;
    let freewrite_path = get_relative_path_anchor_stem_extension(anchor, &stem, "md")?;
    let freewrite_content = get_freewrite_file_content(now)?;
    write_all_to_file_if_not_exists(freewrite_path, freewrite_content)?;
    Ok(())
}
