use toml_edit::{InlineTable, Item, Table, Value};

use crate::types::outcome::Outcome;

use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::functions::get_crate_name_crate_spec::get_crate_name_crate_spec;
use crate::functions::parse_key_value::parse_key_value;
use crate::traits::cargo_info::CargoInfo;
use crate::traits::dependencies::Dependencies;
use crate::types::dependency::DependencyBuilder;
use crate::types::package_info::PackageInfo;
use crate::types::toml_file::TomlFile;

pub fn add_dependency(file: &mut TomlFile, crate_name: impl AsRef<str>, crate_spec: InlineTable) -> Outcome {
    file.modify(|doc| {
        let dependencies = doc.package_dependencies_mut();
        insert_if_not_contains(dependencies, crate_name.as_ref(), Item::Value(Value::InlineTable(crate_spec)));
    })
    .map_err(From::from)
}

pub fn add_workspace_dependency(file: &mut TomlFile, crate_name: impl AsRef<str>, crate_spec: InlineTable) -> Outcome {
    file.modify(|doc| {
        let workspace_dependencies = doc.workspace_dependencies_mut();
        insert_if_not_contains(workspace_dependencies, crate_name.as_ref(), Item::Value(Value::InlineTable(crate_spec)));
    })
    .map_err(From::from)
}

pub fn remove_package_dependency(file: &mut TomlFile, crate_name: impl AsRef<str>) -> Outcome {
    file.modify(|doc| {
        let dependencies = doc.package_dependencies_mut();
        dependencies.remove(crate_name.as_ref());
    })
    .map_err(From::from)
}

pub fn local_package_root(anchor: &Utf8Path, crate_name: impl AsRef<str>) -> Outcome<Utf8PathBuf> {
    let current_package_root = anchor.get_package_root()?;
    let local_package_root = current_package_root.join("..").join(crate_name.as_ref());
    Ok(local_package_root)
}

fn insert_if_not_contains(item: &mut Table, key: &str, value: Item) -> Option<Item> {
    if item.contains_key(key) {
        None
    } else {
        item.insert(key, value)
    }
}

// pub fn add_workspace_and_package_dependency_from_spec(anchor: &Utf8Path, crate_spec: &str, optional: bool) -> Outcome {
//     let doc = crate_spec.parse::<DocumentMut>()?;
//     let (key, value) = get_first_item(doc.iter())?;
//     let workspace_crate_spec = value
//         .as_inline_table()
//         .ok_or(anyhow!("The `item` part of the `crate_spec` must be convertible to `toml_edit::InlineTable`"))?
//         .clone();
//     let package_crate_spec = get_dependency_with_optional(optional);
//     add_global_dependency_for_workspace_and_package(anchor, key, workspace_crate_spec, package_crate_spec)
// }

pub fn add_global_dependency_from_version(anchor: &Utf8Path, crate_name_version: &str, optional: bool) -> Outcome {
    let parse_key_value_data = parse_key_value(crate_name_version, "=")?;
    let (crate_name, crate_version) = get_crate_name_crate_spec(parse_key_value_data)?;
    add_global_dependency_from_crate_name_crate_version(anchor, &crate_name, crate_version, optional)
}

pub fn add_global_dependency_from_crate_name_crate_version(anchor: &Utf8Path, crate_name: impl AsRef<str>, crate_version: impl Into<String>, optional: bool) -> Outcome {
    let crate_name = crate_name.as_ref();
    let crate_version = crate_version.into();
    let (mut package_manifest, workspace_manifest_opt) = PackageInfo::try_from(anchor)?.dissolve();
    match workspace_manifest_opt {
        None => {
            let package_crate_spec = DependencyBuilder::default()
                .version(crate_version)
                .optional_maybe(optional)
                .build()?
                .into();
            add_dependency(&mut package_manifest, crate_name, package_crate_spec)?;
        }
        Some(mut workspace_manifest) => {
            let workspace_crate_spec = DependencyBuilder::default()
                .version(crate_version)
                .build()?
                .into();
            let package_crate_spec = DependencyBuilder::default()
                .optional_maybe(optional)
                .workspace(true)
                .build()?
                .into();
            add_workspace_dependency(&mut workspace_manifest, crate_name, workspace_crate_spec)?;
            add_dependency(&mut package_manifest, crate_name, package_crate_spec)?;
        }
    }
    Ok(())
}

pub fn bool_to_opt(value: bool) -> Option<bool> {
    if value {
        Some(true)
    } else {
        None
    }
}

pub fn add_local_dependency_for_package_from_name(anchor: &Utf8Path, crate_name: impl AsRef<str>) -> Outcome {
    let crate_name_str = crate_name.as_ref();
    let (mut package_manifest, _workspace_manifest_opt) = PackageInfo::try_from(anchor)?.dissolve();
    let path = format!("../{crate_name_str}");
    let package_crate_spec = DependencyBuilder::default().path(path).build()?.into();
    add_dependency(&mut package_manifest, crate_name_str, package_crate_spec)?;
    Ok(())
}

pub fn remove_workspace_and_package_dependency(anchor: &Utf8Path, crate_name: impl AsRef<str>) -> Outcome {
    let (mut package_manifest, _workspace_manifest_opt) = PackageInfo::try_from(anchor)?.dissolve();
    remove_package_dependency(&mut package_manifest, crate_name)?;
    // TODO: remove workspace dependency if not used in other packages
    Ok(())
}
