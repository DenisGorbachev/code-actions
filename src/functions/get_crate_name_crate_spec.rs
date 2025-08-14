use crate::functions::get_latest_crate_version::{GetCrateNameWithMaxStableVersionError, get_crate_name_with_max_stable_version};
use crate::functions::parse_key_value::ParseKeyValueData;

pub fn get_crate_name_crate_spec(data: ParseKeyValueData) -> Result<(String, String), GetCrateNameWithMaxStableVersionError> {
    match data {
        ParseKeyValueData::TheKey(key) => {
            let (name, version) = get_crate_name_with_max_stable_version(key)?;
            Ok((name, version))
        }
        ParseKeyValueData::TheKeyValue((key, value)) => Ok((key.to_string(), value.to_string())),
    }
}
