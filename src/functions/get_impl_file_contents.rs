use std::iter::{empty, once};

use crate::types::outcome::Outcome;
use anyhow::Context;
use derive_more::{Error, From};
use derive_new::new;
use fmt_derive::Display;
use fs_err::File;
use heck::ToSnakeCase;
use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use standard_traits::{Get, Of};
use syn::punctuated::Punctuated;
use syn::token::For;
use syn::{parse_str, AngleBracketedGenericArguments, ConstParam, Expr, ExprLit, GenericArgument, GenericParam, Generics, Item, ItemImpl, ItemUse, Lit, Path, PathArguments, PathSegment, Token, Type, TypeParam, TypePath, UseName, UsePath, UseTree};
use syn_more::default_item_impl;
use syn_more::default_type_path;
use syn_more::get_generics_from_params;
use syn_more::get_iter_generic_param_from_ref_trait_path;
use syn_more::get_use_tree_with_crate;
use syn_more::into_type_path_for_ident;
use syn_more::new_item_use;
use syn_more::{maybe_ref_ident_for_ref_item, parse_main_item_from_path};

use crate::constants::SRC_DIR_NAME;
use crate::extensions::camino::utf8_path::Utf8Path;
use crate::extensions::camino::utf8_path_buf::Utf8PathBuf;
use crate::functions::format::format_token_stream_prettyplease;
use crate::generate_file::create_module_file;
use crate::get_relative_path::get_relative_path_anchor_stem_rs;

pub fn generate_impl_from_anchor_trait_path(anchor: &Utf8Path, trait_path: &str) -> Outcome<File> {
    let trait_path: Path = parse_str(trait_path)?;
    let path = get_path_from_anchor_ref_trait_path(anchor, &trait_path)?;
    let contents = get_contents_from_anchor_trait_path(anchor, trait_path)?;
    create_module_file(path, contents)
}

fn get_path_from_anchor_ref_trait_path(anchor: &Utf8Path, trait_path: &Path) -> Outcome<Utf8PathBuf> {
    let stem = get_stem_from_trait_path(trait_path);
    let path = get_relative_path_anchor_stem_rs(anchor, &stem)?;
    Ok(path)
}

fn get_contents_from_anchor_trait_path(anchor: &Utf8Path, trait_path: Path) -> Outcome<String> {
    let stream = get_impl_token_stream(anchor, trait_path)?;
    let contents = format_token_stream_prettyplease(stream)?;
    Ok(contents)
}

fn get_stem_from_trait_path(trait_path: &Path) -> String {
    let impl_part = "impl".to_string();
    let segments_iter = trait_path
        .segments
        .iter()
        .flat_map(get_stem_parts_from_ref_path_segment);
    let mut full_iter = once(impl_part).chain(segments_iter);
    full_iter.join("_")
}

pub fn get_stem_parts_from_ref_path_segment(path_segment: &PathSegment) -> StemPartIterBox<'_> {
    let ident_stem_part = path_segment.ident.to_string().to_snake_case();
    let iter = once(ident_stem_part).chain(get_stem_parts_from_ref_path_arguments(&path_segment.arguments));
    Box::new(iter)
}

pub type StemPartIterBox<'a> = Box<dyn Iterator<Item = StemPart> + 'a>;

pub type StemPart = String;

pub fn get_stem_parts_from_ref_path_arguments(path_arguments: &PathArguments) -> StemPartIterBox<'_> {
    match path_arguments {
        PathArguments::None => Box::new(empty()),
        PathArguments::AngleBracketed(angle_bracketed_generic_arguments) => get_stem_parts_from_ref_angle_bracketed_generic_arguments(angle_bracketed_generic_arguments),
        PathArguments::Parenthesized(_) => todo!(),
    }
}

fn get_stem_parts_from_ref_angle_bracketed_generic_arguments(angle_bracketed_generic_arguments: &AngleBracketedGenericArguments) -> StemPartIterBox<'_> {
    let iter = angle_bracketed_generic_arguments
        .args
        .iter()
        .flat_map(get_stem_parts_from_ref_generic_argument);
    Box::new(iter)
}

fn get_stem_parts_from_ref_generic_argument(generic_argument: &GenericArgument) -> StemPartIterBox<'_> {
    match generic_argument {
        GenericArgument::Lifetime(_) => Box::new(empty()),
        GenericArgument::Type(ty) => get_stem_parts_from_ref_type(ty),
        GenericArgument::Const(expr) => get_stem_parts_from_ref_expr(expr),
        GenericArgument::AssocType(_) => todo!(),
        GenericArgument::AssocConst(_) => todo!(),
        GenericArgument::Constraint(_) => todo!(),
        _ => todo!(),
    }
}

fn get_stem_parts_from_ref_type(ty: &Type) -> StemPartIterBox<'_> {
    match &ty {
        Type::Path(path) => Box::new(get_stem_parts_from_ref_type_path(path)),
        Type::Reference(reference) => get_stem_parts_from_ref_type(&reference.elem),
        _ => todo!(),
    }
}

fn get_stem_parts_from_ref_type_path(type_path: &TypePath) -> impl Iterator<Item = StemPart> + '_ {
    get_stem_parts_from_ref_path(&type_path.path)
}

fn get_stem_parts_from_ref_path(path: &Path) -> impl Iterator<Item = StemPart> + '_ {
    path.segments
        .iter()
        .flat_map(get_stem_parts_from_ref_path_segment)
}

fn get_stem_parts_from_ref_expr(expr: &Expr) -> StemPartIterBox<'_> {
    match expr {
        Expr::Lit(lit) => get_stem_parts_from_ref_expr_lit(lit),
        _ => todo!(),
    }
}

fn get_stem_parts_from_ref_expr_lit(expr_lit: &ExprLit) -> StemPartIterBox<'_> {
    match &expr_lit.lit {
        Lit::Str(str) => Box::new(once(str.value())),
        Lit::Char(char) => Box::new(once(char.value().to_string())),
        Lit::Int(int) => Box::new(once(int.to_string())),
        Lit::Bool(bool) => Box::new(once(bool.value().to_string())),
        Lit::Verbatim(literal) => Box::new(once(literal.to_string())),
        _ => todo!(),
    }
}

pub fn get_impl_token_stream(anchor: &Utf8Path, trait_path: Path) -> Outcome<TokenStream> {
    let item = parse_main_item_from_path(anchor)?.with_context(|| format!("Main item not found in \"{anchor}\""))?;
    let item_ident = maybe_ref_ident_for_ref_item(&item).context("Expected the main item to have an ident")?;
    let use_tree_root = UseTree::Name(UseName {
        ident: item_ident.to_owned(),
    });
    let item_impl = get_item_impl(trait_path, item)?;
    let module_item_use = get_item_use_from_file_path(anchor, use_tree_root)?;
    Ok(quote! {
        #module_item_use

        #item_impl
    })
}

pub fn get_item_use_from_file_path(path: &Utf8Path, use_tree_root: UseTree) -> Outcome<ItemUse> {
    let use_tree = try_from_use_tree_into_utf8_path(path, use_tree_root)?;
    let use_tree_with_crate = get_use_tree_with_crate(use_tree);
    Ok(new_item_use(use_tree_with_crate))
}

pub fn try_from_use_tree_into_utf8_path(path: &Utf8Path, use_tree_root: UseTree) -> Outcome<UseTree> {
    let src_root = path.get_src_root()?;
    let src = src_root.join(SRC_DIR_NAME);
    let parent_stems = path
        .ancestors_up_to(src.as_path())
        .filter_map(|p| p.file_stem());
    let use_tree = fold_as_str_slices_into_use_tree(use_tree_root, parent_stems);
    Ok(use_tree)
}

pub fn get_use_name_from_path(path: &Utf8Path) -> Outcome<UseName> {
    let file_stem = path.file_stem().context("Failed to get file_stem")?;
    Ok(UseName {
        ident: Ident::of(file_stem),
    })
}

pub fn fold_as_str_slices_into_use_tree<'a, T: AsRef<str> + 'a + ?Sized>(root: UseTree, slices: impl Iterator<Item = &'a T>) -> UseTree {
    fold_str_slices_into_use_tree(root, slices.map(|s| s.as_ref()))
}

pub fn fold_str_slices_into_use_tree<'a>(root: UseTree, slices: impl Iterator<Item = &'a str>) -> UseTree {
    slices.fold(root, |memo, str| {
        UseTree::Path(UsePath {
            ident: Ident::of(str),
            colon2_token: Default::default(),
            tree: Box::new(memo),
        })
    })
}

pub fn get_item_impl(trait_path: Path, item: Item) -> Result<ItemImpl, GetItemImplError> {
    let (type_ident, type_generics) = try_into_ident_and_generics_for_item(item)?;
    let self_ty = get_self_ty_from_ident_and_generics(type_ident, type_generics.clone());
    let trait_generics = get_iter_generic_param_from_ref_trait_path(&trait_path);
    let generic_params = trait_generics.chain(type_generics.params);
    let mut item_impl = default_item_impl(Box::new(self_ty));
    item_impl.generics = get_generics_from_params(generic_params);
    item_impl.trait_ = Some((None, trait_path, For(Span::call_site())));
    Ok(item_impl)
}

pub fn get_self_ty_from_ident_and_generics(ident: Ident, generics: Generics) -> Type {
    let generic_arguments: Punctuated<GenericArgument, Token![,]> = generics
        .params
        .into_iter()
        .map(into_generic_argument_for_generic_param)
        .collect();
    let angle_bracketed_generic_arguments = generic_arguments.get::<AngleBracketedGenericArguments>();
    let arguments = if angle_bracketed_generic_arguments.args.is_empty() {
        PathArguments::None
    } else {
        PathArguments::AngleBracketed(angle_bracketed_generic_arguments)
    };
    let segment = PathSegment {
        ident,
        arguments,
    };
    let path = Path::from(segment);
    let type_path = default_type_path(path);
    Type::Path(type_path)
}

pub fn into_generic_argument_for_generic_param(param: GenericParam) -> GenericArgument {
    match param {
        GenericParam::Lifetime(param) => GenericArgument::Lifetime(param.lifetime),
        GenericParam::Type(param) => GenericArgument::Type(into_type_from_type_param(param)),
        GenericParam::Const(param) => GenericArgument::Const(into_const_expr_from_const_param(param)),
    }
}

pub fn into_const_expr_from_const_param(_param: ConstParam) -> Expr {
    todo!()
}

/// NOTE: This implementation may be incorrect
pub fn into_type_from_type_param(param: TypeParam) -> Type {
    Type::Path(into_type_path_for_ident(param.ident))
}

pub fn try_into_ident_and_generics_for_item(item: Item) -> Result<(Ident, Generics), UnsupportedItemError> {
    match item {
        Item::Enum(item_enum) => Ok((item_enum.ident, item_enum.generics)),
        Item::Struct(item_struct) => Ok((item_struct.ident, item_struct.generics)),
        Item::Type(item_type) => Ok((item_type.ident, item_type.generics)),
        _ => Err(UnsupportedItemError::new(item.into_token_stream().to_string())),
    }
}

#[derive(Error, Display, From, Clone, Debug)]
pub enum GetItemImplError {
    TheUnsupportedItemError(UnsupportedItemError),
}

#[derive(new, Error, Display, Eq, PartialEq, Hash, Clone, Debug)]
pub struct UnsupportedItemError {
    pub item: String,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use syn::{parse_quote, ItemImpl, ItemStruct};

    use super::*;

    #[test]
    fn must_get_item_impl() {
        let person_struct: ItemStruct = parse_quote! { struct Person {} };
        let pair_struct: ItemStruct = parse_quote! { struct Pair<A, B> {} };
        let person_item = Item::Struct(person_struct);
        let pair_item = Item::Struct(pair_struct);
        let default_trait_path: Path = parse_quote! { Default };
        let try_from_str_trait_path: Path = parse_quote! { TryFrom<&'a str> };
        let impl_default_for_person_expected: ItemImpl = parse_quote! { impl Default for Person {} };
        let impl_default_for_pair_expected: ItemImpl = parse_quote! { impl<A, B> Default for Pair<A, B> {} };
        let impl_try_from_str_for_pair_expected: ItemImpl = parse_quote! { impl<'a, A, B> TryFrom<&'a str> for Pair<A, B> {} };
        let impl_default_for_person_actual = get_item_impl(default_trait_path.clone(), person_item.clone()).unwrap();
        let impl_default_for_pair_actual = get_item_impl(default_trait_path.clone(), pair_item.clone()).unwrap();
        let impl_try_from_str_for_pair_actual = get_item_impl(try_from_str_trait_path.clone(), pair_item.clone()).unwrap();
        assert_eq!(impl_default_for_person_actual, impl_default_for_person_expected);
        assert_eq!(impl_default_for_pair_actual, impl_default_for_pair_expected);
        assert_eq!(impl_try_from_str_for_pair_actual, impl_try_from_str_for_pair_expected);
    }
}
