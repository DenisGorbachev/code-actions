use cargo_metadata::CompilerMessage;
use cargo_metadata::diagnostic::Diagnostic;
use proc_macro2::{Ident, Span, TokenTree};
use regex::Regex;
use syn::Meta;
use syn_more::parse_attribute;
/*
Example message:

{
  "rendered": "error[E0277]: the trait bound `indexmap::IndexMap<examples::undo_last_commit_in_git::object_id::ObjectId, examples::undo_last_commit_in_git::commit_data::CommitData>: std::cmp::Ord` is not satisfied\n --> src/examples/undo_last_commit_in_git/repo.rs:9:5\n  |\n7 | #[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]\n  |                                    --- in this derive macro expansion\n8 | pub struct Repo {\n9 |     commits: IndexMap<ObjectId, CommitData>,\n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::cmp::Ord` is not implemented for `IndexMap<ObjectId, CommitData>`\n  |\n  = note: this error originates in the derive macro `Ord` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n",
  "$message_type": "diagnostic",
  "children": [],
  "code": {
    "code": "E0277",
    "explanation": "You tried to use a type which doesn't implement some trait in a place which\nexpected that trait.\n\nErroneous code example:\n\n```compile_fail,E0277\n// here we declare the Foo trait with a bar method\ntrait Foo {\n    fn bar(&self);\n}\n\n// we now declare a function which takes an object implementing the Foo trait\nfn some_func<T: Foo>(foo: T) {\n    foo.bar();\n}\n\nfn main() {\n    // we now call the method with the i32 type, which doesn't implement\n    // the Foo trait\n    some_func(5i32); // error: the trait bound `i32 : Foo` is not satisfied\n}\n```\n\nIn order to fix this error, verify that the type you're using does implement\nthe trait. Example:\n\n```\ntrait Foo {\n    fn bar(&self);\n}\n\n// we implement the trait on the i32 type\nimpl Foo for i32 {\n    fn bar(&self) {}\n}\n\nfn some_func<T: Foo>(foo: T) {\n    foo.bar(); // we can now use this method since i32 implements the\n               // Foo trait\n}\n\nfn main() {\n    some_func(5i32); // ok!\n}\n```\n\nOr in a generic context, an erroneous code example would look like:\n\n```compile_fail,E0277\nfn some_func<T>(foo: T) {\n    println!(\"{:?}\", foo); // error: the trait `core::fmt::Debug` is not\n                           //        implemented for the type `T`\n}\n\nfn main() {\n    // We now call the method with the i32 type,\n    // which *does* implement the Debug trait.\n    some_func(5i32);\n}\n```\n\nNote that the error here is in the definition of the generic function. Although\nwe only call it with a parameter that does implement `Debug`, the compiler\nstill rejects the function. It must work with all possible input types. In\norder to make this example compile, we need to restrict the generic type we're\naccepting:\n\n```\nuse std::fmt;\n\n// Restrict the input type to types that implement Debug.\nfn some_func<T: fmt::Debug>(foo: T) {\n    println!(\"{:?}\", foo);\n}\n\nfn main() {\n    // Calling the method is still fine, as i32 implements Debug.\n    some_func(5i32);\n\n    // This would fail to compile now:\n    // struct WithoutDebug;\n    // some_func(WithoutDebug);\n}\n```\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"
  },
  "level": "error",
  "message": "the trait bound `indexmap::IndexMap<examples::undo_last_commit_in_git::object_id::ObjectId, examples::undo_last_commit_in_git::commit_data::CommitData>: std::cmp::Ord` is not satisfied",
  "spans": [
    {
      "byte_end": 299,
      "byte_start": 260,
      "column_end": 44,
      "column_start": 5,
      "expansion": {
        "def_site_span": {
          "byte_end": 30333,
          "byte_start": 30320,
          "column_end": 14,
          "column_start": 1,
          "expansion": null,
          "file_name": "/Users/starfall/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/cmp.rs",
          "is_primary": false,
          "label": null,
          "line_end": 924,
          "line_start": 924,
          "suggested_replacement": null,
          "suggestion_applicability": null,
          "text": [
            {
              "highlight_end": 14,
              "highlight_start": 1,
              "text": "pub macro Ord($item:item) {"
            }
          ]
        },
        "macro_decl_name": "#[derive(Ord)]",
        "span": {
          "byte_end": 179,
          "byte_start": 176,
          "column_end": 39,
          "column_start": 36,
          "expansion": null,
          "file_name": "src/examples/undo_last_commit_in_git/repo.rs",
          "is_primary": false,
          "label": null,
          "line_end": 7,
          "line_start": 7,
          "suggested_replacement": null,
          "suggestion_applicability": null,
          "text": [
            {
              "highlight_end": 39,
              "highlight_start": 36,
              "text": "#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]"
            }
          ]
        }
      },
      "file_name": "src/examples/undo_last_commit_in_git/repo.rs",
      "is_primary": true,
      "label": "the trait `std::cmp::Ord` is not implemented for `IndexMap<ObjectId, CommitData>`",
      "line_end": 9,
      "line_start": 9,
      "suggested_replacement": null,
      "suggestion_applicability": null,
      "text": [
        {
          "highlight_end": 44,
          "highlight_start": 5,
          "text": "    commits: IndexMap<ObjectId, CommitData>,"
        }
      ]
    }
  ]
}
*/

pub fn filter_map_impossible_derives(messages: impl IntoIterator<Item = CompilerMessage>) -> impl Iterator<Item = Ident> {
    messages.into_iter().filter_map(|msg| {
        let CompilerMessage {
            message,
            ..
        } = msg;
        dbg!(&message);
        let code = message.code.as_ref()?;

        match code.code.as_str() {
            "E0277" => filter_map_impossible_derive_e0277(message),
            "E0204" => filter_map_impossible_derive_e0204(message),
            _ => None,
        }
    })
}

/// https://doc.rust-lang.org/error_codes/E0277.html
pub fn filter_map_impossible_derive_e0277(diagnostic: Diagnostic) -> Option<Ident> {
    // Find the primary span
    let primary_span = diagnostic.spans.iter().find(|span| span.is_primary)?;
    let label = primary_span.label.as_ref()?;

    // Check if the label matches the expected pattern
    let label_regex = Regex::new(r"the trait `[^`]*` is not implemented for `[^`]*`").unwrap();
    let label_is_trait_not_implemented = label_regex.is_match(primary_span.label.as_ref()?);
    let label_is_no_implementation = label.starts_with("no implementation");
    let label_is_correct = label_is_trait_not_implemented || label_is_no_implementation;
    if !label_is_correct {
        return None;
    }

    // Parse "macro_decl_name" as attribute
    let macro_decl_name = primary_span.expansion.as_ref()?.macro_decl_name.as_ref();
    let attr = parse_attribute(macro_decl_name)?;

    // Ensure it has the structure of a single MetaList with path.is_ident("derive") and a single TokenTree
    if let Meta::List(meta_list) = attr.meta {
        if meta_list.path.is_ident("derive") {
            let mut tokens_iter = meta_list.tokens.into_iter();
            let token_tree = tokens_iter.next()?;
            if tokens_iter.next().is_some() {
                // expecting only a single token_tree
                return None;
            }
            if let TokenTree::Ident(ident) = token_tree {
                return Some(ident);
            }
        }
    }

    None
}

/// https://doc.rust-lang.org/error_codes/E0204.html
fn filter_map_impossible_derive_e0204(_diagnostic: Diagnostic) -> Option<Ident> {
    Some(Ident::new("Copy", Span::call_site()))
}
