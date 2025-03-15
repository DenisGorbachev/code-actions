use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;

pub fn parent_candidates<'a, 'b: 'a>(path: &'a Utf8Path, src: &'b Utf8Path) -> impl Iterator<Item = Utf8PathBuf> + 'a {
    path.parents_up_to(src).map(move |parent| {
        let mut module_file_path = parent.to_path_buf();
        module_file_path.set_extension("rs");
        module_file_path
    })
}
