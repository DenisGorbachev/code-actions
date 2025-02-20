use derive_more::Error;
use fmt_derive::Display;

#[derive(Error, Display, Debug)]
pub struct TryFromTempDirForUtf8PathError;

// impl TryFrom<&TempDir> for Utf8Path {
//     type Error = TryFromTempDirForUtf8PathError;
//
//     fn try_from(value: &TempDir) -> Result<Self, TryFromTempDirForUtf8PathError> {
//         let path = value.path();
//         let option = camino::Utf8Path::from_path(path);
//         Utf8Path(option)
//     }
// }
