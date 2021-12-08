//! "Recursive" Syntax highlighting for code in doctests and fixtures.

use std::mem;

use common::*;

use either::Either;
use hir::{InFile, Semantics};
use husky_lang_db::{
    active_parameter::ActiveParameter, defs::Definition, helpers::rust_doc::is_rust_fence,
    SymbolKind,
};
use syntax::{ast, NodeOrToken, SyntaxNode};

use crate::{
    doc_links::{doc_attributes, extract_definitions_from_docs, resolve_doc_path_for_def},
    syntax_highlighting::{highlights::Highlights, injector::Injector},
    HlMod, HlRange, HlTag, HuskyLangDatabase, IdeDatabaseSnapshot,
};

pub(super) fn ra_fixture(
    hl: &mut Highlights,
    sema: &Semantics<HuskyLangDatabase>,
    literal: &ast::String,
    expanded: &ast::String,
) -> Option<()> {
    todo!()
}

const RUSTDOC_FENCE: &'static str = "```";

/// Injection of syntax highlighting of doctests.
pub(super) fn doc_comment(
    hl: &mut Highlights,
    sema: &Semantics<HuskyLangDatabase>,
    node: InFile<&SyntaxNode>,
) {
    todo!()
}

fn find_doc_string_in_attr(attr: &hir::Attr, it: &ast::Attr) -> Option<ast::String> {
    todo!()
}

fn module_def_to_hl_tag(def: Definition) -> HlTag {
    todo!()
}
