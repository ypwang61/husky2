use crate::{
    atom::symbol_proxy::Symbol,
    stmt::{RawBranchKind, RawStmtKind},
    transform::utils::*,
    *,
};
use text::{TextRange, TextRanged};
use token::{Special, Token, TokenKind};

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_stmt(
        &mut self,
        keyword: Option<(StmtKeyword, TextRange)>,
        tokens: &[Token],
    ) -> AstResult<RawStmt> {
        Ok(if let Some((keyword, kw_range)) = keyword {
            match keyword {
                StmtKeyword::Let => self.parse_init_stmt(InitKind::Let, kw_range, tokens)?,
                StmtKeyword::Var => self.parse_init_stmt(InitKind::Var, kw_range, tokens)?,
                StmtKeyword::If => {
                    expect_at_least!(tokens, kw_range, 2);
                    expect_block_head!(tokens);
                    RawStmt {
                        range: tokens.into(),
                        kind: RawStmtKind::Branch(RawBranchKind::If {
                            condition: self.parse_expr(&tokens[0..(tokens.len() - 1)])?,
                        }),
                    }
                }
                StmtKeyword::Elif => todo!(),
                StmtKeyword::Else => {
                    expect!(tokens.len() == 1, kw_range, "expect one tokens after");
                    expect!(
                        tokens[0].kind == TokenKind::Special(Special::Colon),
                        tokens[0].range,
                        "expect `:`"
                    );
                    RawStmt {
                        range: tokens.into(),
                        kind: RawStmtKind::Branch(RawBranchKind::Else),
                    }
                }
                StmtKeyword::Switch => todo!(),
                StmtKeyword::Match => todo!(),
                StmtKeyword::Case => todo!(),
                StmtKeyword::DeFault => todo!(),
                StmtKeyword::For => {
                    expect_block_head!(tokens);
                    let expr = self.parse_expr(&tokens[0..(tokens.len() - 1)])?;
                    let expr = &self.arena[expr];
                    match expr.kind {
                        RawExprKind::Opn { opr, ref opds } => todo!(),
                        _ => todo!(),
                    }
                    todo!()
                }
                StmtKeyword::ForExt => todo!(),
                StmtKeyword::While => todo!(),
                StmtKeyword::Do => todo!(),
                StmtKeyword::Break => todo!(),
                StmtKeyword::Return => {
                    expect!(tokens.len() > 0, kw_range, "expect some tokens after");
                    RawStmt {
                        range: tokens.into(),
                        kind: RawStmtKind::Return(self.parse_expr(tokens)?),
                    }
                }
                StmtKeyword::Assert => {
                    expect!(tokens.len() > 0, kw_range, "expect some tokens after");
                    RawStmt {
                        range: tokens.into(),
                        kind: RawStmtKind::Assert(self.parse_expr(tokens)?),
                    }
                }
            }
        } else {
            if tokens.len() > 2 && tokens[1].kind == Special::Assign.into() {
                // declarative initialization
                let varname = identify!(tokens[0]);
                self.symbols
                    .push(Symbol::var(varname, tokens[0].text_range()));
                RawStmt {
                    range: tokens.into(),
                    kind: RawStmtKind::Init {
                        init_kind: InitKind::Decl,
                        varname,
                        initial_value: self.parse_expr(&tokens[2..])?,
                    },
                }
            } else {
                match self.env() {
                    Env::Package => todo!(),
                    Env::Module(_) => todo!(),
                    Env::DatasetConfig | Env::Main | Env::Def | Env::Func => {
                        // declarative return
                        RawStmt {
                            range: tokens.into(),
                            kind: RawStmtKind::Return(self.parse_expr(tokens)?),
                        }
                    }
                    Env::Proc => RawStmt {
                        range: tokens.into(),
                        kind: RawStmtKind::Exec(self.parse_expr(tokens)?),
                    },
                    Env::Test => todo!(),
                }
            }
            // Ok(Stmt::Exec(expr.unwrap()).into())
        })
    }

    fn parse_init_stmt(
        &mut self,
        kind: InitKind,
        kw_range: TextRange,
        tokens: &[Token],
    ) -> AstResult<RawStmt> {
        match kind {
            InitKind::Let | InitKind::Var => match self.env() {
                Env::Proc | Env::Test => (),
                _ => ast_err!(
                    kw_range,
                    format!(
                        "`{}` statement requires env to be `proc` or `test`, but got `{}` instead",
                        kind,
                        self.env()
                    )
                )?,
            },
            InitKind::Decl => todo!(),
        }
        expect_at_least!(tokens, kw_range, 3);
        let varname = identify!(&tokens[0]);
        self.symbols
            .push(Symbol::var(varname, tokens[0].range.clone()));
        expect_kind!(tokens[1], Special::Assign);
        let initial_value = self.parse_expr(&tokens[2..])?;
        Ok(RawStmt {
            range: tokens.into(),
            kind: RawStmtKind::Init {
                init_kind: kind,
                varname,
                initial_value,
            },
        })
    }
}
