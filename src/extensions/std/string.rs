pub fn ensure_suffix(string: &str, suffix: &str) -> String {
    let mut s = string.to_string();
    ensure_suffix_mut(&mut s, suffix);
    s
}

pub fn ensure_suffix_mut(string: &mut String, suffix: &str) {
    if !string.ends_with(suffix) {
        string.push_str(suffix)
    }
}

pub fn ensure_suffix_mut_snake_case(string: &mut String, suffix: &str) {
    if !string.ends_with(suffix) {
        string.push('_');
        string.push_str(suffix)
    }
}

#[cfg(test)]
mod tests {
    use crate::extensions::std::string::ensure_suffix_mut_snake_case;

    #[test]
    fn must_add_suffix() {
        let mut string = String::from("some_string");
        ensure_suffix_mut_snake_case(&mut string, "suf");
        assert_eq!("some_string_suf", string);
    }
}
