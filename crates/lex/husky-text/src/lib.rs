mod column;
mod db;
mod indent;
mod info;
mod position;
mod range;
mod row;
#[cfg(test)]
mod tests;

pub use indent::TextIndent;
pub use info::*;
pub use position::{FilePosition, TextPosition};
pub use range::*;
pub type CharIter<'token_line> = std::iter::Peekable<Enumerate<Chars<'token_line>>>;
pub use column::Column;
pub use db::TextDb;
pub use row::Row;

use std::{iter::Enumerate, ops::Deref, str::Chars, sync::Arc};

#[salsa::jar(db = TextDb)]
pub struct TextJar();

#[derive(Clone, PartialEq, Eq)]
pub struct Text {
    content: String,
}

impl std::fmt::Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Text...")
    }
}

impl std::ops::Index<TextRange> for Text {
    type Output = str;

    fn index(&self, index: TextRange) -> &Self::Output {
        todo!()
    }
}

impl std::ops::Index<std::ops::Range<(u32, u32)>> for Text {
    type Output = str;

    fn index(&self, index: std::ops::Range<(u32, u32)>) -> &Self::Output {
        self.text_within(index.into())
    }
}

impl Text {
    pub(crate) fn new(content: impl Into<String>) -> Self {
        let content: String = content.into();
        Self { content }
        // Self {
        //     lines: raw.lines().map(|line| line.chars().collect()).collect(),
        // }
    }

    pub fn text_within(&self, range: TextRange) -> &str {
        todo!()
        // if range.end.i() > range.start.i() {
        //     let mut result = String::new();
        //     for c in &self.lines[range.start.i() as usize][(range.start.j() as usize)..] {
        //         result.push(*c);
        //     }
        //     for i in (range.start.i() + 1)..range.end.i() {
        //         for c in &self.lines[i as usize] {
        //             result.push(*c);
        //         }
        //     }
        //     for c in &self.lines[range.end.i() as usize][..(range.end.j() as usize)] {
        //         result.push(*c);
        //     }
        //     result
        // } else if range.end.i() == range.start.i() {
        //     self.lines[range.start.i() as usize][(range.start.j() as usize)..range.end.j() as usize]
        //         .iter()
        //         .collect()
        // } else {
        //     "".into()
        // }
    }
}
