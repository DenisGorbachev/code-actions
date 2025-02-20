use derive_more::Deref;

pub mod as_ref_path;
pub mod from_temp_dir;

#[derive(Deref, Debug)]
pub struct Utf8PathOld(pub camino::Utf8Path);

impl Utf8PathOld {}
