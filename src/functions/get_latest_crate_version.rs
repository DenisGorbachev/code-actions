use derive_more::{Error, From};
use derive_new::new;
use fmt_derive::Display;

use crate::statics::CRATES_IO_CLIENT;
use crate::types::crates_io_api_error::CratesIoApiError;

pub fn get_crate_name_with_max_stable_version(name: &str) -> Result<(String, String), GetCrateNameWithMaxStableVersionError> {
    let info = CRATES_IO_CLIENT.get_crate(name)?;
    let krate = info.crate_data;
    let version = krate.max_stable_version.ok_or(StableVersionNotFoundError)?;
    Ok((krate.name, version))
}

#[derive(new, Error, Display, Eq, PartialEq, Hash, Clone, Debug)]
pub struct StableVersionNotFoundError;

#[derive(Error, From, Display, Debug)]
pub enum GetCrateNameWithMaxStableVersionError {
    TheCratesIoApiError(CratesIoApiError),
    TheStableVersionNotFoundError(StableVersionNotFoundError),
}
