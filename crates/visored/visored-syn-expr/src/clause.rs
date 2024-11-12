mod r#let;
#[cfg(test)]
mod tests;

use self::r#let::*;
use super::*;
use builder::VdSynExprBuilder;
use expr::VdSynExprIdx;
use husky_coword::Coword;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_ast::ast::{
    math::{LxMathAstIdx, LxMathAstIdxRange},
    rose::{LxRoseAstData, LxRoseAstIdx},
};
use latex_token::idx::LxRoseTokenIdx;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum VdSynClauseData {
    Let {
        let_token_idx: LxRoseTokenIdx,
        left_dollar_token_idx: LxRoseTokenIdx,
        formula: VdSynExprIdx,
        right_dollar_token_idx: LxRoseTokenIdx,
        resolution: VdSynLetStmtResolution,
    },
    Assume {
        assume_token_idx: LxRoseTokenIdx,
        left_dollar_token_idx: LxRoseTokenIdx,
        formula: VdSynExprIdx,
        right_dollar_token_idx: LxRoseTokenIdx,
    },
    Then {
        then_token_idx: LxRoseTokenIdx,
        left_dollar_token_idx: LxRoseTokenIdx,
        formula: VdSynExprIdx,
        right_dollar_token_idx: LxRoseTokenIdx,
    },
}

pub enum VdSynClauseChild {
    Expr(VdSynExprIdx),
}

impl VdSynClauseData {
    pub(crate) fn children(&self) -> Vec<VdSynClauseChild> {
        match *self {
            VdSynClauseData::Let { formula, .. } => vec![VdSynClauseChild::Expr(formula)],
            VdSynClauseData::Assume { formula, .. } => vec![VdSynClauseChild::Expr(formula)],
            VdSynClauseData::Then { formula, .. } => vec![VdSynClauseChild::Expr(formula)],
        }
    }
}

pub type VdSynClauseArena = Arena<VdSynClauseData>;
pub type VdSynClauseArenaRef<'a> = ArenaRef<'a, VdSynClauseData>;
pub type VdSynClauseIdx = ArenaIdx<VdSynClauseData>;
pub type VdSynClauseIdxRange = ArenaIdxRange<VdSynClauseData>;
pub type VdSynClauseMap<T> = ArenaMap<VdSynClauseData, T>;
pub type VdSynClauseOrderedMap<T> = ArenaOrderedMap<VdSynClauseData, T>;

impl<'db> VdSynExprBuilder<'db> {
    pub(crate) fn parse_clause(
        &mut self,
        token_idx: LxRoseTokenIdx,
        word: Coword,
        asts: &mut Peekable<impl Iterator<Item = LxRoseAstIdx>>,
    ) -> VdSynClauseData {
        match word.data(self.db()) {
            "Let" | "let" => {
                let ast = asts.next().expect("expect a let clause");
                match self.ast_arena()[ast] {
                    LxRoseAstData::Math {
                        left_dollar_token_idx,
                        math_asts,
                        right_dollar_token_idx,
                    } => {
                        let formula = (
                            ((*left_dollar_token_idx + 1)..*right_dollar_token_idx).into(),
                            math_asts,
                        )
                            .to_vd_syn(self);
                        let resolution = self.build_let_stmt_resolution(formula);
                        VdSynClauseData::Let {
                            let_token_idx: token_idx,
                            left_dollar_token_idx,
                            formula,
                            right_dollar_token_idx,
                            resolution,
                        }
                    }
                    LxRoseAstData::TextEdit { ref buffer } => todo!(),
                    LxRoseAstData::Word(lx_rose_token_idx, coword) => todo!(),
                    LxRoseAstData::Punctuation(lx_rose_token_idx, lx_rose_punctuation) => todo!(),
                }
            }
            "Assume" | "assume" | "Suppose" | "suppose" => {
                let ast = asts.next().expect("expect a assume clause");
                match self.ast_arena()[ast] {
                    LxRoseAstData::Math {
                        left_dollar_token_idx,
                        math_asts,
                        right_dollar_token_idx,
                    } => VdSynClauseData::Assume {
                        assume_token_idx: token_idx,
                        left_dollar_token_idx,
                        formula: (
                            ((*left_dollar_token_idx + 1)..*right_dollar_token_idx).into(),
                            math_asts,
                        )
                            .to_vd_syn(self),
                        right_dollar_token_idx,
                    },
                    LxRoseAstData::TextEdit { ref buffer } => todo!(),
                    LxRoseAstData::Word(lx_rose_token_idx, coword) => todo!(),
                    LxRoseAstData::Punctuation(lx_rose_token_idx, lx_rose_punctuation) => todo!(),
                }
            }
            "Then" | "then" => {
                let ast = asts.next().expect("expect a then clause");
                match self.ast_arena()[ast] {
                    LxRoseAstData::Math {
                        left_dollar_token_idx,
                        math_asts,
                        right_dollar_token_idx,
                    } => VdSynClauseData::Then {
                        then_token_idx: token_idx,
                        left_dollar_token_idx,
                        formula: (
                            ((*left_dollar_token_idx + 1)..*right_dollar_token_idx).into(),
                            math_asts,
                        )
                            .to_vd_syn(self),
                        right_dollar_token_idx,
                    },
                    LxRoseAstData::TextEdit { ref buffer } => todo!(),
                    LxRoseAstData::Word(lx_rose_token_idx, coword) => todo!(),
                    LxRoseAstData::Punctuation(lx_rose_token_idx, lx_rose_punctuation) => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}
