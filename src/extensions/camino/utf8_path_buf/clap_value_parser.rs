use clap::error::ErrorKind;

use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;

impl clap::builder::ValueParserFactory for Utf8PathBuf {
    type Parser = Utf8PathBufValueParser;

    fn value_parser() -> Self::Parser {
        Utf8PathBufValueParser
    }
}

#[derive(Clone, Debug)]
pub struct Utf8PathBufValueParser;

impl clap::builder::TypedValueParser for Utf8PathBufValueParser {
    type Value = Utf8PathBuf;

    fn parse_ref(&self, cmd: &clap::Command, _arg: Option<&clap::Arg>, value: &std::ffi::OsStr) -> Result<Self::Value, clap::Error> {
        let result = value.try_into();
        result.map_err(|_err| clap::Error::new(ErrorKind::InvalidUtf8).with_cmd(cmd))
    }
}
