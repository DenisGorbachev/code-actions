use time::error::Format;
use time::macros::format_description;
use time::OffsetDateTime;

pub fn get_freewrite_file_stem(now: OffsetDateTime) -> Result<String, Format> {
    now.format(format_description!("freewrite_[year]_[month]_[day]"))
}

pub fn get_freewrite_file_name(now: OffsetDateTime, extension: &str) -> Result<String, Format> {
    let stem = get_freewrite_file_stem(now)?;
    Ok(format!("{stem}.{extension}"))
}

pub fn get_freewrite_file_content(now: OffsetDateTime) -> Result<String, Format> {
    now.format(format_description!("# Freewrite on [year]-[month]-[day]\n"))
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn must_get_freewrite_filename() {
        let now = datetime!(2005-05-09 10:46:09 UTC);
        assert_eq!(get_freewrite_file_stem(now).unwrap(), "freewrite_2005_05_09")
    }
}
