pub trait RenameModule {
    type Output;

    fn rename_module(self, module_name_old: &str, module_name_new: &str) -> Self::Output;
}

mod impl_rename_module_for_syn_file;

pub use impl_rename_module_for_syn_file::*;
