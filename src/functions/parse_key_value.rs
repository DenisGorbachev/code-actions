use derive_more::{Error, From};
use derive_new::new;
use fmt_derive::Display;

#[derive(new, Error, Display, Eq, PartialEq, Hash, Clone, Debug)]
pub struct ParseKeyValueError;

#[derive(From, Eq, PartialEq, Hash, Clone, Debug)]
pub enum ParseKeyValueData<'a> {
    TheKey(&'a str),
    TheKeyValue((&'a str, &'a str)),
}

pub fn parse_key_value<'a>(key_value: &'a str, separator: &str) -> Result<ParseKeyValueData<'a>, ParseKeyValueError> {
    let parts: Vec<&str> = key_value.split(separator).map(str::trim).collect();
    match parts.len() {
        1 => Ok(parts[0].into()),
        2 => Ok((parts[0], parts[1]).into()),
        _ => Err(ParseKeyValueError),
    }
}

#[cfg(test)]
mod tests {
    use crate::functions::parse_key_value;
    use crate::functions::parse_key_value::ParseKeyValueError;

    #[test]
    fn must_parse_key_value() {
        assert_eq!(parse_key_value::parse_key_value("foo", "="), Ok("foo".into()));
        assert_eq!(parse_key_value::parse_key_value("foo = 2.0.4", "="), Ok(("foo", "2.0.4").into()));
        assert_eq!(parse_key_value::parse_key_value("foo = 2.0.4 = something", "="), Err(ParseKeyValueError));
    }
}
