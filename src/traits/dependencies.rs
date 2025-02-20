use crate::types::toml_file::TomlFile;
use std::ops::{Deref, DerefMut};
use toml_edit::{DocumentMut, Item, Table};

pub trait Dependencies {
    fn package_dependencies(&self) -> Option<&Table>;

    fn workspace_dependencies(&self) -> Option<&Table>;

    fn package_dependencies_mut(&mut self) -> &mut Table;

    fn workspace_dependencies_mut(&mut self) -> &mut Table;
}

impl Dependencies for DocumentMut {
    fn package_dependencies(&self) -> Option<&Table> {
        self["dependencies"].as_table()
    }

    fn workspace_dependencies(&self) -> Option<&Table> {
        self["workspace"]["dependencies"].as_table()
    }

    fn package_dependencies_mut(&mut self) -> &mut Table {
        get_table(&mut self["dependencies"])
            .as_table_mut()
            .expect("dependencies must be a table")
    }

    fn workspace_dependencies_mut(&mut self) -> &mut Table {
        get_table(&mut self["workspace"]["dependencies"])
            .as_table_mut()
            .expect("workspace.dependencies must be a table")
    }
}

fn get_table(item: &mut Item) -> &mut Item {
    item.or_insert(Item::Table(Default::default()))
}

impl Dependencies for TomlFile {
    fn package_dependencies(&self) -> Option<&Table> {
        self.deref().package_dependencies()
    }

    fn workspace_dependencies(&self) -> Option<&Table> {
        self.deref().workspace_dependencies()
    }

    fn package_dependencies_mut(&mut self) -> &mut Table {
        self.deref_mut().package_dependencies_mut()
    }

    fn workspace_dependencies_mut(&mut self) -> &mut Table {
        self.deref_mut().workspace_dependencies_mut()
    }
}
