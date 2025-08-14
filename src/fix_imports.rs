use crate::constants::SRC_DIR_NAME;
use crate::extensions::camino::utf8_path::Utf8Path;
use crate::types::outcome::Outcome;
use crate::types::package_info::PackageInfo;
use anyhow::anyhow;
use duct::cmd;
use quote::ToTokens;
use regex::Regex;
use std::borrow::Cow;
use std::fs::read_to_string;
use std::path::Path;
use syn::{File, Item, ItemMod, ItemUse, UseGlob, UsePath, UseTree, parse_file};
use syn_more::new_item_use;
use walkdir::{DirEntry, WalkDir};

// TODO: use `yes` instead of `dry_run`
pub fn fix_imports(anchor: &Utf8Path, yes: bool) -> Outcome {
    let package_info = PackageInfo::try_from(anchor)?;
    let (package_manifest, workspace_manifest) = package_info.dissolve();
    if workspace_manifest.is_some() {
        return Err(anyhow!("Not supported for workspaces"));
    }
    // TODO: It can be overridden on a per-target basis via `path` - see [reference](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#configuring-a-target)
    let src = package_manifest.path().get_src_root()?.join(SRC_DIR_NAME);
    // let src_path_buf = PathBuf::from(src.clone());
    let walker = WalkDir::new(src);
    walker
        .into_iter()
        .try_for_each(|entry_result| fix_imports_in_entry(entry_result, yes))?;
    if yes {
        eprintln!("Running rustfmt");
        cmd!("cargo", "fmt", "--all", "--manifest-path", package_manifest.path()).run()?;
    }
    Ok(())
}

pub fn fix_imports_in_entry(entry_result: walkdir::Result<DirEntry>, yes: bool) -> Outcome {
    let entry = entry_result?;
    let path = entry.path();
    if path.extension().and_then(|s| s.to_str()) == Some("rs") {
        fix_rust_file(path, yes)
    } else {
        Ok(())
    }
}

pub fn fix_rust_file(path: &Path, yes: bool) -> Outcome {
    eprintln!("Checking {}", path.display());
    // TODO: Use `modify_rust_file`
    let content = read_to_string(path)?;
    let file = parse_file(&content)?;
    let content_new = if is_aggregate_syn_file(&file) {
        let file = fix_aggregate_syn_file(file);
        prettyplease::unparse(&file)
    } else {
        fix_regular_file(&content).to_string()
    };
    if content_new == content {
        eprintln!("Already correct {}", path.display());
    } else if yes {
        eprintln!("Overwriting {}", path.display());
        fs_err::write(path, content_new)?;
    } else {
        eprintln!("Would overwrite (use --yes to overwrite) {}", path.display());
    }
    Ok(())
}

pub fn fix_regular_file(content: &str) -> Cow<'_, str> {
    let re = Regex::new(r"use crate::((?:\w+::)*)").unwrap();
    re.replace_all(content, "use crate::")
}

pub fn is_aggregate_syn_file(file: &File) -> bool {
    file.items.iter().all(|item| match item {
        Item::Mod(mod_item) => mod_item.content.is_none(),
        Item::Use(_) | Item::Type(_) => true,
        _ => false,
    })
}

pub fn fix_aggregate_syn_file(mut file: File) -> File {
    let mut mod_items: Vec<ItemMod> = Vec::new();
    let mut use_items: Vec<ItemUse> = Vec::new();
    let mut other_items: Vec<Item> = Vec::new();

    // Separate items into mod, use, and other
    for item in file.items.drain(..) {
        match item {
            Item::Mod(mut mod_item) => {
                // Remove the `pub` modifier
                mod_item.vis = syn::Visibility::Inherited;
                mod_items.push(mod_item);
            }
            Item::Use(use_item) => use_items.push(use_item),
            other => other_items.push(other),
        }
    }

    let non_test_mod_items = mod_items.iter().filter(|item| !is_test_mod(item));

    // Create missing use items for mod items
    for mod_item in non_test_mod_items {
        let mod_name = &mod_item.ident;
        let corresponding_use = use_items.iter().find(|use_item| {
            if let UseTree::Path(use_path) = &use_item.tree {
                if use_path.ident == *mod_name {
                    if let UseTree::Glob(_) = &*use_path.tree {
                        return true;
                    }
                }
            }
            false
        });

        if corresponding_use.is_none() {
            let mut new_use_item = new_item_use(UseTree::Path(UsePath {
                ident: mod_name.clone(),
                colon2_token: Default::default(),
                tree: Box::new(UseTree::Glob(UseGlob {
                    star_token: Default::default(),
                })),
            }));
            new_use_item.vis = syn::Visibility::Public(Default::default());
            // Copy attributes from mod item to use item, excluding #[cfg(test)]
            new_use_item.attrs.extend(mod_item.attrs.clone());
            use_items.push(new_use_item);
        }
    }

    // Reassemble the items in the correct order
    file.items = mod_items.into_iter().map(Item::Mod).collect();
    file.items.extend(use_items.into_iter().map(Item::Use));
    file.items.extend(other_items);

    file
}

pub fn is_test_mod(item: &ItemMod) -> bool {
    item.attrs
        .iter()
        .any(|attr| attr.path().is_ident("cfg") && attr.meta.to_token_stream().to_string() == "(test)")
}

pub fn fix_regular_syn_file(mut file: File) -> File {
    file.items = file
        .items
        .into_iter()
        .map(|item| {
            if let Item::Use(mut use_item) = item {
                use_item.tree = fix_use_tree(use_item.tree);
                Item::Use(use_item)
            } else {
                item
            }
        })
        .collect();

    file
}

pub fn fix_use_tree(tree: UseTree) -> UseTree {
    match tree {
        UseTree::Path(mut use_path) if use_path.ident == "crate" => {
            // If the path starts with "crate", we need to modify it
            let mut current_tree = use_path.tree.clone();
            while let UseTree::Path(ref inner_path) = *current_tree {
                let tree = inner_path.tree.clone();
                if matches!(*tree, UseTree::Glob(_)) {
                    break;
                } else {
                    current_tree = tree;
                }
            }
            use_path.tree = current_tree;
            UseTree::Path(use_path)
        }
        UseTree::Path(use_path) => UseTree::Path(UsePath {
            ident: use_path.ident,
            colon2_token: use_path.colon2_token,
            tree: Box::new(fix_use_tree(*use_path.tree)),
        }),
        UseTree::Group(use_group) => UseTree::Group(syn::UseGroup {
            brace_token: use_group.brace_token,
            items: use_group.items.into_iter().map(fix_use_tree).collect(),
        }),
        // Other cases (Name, Rename, Glob) don't need modification
        _ => tree,
    }
}
