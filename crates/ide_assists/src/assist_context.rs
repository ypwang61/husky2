//! See [`AssistContext`].

use std::mem;

use common::*;

use hir::Semantics;
use husky_lang_db::{
    helpers::SnippetCap,
    vfs::{FileId, FileRange},
};
use husky_lang_db::{
    label::Label,
    source_change::{FileSystemEdit, SourceChange},
    HuskyLangDatabase,
};
use syntax::{
    Direction, SingleFileParseTree, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxNodePtr,
    SyntaxToken,
};
use text_edit::{TextEdit, TextEditBuilder};

use crate::{
    assist_config::AssistConfig, Assist, AssistId, AssistKind, AssistResolveStrategy, GroupLabel,
};

/// `AssistContext` allows to apply an assist or check if it could be applied.
///
/// Assists use a somewhat over-engineered approach, given the current needs.
/// The assists workflow consists of two phases. In the first phase, a user asks
/// for the list of available assists. In the second phase, the user picks a
/// particular assist and it gets applied.
///
/// There are two peculiarities here:
///
/// * first, we ideally avoid computing more things then necessary to answer "is
///   assist applicable" in the first phase.
/// * second, when we are applying assist, we don't have a guarantee that there
///   weren't any changes between the point when user asked for assists and when
///   they applied a particular assist. So, when applying assist, we need to do
///   all the checks from scratch.
///
/// To avoid repeating the same code twice for both "check" and "apply"
/// functions, we use an approach reminiscent of that of Django's function based
/// views dealing with forms. Each assist receives a runtime parameter,
/// `resolve`. It first check if an edit is applicable (potentially computing
/// info required to compute the actual edit). If it is applicable, and
/// `resolve` is `true`, it then computes the actual edit.
///
/// So, to implement the original assists workflow, we can first apply each edit
/// with `resolve = false`, and then applying the selected edit again, with
/// `resolve = true` this time.
///
/// Note, however, that we don't actually use such two-phase logic at the
/// moment, because the LSP API is pretty awkward in this place, and it's much
/// easier to just compute the edit eagerly :-)
pub(crate) struct AssistContext<'a> {
    pub(crate) config: &'a AssistConfig,
    pub(crate) sema: Semantics<'a, HuskyLangDatabase>,
    frange: FileRange,
    trimmed_range: TextRange,
    source_file: SingleFileParseTree,
}

impl<'a> AssistContext<'a> {
    pub(crate) fn new(
        sema: Semantics<'a, HuskyLangDatabase>,
        config: &'a AssistConfig,
        frange: FileRange,
    ) -> AssistContext<'a> {
        todo!()
    }

    pub(crate) fn db(&self) -> &HuskyLangDatabase {
        self.sema.db
    }

    // NB, this ignores active selection.
    pub(crate) fn offset(&self) -> TextSize {
        self.frange.range.start()
    }

    pub(crate) fn file_id(&self) -> FileId {
        self.frange.file_id
    }

    pub(crate) fn has_empty_selection(&self) -> bool {
        self.trimmed_range.is_empty()
    }

    /// Returns the selected range trimmed for whitespace tokens, that is the range will be snapped
    /// to the nearest enclosed token.
    pub(crate) fn selection_trimmed(&self) -> TextRange {
        self.trimmed_range
    }

    pub(crate) fn token_at_offset(&self) -> TokenAtOffset<SyntaxToken> {
        todo!()
    }
    pub(crate) fn find_token_syntax_at_offset(&self, kind: SyntaxKind) -> Option<SyntaxToken> {
        todo!()
    }
    pub(crate) fn find_token_at_offset(&self) -> Option<SyntaxNode> {
        todo!()
    }
    pub(crate) fn find_node_at_offset(&self) -> Option<SyntaxNode> {
        todo!()
    }
    pub(crate) fn find_node_at_range(&self) -> Option<SyntaxNode> {
        todo!()
    }
    pub(crate) fn find_node_at_offset_with_descend(&self) -> Option<SyntaxNode> {
        todo!()
    }
    /// Returns the element covered by the selection range, this excludes trailing whitespace in the selection.
    pub(crate) fn covering_element(&self) -> SyntaxElement {
        todo!()
    }
}

pub(crate) struct Assists {
    file: FileId,
    resolve: AssistResolveStrategy,
    buf: Vec<Assist>,
    allowed: Option<Vec<AssistKind>>,
}

impl Assists {
    pub(crate) fn new(ctx: &AssistContext, resolve: AssistResolveStrategy) -> Assists {
        Assists {
            resolve,
            file: ctx.frange.file_id,
            buf: Vec::new(),
            allowed: ctx.config.allowed.clone(),
        }
    }

    pub(crate) fn finish(mut self) -> Vec<Assist> {
        self.buf.sort_by_key(|assist| assist.target.len());
        self.buf
    }

    pub(crate) fn add(
        &mut self,
        id: AssistId,
        label: impl Into<String>,
        target: TextRange,
        f: impl FnOnce(&mut AssistBuilder),
    ) -> Option<()> {
        let mut f = Some(f);
        self.add_impl(None, id, label.into(), target, &mut |it| {
            f.take().unwrap()(it)
        })
    }

    pub(crate) fn add_group(
        &mut self,
        group: &GroupLabel,
        id: AssistId,
        label: impl Into<String>,
        target: TextRange,
        f: impl FnOnce(&mut AssistBuilder),
    ) -> Option<()> {
        let mut f = Some(f);
        self.add_impl(Some(group), id, label.into(), target, &mut |it| {
            f.take().unwrap()(it)
        })
    }

    fn add_impl(
        &mut self,
        group: Option<&GroupLabel>,
        id: AssistId,
        label: String,
        target: TextRange,
        f: &mut dyn FnMut(&mut AssistBuilder),
    ) -> Option<()> {
        if !self.is_allowed(&id) {
            return None;
        }

        let source_change = if self.resolve.should_resolve(&id) {
            let mut builder = AssistBuilder::new(self.file);
            f(&mut builder);
            Some(builder.finish())
        } else {
            None
        };

        let label = Label::new(label);
        let group = group.cloned();
        self.buf.push(Assist {
            id,
            label,
            group,
            target,
            source_change,
        });
        Some(())
    }

    fn is_allowed(&self, id: &AssistId) -> bool {
        match &self.allowed {
            Some(allowed) => allowed.iter().any(|kind| kind.contains(id.1)),
            None => true,
        }
    }
}

pub(crate) struct AssistBuilder {
    edit: TextEditBuilder,
    file_id: FileId,
    source_change: SourceChange,

    /// Maps the original, immutable `SyntaxNode` to a `clone_for_update` twin.
    mutated_tree: Option<TreeMutator>,
}

pub(crate) struct TreeMutator {
    immutable: SyntaxNode,
    mutable_clone: SyntaxNode,
}

impl TreeMutator {
    pub(crate) fn new(immutable: &SyntaxNode) -> TreeMutator {
        todo!()
    }

    pub(crate) fn make_mut(&self, node: &SyntaxNode) -> SyntaxNode {
        todo!()
    }

    pub(crate) fn make_syntax_mut(&self, node: &SyntaxNode) -> SyntaxNode {
        todo!()
    }
}

impl AssistBuilder {
    pub(crate) fn new(file_id: FileId) -> AssistBuilder {
        AssistBuilder {
            edit: TextEdit::builder(),
            file_id,
            source_change: SourceChange::default(),
            mutated_tree: None,
        }
    }

    pub(crate) fn edit_file(&mut self, file_id: FileId) {
        self.commit();
        self.file_id = file_id;
    }

    fn commit(&mut self) {
        todo!()
    }

    pub(crate) fn make_mut(&mut self, node: SyntaxNode) -> SyntaxNode {
        todo!()
    }
    /// Returns a copy of the `node`, suitable for mutation.
    ///
    /// Syntax trees in husky-lang-server are typically immutable, and mutating
    /// operations panic at runtime. However, it is possible to make a copy of
    /// the tree and mutate the copy freely. Mutation is based on interior
    /// mutability, and different nodes in the same tree see the same mutations.
    ///
    /// The typical pattern for an assist is to find specific nodes in the read
    /// phase, and then get their mutable couterparts using `make_mut` in the
    /// mutable state.
    pub(crate) fn make_syntax_mut(&mut self, node: SyntaxNode) -> SyntaxNode {
        self.mutated_tree
            .get_or_insert_with(|| TreeMutator::new(&node))
            .make_syntax_mut(&node)
    }

    /// Remove specified `range` of text.
    pub(crate) fn delete(&mut self, range: TextRange) {
        self.edit.delete(range)
    }
    /// Append specified `text` at the given `offset`
    pub(crate) fn insert(&mut self, offset: TextSize, text: impl Into<String>) {
        self.edit.insert(offset, text.into())
    }
    /// Append specified `snippet` at the given `offset`
    pub(crate) fn insert_snippet(
        &mut self,
        _cap: SnippetCap,
        offset: TextSize,
        snippet: impl Into<String>,
    ) {
        self.source_change.is_snippet = true;
        self.insert(offset, snippet);
    }
    /// Replaces specified `range` of text with a given string.
    pub(crate) fn replace(&mut self, range: TextRange, replace_with: impl Into<String>) {
        self.edit.replace(range, replace_with.into())
    }
    /// Replaces specified `range` of text with a given `snippet`.
    pub(crate) fn replace_snippet(
        &mut self,
        _cap: SnippetCap,
        range: TextRange,
        snippet: impl Into<String>,
    ) {
        self.source_change.is_snippet = true;
        self.replace(range, snippet);
    }
    pub(crate) fn replace_ast(&mut self, old: SyntaxNode, new: SyntaxNode) {
        todo!()
    }
    // pub(crate) fn create_file(&mut self, dst: AnchoredPathBuf, content: impl Into<String>) {
    //     let file_system_edit = FileSystemEdit::CreateFile {
    //         dst,
    //         initial_contents: content.into(),
    //     };
    //     self.source_change.push_file_system_edit(file_system_edit);
    // }
    // pub(crate) fn move_file(&mut self, src: FileId, dst: AnchoredPathBuf) {
    //     let file_system_edit = FileSystemEdit::MoveFile { src, dst };
    //     self.source_change.push_file_system_edit(file_system_edit);
    // }

    fn finish(mut self) -> SourceChange {
        self.commit();
        mem::take(&mut self.source_change)
    }
}
