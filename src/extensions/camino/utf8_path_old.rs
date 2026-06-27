use camino::Utf8Path as CaminoUtf8Path;
use derive_more::Deref;

pub mod as_ref_path;
pub mod from_temp_dir;

#[derive(Deref, Debug)]
pub struct Utf8PathOld(pub CaminoUtf8Path);

impl Utf8PathOld {}
