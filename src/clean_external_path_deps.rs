use crate::extensions::camino::utf8_path::Utf8Path;
use crate::traits::cargo_info::CargoInfo;
use crate::traits::is_internal::IsInternal;
use crate::types::dependency::Dependency;
use crate::types::outcome::Outcome;
use anyhow::Context;
use cargo_metadata::MetadataCommand;
use cargo_toml::Manifest;
use glob::glob;
use heck::ToSnakeCase;
use itertools::Itertools;
use std::ops::Deref;
use std::{fs, io};
use toml_edit::Table;

pub fn clean_external_path_deps(path: &Utf8Path, yes: bool) -> Outcome {
    let manifest_path = path.get_package_manifest()?.canonicalize_utf8()?;
    let metadata = MetadataCommand::new()
        .manifest_path(manifest_path.as_std_path())
        .exec()?;
    let package = metadata
        .packages
        .iter()
        .find(|pkg| pkg.manifest_path == manifest_path.deref())
        .expect("Could not find the current package metadata");
    let external_deps = package.dependencies.iter().filter(|dep| match &dep.source {
        // local path dependencies have `source == None`
        None => true,
        Some(source) => !source.starts_with("registry"),
    });
    let external_deps_names = external_deps.map(|x| x.name.as_str());
    // let deps = get_external_path_deps_old(path, Utf8Path::new(metadata.workspace_root.as_path()))?;
    for name in external_deps_names {
        clean_dependency(name, Utf8Path::new(metadata.target_directory.as_path()), yes)?
    }
    Ok(())
}

pub fn clean_dependency(name: &str, target_dir: &Utf8Path, yes: bool) -> Outcome {
    let name_snake = name.to_snake_case();
    let names = [name, name_snake.as_str()];
    for name in names {
        let patterns = [
            format!("{target_dir}/**/{name}*"),
            format!("{target_dir}/**/lib{name}*"),
        ];
        for pattern in patterns {
            remove_entries_by_pattern(&pattern, yes)?;
        }
    }
    Ok(())
}

pub fn remove_entries_by_pattern(pattern: &str, yes: bool) -> Outcome<()> {
    glob(pattern)?
        .flat_map(|entry| {
            entry.map(|path_buf| {
                if yes {
                    eprintln!("Removing {}", path_buf.display());
                    if path_buf.is_dir() {
                        fs::remove_dir_all(path_buf)
                    } else {
                        fs::remove_file(path_buf)
                    }
                } else {
                    eprintln!("Would remove (use --yes to remove) {}", path_buf.display());
                    Ok(())
                }
            })
        })
        .collect::<io::Result<()>>()?;
    Ok(())
}

// pub fn get_external_path_deps_new(path: &Utf8Path) -> Outcome {
//     let package_manifest_path = path.get_package_manifest()?;
//     let manifest = Manifest::from_path(&package_manifest_path)?;
//     manifest.package.unwrap().name
// }

pub fn get_external_path_deps_old(path: &Utf8Path, project_root: &Utf8Path) -> Outcome<Vec<Name>> {
    let package_manifest_path = path.get_package_manifest()?;
    let package_manifest_root = package_manifest_path
        .parent()
        .context("Could not find package_manifest_root")?;
    let manifest = Manifest::from_path(&package_manifest_path)?;
    let deps = manifest
        .dependencies
        .into_iter()
        .filter_map(|(name, dep)| if dep.is_internal(package_manifest_root, project_root) { None } else { Some(name) })
        .collect();
    Ok(deps)
    // let package_info = PackageInfo::try_from(path)?;
    // let project_root = package_info.project_manifest().path().parent().context("Could not find project_root")?;
    // let mut deps = vec![];
    // let (package_manifest, workspace_manifest_opt) = package_info.as_refs();
    // // TODO: No need to parse the workspace Cargo.toml separately, because Manifest::from_path fills in the data from workspace
    // if let Some(dependencies) = package_manifest.package_dependencies() {
    //     let manifest_root = package_manifest.path().parent().context("Could not find manifest_root")?;
    //     deps.extend(try_collect_external_dependencies(dependencies, manifest_root, project_root)?)
    // }
    // if let Some(workspace_manifest) = workspace_manifest_opt {
    //     if let Some(dependencies) = workspace_manifest.workspace_dependencies() {
    //         let manifest_root = workspace_manifest.path().parent().context("Could not find manifest_root")?;
    //         deps.extend(try_collect_external_dependencies(dependencies, manifest_root, project_root)?)
    //     }
    // }
    // Ok(deps)
}

pub fn parse_dependencies(dependencies: &Table) -> impl Iterator<Item = Result<(Name, Dependency), toml::de::Error>> + '_ {
    dependencies.iter().map(|(name, item)| {
        // It's easier to serialize to & from string than to deal with InlineTable / Table intricacies
        let string = item.to_string();
        let dependency: Dependency = toml::from_str(&string)?;
        Ok((name.to_string(), dependency))
    })
}

pub fn try_collect_dependencies(dependencies: &Table) -> Outcome<Vec<(Name, Dependency)>> {
    parse_dependencies(dependencies)
        .try_collect()
        .map_err(toml::de::Error::into)
}

pub fn try_collect_external_dependencies(dependencies: &Table, manifest_root: &Utf8Path, project_root: &Utf8Path) -> Outcome<Vec<(Name, Dependency)>> {
    parse_dependencies(dependencies)
        .filter(|result| match result {
            Ok((_, dep)) => !dep.is_internal(manifest_root, project_root),
            Err(_) => true,
        })
        .try_collect()
        .map_err(toml::de::Error::into)
}

type Name = String;
