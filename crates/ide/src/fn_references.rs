//! This module implements a methods and free functions search in the specified file.
//! We have to skip tests, so cannot reuse file_structure module.

use hir::Semantics;
use ide_assists::utils::test_related_attribute;
use husky_lang_db::HuskyLangDatabase;
use syntax::{ast, SyntaxNode};

use crate::{FileId, FileRange};

pub(crate) fn find_all_methods(db: &HuskyLangDatabase, file_id: FileId) -> Vec<FileRange> {
    todo!()
}

fn method_range(item: SyntaxNode, file_id: FileId) -> Option<FileRange> {
    todo!()
}
