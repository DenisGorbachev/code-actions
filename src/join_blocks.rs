pub fn join_blocks<T: AsRef<str>>(blocks: &[&[T]]) -> String {
    blocks
        .iter()
        .map(|block| {
            block
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>()
                .join("\n")
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}
