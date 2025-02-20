use anyhow::anyhow;
use time::OffsetDateTime;

use crate::types::outcome::Outcome;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::traits::cargo_info::CargoInfo;
use crate::utils::get_freewrite_file_name;

pub fn get_freewrite_path_from_anchor(now: OffsetDateTime, path: &Utf8Path) -> Outcome<Utf8PathBuf> {
    let package_root = path
        .find_package_root()
        .ok_or_else(|| anyhow!("Could not find package root"))?;
    let freewrite_filename = get_freewrite_file_name(now, "rs")?;
    let freewrite_path = package_root.join("src").join(freewrite_filename);
    Ok(freewrite_path)
}
