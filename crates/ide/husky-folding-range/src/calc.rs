use crate::*;
use husky_ast::{Ast, AstBlock, AstIdx, AstIdxRange, AstRangeSheet, AstSheet};
use husky_token::{TokenGroupIdx, TokenSheet};
use lsp_types::FoldingRangeKind;

pub(crate) fn calc_folding_ranges(
    ast_sheet: &AstSheet,
    ast_range_sheet: &AstRangeSheet,
) -> Vec<FoldingRange> {
    FoldingRangeCalculator {
        ast_range_sheet,
        ast_sheet,
    }
    .calc_all()
}

struct FoldingRangeCalculator<'a> {
    ast_sheet: &'a AstSheet,
    ast_range_sheet: &'a AstRangeSheet,
}

impl<'a> FoldingRangeCalculator<'a> {
    fn calc_all(mut self) -> Vec<FoldingRange> {
        self.ast_sheet
            .indexed_asts()
            .filter_map(|(idx, ast)| self.calc_ast(idx, ast))
            .collect()
    }

    fn calc_ast(&self, idx: AstIdx, ast: &Ast) -> Option<FoldingRange> {
        let (text_range, kind) = match ast {
            Ast::Err(_, _) | Ast::Mod(_) | Ast::Use(_) | Ast::Comment(_) | Ast::Decor(_) => None,
            Ast::Stmt(block)|
            Ast::BlockDefn(block) => {
                if !block.empty() {
                    Some((self.ast_range_sheet[idx], FoldingRangeKind::Region))
                } else {
                    None
                }
            }
            Ast::IfElseStmts {
                if_stmt,
                elif_stmts,
                else_stmt,
            } => {
                todo!()
                // self.calc_block(if_stmt);
                // for elif_stmt in elif_stmts {
                //     self.calc_block(elif_stmt);
                // }
                // if let Some(else_stmt) = else_stmt {
                //     self.calc_block(else_stmt);
                // }
            }
            Ast::MatchStmts {
                pattern_stmt,
                case_stmts,
            } => todo!(),
        }?;
        Some(FoldingRange {
            start_line: text_range.start.i(),
            start_character: Some(text_range.start.j()),
            end_line: text_range.end.i(),
            end_character: Some(text_range.end.j()),
            kind: Some(kind),
        })
    }

    // fn calc_block(&self, block: &AstBlock) -> FoldingRange {
    //     todo!()
    //     // block.last()
    // }
}