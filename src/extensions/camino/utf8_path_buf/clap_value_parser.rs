use std::ffi::OsStr;

use clap::builder::{TypedValueParser, ValueParserFactory};
use clap::error::ErrorKind;
use clap::{Arg, Command, Error};

use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;

impl ValueParserFactory for Utf8PathBuf {
    type Parser = Utf8PathBufValueParser;

    fn value_parser() -> Self::Parser {
        Utf8PathBufValueParser
    }
}

#[derive(Clone, Debug)]
pub struct Utf8PathBufValueParser;

impl TypedValueParser for Utf8PathBufValueParser {
    type Value = Utf8PathBuf;

    fn parse_ref(&self, cmd: &Command, _arg: Option<&Arg>, value: &OsStr) -> Result<Self::Value, Error> {
        let result = value.try_into();
        result.map_err(|_err| Error::new(ErrorKind::InvalidUtf8).with_cmd(cmd))
    }
}
