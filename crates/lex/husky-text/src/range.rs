mod bind_from;
mod bind_into;

pub use bind_from::*;
pub use bind_into::*;
use husky_coword::Ident;

use crate::*;
use husky_dev_utils::__StaticDevSource;

use serde::{Deserialize, Serialize};
use std::{
    fmt::Write,
    path::{Path, PathBuf},
};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextRange {
    pub start: TextPosition,
    pub end: TextPosition,
}

impl TextRange {
    pub fn join(self, other: Self) -> Self {
        Self {
            start: self.start,
            end: other.end,
        }
    }

    /// returns the text range after `self` in the same line
    /// ```
    /// use husky_text::TextRange;
    ///
    /// let a: TextRange = ((0,0)..(0,3)).into();
    /// let b: TextRange = ((0,3)..(0,4)).into();
    /// assert_eq!(a.right_after(), b)
    /// ```
    pub fn right_after(self) -> Self {
        Self {
            start: self.end,
            end: self.end.to_right(1),
        }
    }
}

#[cfg(feature = "lsp_support")]
impl From<lsp_types::Range> for TextRange {
    fn from(range: lsp_types::Range) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }
}

impl TextRange {
    pub fn closed_end(&self) -> TextPosition {
        self.end.to_left(1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModuleRange {}

impl ModuleRange {
    pub fn file(&self) -> &Path {
        todo!()
        // &self.file
    }

    pub fn range(&self) -> TextRange {
        todo!()
        // self.range
    }
}

impl HasTextRange for ModuleRange {
    fn text_range(&self) -> TextRange {
        todo!()
        // self.range
    }
}

pub trait HasSourceRange: HasTextRange {
    fn source(&self) -> &Path;

    fn source_range(&self) -> ModuleRange {
        todo!()
        // ModuleRange {
        //     file: self.source().to_owned(),
        //     range: self.text_range(),
        // }
    }
}

impl<S: Deref<Target = T>, T: HasTextRange> HasTextRange for S {
    fn text_range(&self) -> TextRange {
        self.deref().text_range()
    }
}

impl<S: Deref<Target = T>, T: HasSourceRange + 'static> HasSourceRange for S {
    fn source(&self) -> &Path {
        self.deref().source()
    }
}

impl ModuleRange {
    pub fn new(_file: PathBuf, _range: TextRange) -> Self {
        todo!()
        // Self { file, range }
    }
}

impl std::fmt::Display for TextRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} - {}", self.start, self.end))
    }
}

impl std::fmt::Debug for TextRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {})", self.start, self.end))
    }
}

impl TextRange {
    pub fn whole() -> TextRange {
        ((0, 0)..(0, 4)).into()
    }

    pub fn from_line(line: u32) -> TextRange {
        ((line, 0)..(line, 4)).into()
    }

    pub fn new(range: std::ops::Range<(u32, u32)>) -> Self {
        range.into()
    }

    pub fn is_within(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }
}

impl From<__StaticDevSource> for TextRange {
    fn from(dev_src: __StaticDevSource) -> Self {
        ((dev_src.line, 0)..(dev_src.line, 10)).into()
    }
}

impl From<std::ops::Range<(u32, u32)>> for TextRange {
    fn from(range: std::ops::Range<(u32, u32)>) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }
}

impl From<std::ops::Range<TextPosition>> for TextRange {
    fn from(range: std::ops::Range<TextPosition>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

#[cfg(feature = "lsp_support")]
impl Into<lsp_types::Range> for TextRange {
    fn into(self) -> lsp_types::Range {
        lsp_types::Range::new(self.start.into(), self.end.into())
    }
}

pub trait HasTextRange {
    fn text_range(&self) -> TextRange;

    /// convenient getter
    fn text_start(&self) -> TextPosition {
        self.text_range().start
    }

    /// convenient getter
    fn text_end(&self) -> TextPosition {
        self.text_range().end
    }

    fn text_range_to(&self, other: &dyn HasTextRange) -> TextRange {
        (self.text_end()..(other.text_range().end)).into()
    }

    fn row(&self) -> TextLine {
        self.text_range().start.line
    }

    /// returns 1-baesd line index
    fn one_based_line(&self) -> u32 {
        self.text_range().start.one_based_line()
    }
}

impl HasTextRange for TextRange {
    fn text_range(&self) -> TextRange {
        *self
    }
}

pub fn new_same_line(i: u32, start: u32, end: u32) -> TextRange {
    TextRange {
        start: (i, start).into(),
        end: (i, end).into(),
    }
}

impl<T: HasTextRange> HasTextRange for [T] {
    fn text_range(&self) -> TextRange {
        if self.len() > 0 {
            ((self[0].text_range().start)..(self.last().unwrap().text_range().end)).into()
        } else {
            TextRange::default()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RangedIdent {
    pub ident: Ident,
    pub range: TextRange,
}

impl HasTextRange for RangedIdent {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
