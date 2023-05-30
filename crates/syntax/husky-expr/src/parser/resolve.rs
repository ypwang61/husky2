use super::*;
use husky_entity_taxonomy::{EntityKind, FugitiveKind, ModuleItemKind};
use husky_print_utils::p;
use husky_token::Punctuation;
use salsa::DebugWithDb;
use std::ops::ControlFlow;

pub type TokenResolveResult<T> = ControlFlow<(), T>;

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn resolve_token(
        &mut self,
        token_idx: TokenIdx,
        token: Token,
    ) -> TokenResolveResult<ResolvedToken> {
        TokenResolveResult::Continue(match token {
            Token::Keyword(keyword) => match keyword {
                Keyword::Connection(keyword) => match keyword {
                    ConnectionKeyword::For | ConnectionKeyword::Where => {
                        return TokenResolveResult::Break(())
                    }
                },
                Keyword::Pronoun(pronoun) => match pronoun {
                    PronounKeyword::Crate => todo!(),
                    PronounKeyword::SelfType => match self.allow_self_ty() {
                        AllowSelfType::True => ResolvedToken::AtomicExpr(Expr::SelfType(token_idx)),
                        AllowSelfType::False => ResolvedToken::AtomicExpr(Expr::Err(
                            OriginalExprError::SelfTypeNotAllowed(token_idx).into(),
                        )),
                    },
                    PronounKeyword::SelfValue => match self.peek() {
                        Some(Token::Punctuation(Punctuation::COLON_COLON)) => {
                            todo!()
                        }
                        _ => match self.allow_self_value() {
                            AllowSelfValue::True => {
                                ResolvedToken::AtomicExpr(Expr::SelfValue(token_idx))
                            }
                            AllowSelfValue::False => ResolvedToken::AtomicExpr(Expr::Err(
                                OriginalExprError::SelfValueNotAllowed(token_idx).into(),
                            )),
                        },
                    },

                    PronounKeyword::Super => todo!(),
                },
                _ => ResolvedToken::AtomicExpr(Expr::Err(
                    OriginalExprError::UnexpectedKeyword(token_idx).into(),
                )),
            },
            Token::Ident(ident) => self.resolve_ident(token_idx, ident),
            Token::Label(_) => todo!(),
            Token::Punctuation(punct) => match punct.mapped() {
                PunctuationMapped::Binary(binary) => ResolvedToken::BinaryOpr(token_idx, binary),
                PunctuationMapped::Bra(bra) => ResolvedToken::Bra(token_idx, bra),
                PunctuationMapped::Ket(ket) => match self.last_bra() {
                    Some(bra) => {
                        if bra != ket {
                            p!(bra, ket);
                            p!(self.unfinished_exprs());
                            todo!()
                        }
                        ResolvedToken::Ket(token_idx, ket)
                    }
                    None => return TokenResolveResult::Break(()),
                },
                PunctuationMapped::Suffix(suffix) => ResolvedToken::SuffixOpr(token_idx, suffix),
                PunctuationMapped::LaOrLt => match self.top_expr() {
                    TopExprRef::Unfinished(_) => todo!(),
                    TopExprRef::Finished(expr) => {
                        match expr.base_entity_path(self.db(), &self.parser.expr_arena) {
                            BaseEntityPath::Uncertain {
                                inclination: BaseEntityPathInclination::TypeOrVariant,
                            } => ResolvedToken::Bra(token_idx, Bracket::TemplateAngle),
                            BaseEntityPath::Some(entity_path) => {
                                match entity_path.entity_kind(self.db()) {
                                    EntityKind::Module => todo!(),
                                    EntityKind::ModuleItem {
                                        module_item_kind,
                                        connection,
                                    } => match module_item_kind {
                                        ModuleItemKind::Fugitive(FugitiveKind::Val) => {
                                            ResolvedToken::BinaryOpr(
                                                token_idx,
                                                BinaryComparisonOpr::Less.into(),
                                            )
                                        }
                                        ModuleItemKind::Type(_)
                                        | ModuleItemKind::Fugitive(_)
                                        | ModuleItemKind::Trait => {
                                            ResolvedToken::Bra(token_idx, Bracket::TemplateAngle)
                                        }
                                    },
                                    EntityKind::AssociatedItem {
                                        associated_item_kind,
                                    } => todo!(),
                                    EntityKind::TypeVariant => todo!(),
                                }
                            }
                            _ => ResolvedToken::BinaryOpr(
                                token_idx,
                                BinaryOpr::Comparison(BinaryComparisonOpr::Less),
                            ),
                        }
                    }
                    TopExprRef::None => ResolvedToken::Bra(token_idx, Bracket::HtmlAngle),
                },
                PunctuationMapped::ColonColonLa => todo!(),
                PunctuationMapped::RaOrGt => match (self.last_bra(), self.env_bra()) {
                    (Some(Bracket::TemplateAngle), _) => {
                        ResolvedToken::Ket(token_idx, Bracket::TemplateAngle)
                    }
                    (None, Some(Bracket::TemplateAngle)) => return TokenResolveResult::Break(()),
                    _ => ResolvedToken::BinaryOpr(token_idx, BinaryComparisonOpr::Greater.into()),
                },
                PunctuationMapped::Sheba => ResolvedToken::AtomicExpr(Expr::Err(
                    OriginalExprError::UnexpectedSheba(token_idx).into(),
                )),
                PunctuationMapped::Shr => {
                    ResolvedToken::BinaryOpr(token_idx, BinaryOpr::Shift(BinaryShiftOpr::Shr))
                }
                PunctuationMapped::DeriveAssign => return TokenResolveResult::Break(()),
                PunctuationMapped::Minus => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Minus),
                PunctuationMapped::DoubleVertical => todo!(),
                PunctuationMapped::Tilde => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Tilde),
                PunctuationMapped::Dot => ResolvedToken::Dot(token_idx),
                PunctuationMapped::Colon => match self.last_unfinished_expr() {
                    Some(UnfinishedExpr::SimpleList {
                        opr: UnfinishedListOpr::BoxList { .. },
                        items,
                        ..
                    }) => {
                        if items.len() == 0 && self.finished_expr().is_none() {
                            ResolvedToken::ColonRightAfterLBox(token_idx)
                        } else {
                            match self.token_stream.is_empty() {
                                true => return TokenResolveResult::Break(()),
                                false => ResolvedToken::BinaryOpr(token_idx, BinaryOpr::Ins),
                            }
                        }
                    }
                    _ => match self.peek() {
                        // not end of token group
                        Some(_) => ResolvedToken::BinaryOpr(token_idx, BinaryOpr::Ins),
                        // end of token group
                        None => return TokenResolveResult::Break(()),
                    },
                },
                PunctuationMapped::Comma => {
                    self.reduce(Precedence::ListItem);
                    match self.last_unfinished_expr() {
                        Some(expr) => match expr {
                            UnfinishedExpr::SimpleList { .. } => ResolvedToken::ListItem(token_idx),
                            _ => return TokenResolveResult::Break(()),
                        },
                        None => return TokenResolveResult::Break(()),
                    }
                }
                PunctuationMapped::Vertical => match self.last_unfinished_expr() {
                    Some(UnfinishedExpr::SimpleList {
                        bra: Bracket::Lambda,
                        ..
                    }) => ResolvedToken::Ket(token_idx, Bracket::Lambda),
                    _ => match self.finished_expr().is_some() {
                        true => ResolvedToken::BinaryOpr(
                            token_idx,
                            BinaryOpr::Closed(BinaryClosedOpr::BitOr),
                        ),
                        false => ResolvedToken::Bra(token_idx, Bracket::Lambda),
                    },
                },
                PunctuationMapped::DoubleExclamation => todo!(),
                PunctuationMapped::Semicolon => return TokenResolveResult::Break(()),
                PunctuationMapped::EmptyHtmlKet => return TokenResolveResult::Break(()),
                PunctuationMapped::At => return TokenResolveResult::Break(()),
                PunctuationMapped::AtEq => return TokenResolveResult::Break(()),
                PunctuationMapped::Exclamation => self.resolve_prefix_or_other(
                    token_idx,
                    PrefixOpr::Not,
                    ResolvedToken::SuffixOpr(token_idx, SuffixOpr::UnwrapOrComposeWithNot),
                ),
                PunctuationMapped::Question => self.resolve_prefix_or_other(
                    token_idx,
                    PrefixOpr::Option,
                    ResolvedToken::SuffixOpr(token_idx, SuffixOpr::UnveilOrComposeWithOption),
                ),
                PunctuationMapped::Pound => return TokenResolveResult::Break(()),
                PunctuationMapped::Ambersand => self.resolve_prefix_or_other(
                    token_idx,
                    PrefixOpr::Ref,
                    ResolvedToken::BinaryOpr(token_idx, BinaryOpr::Closed(BinaryClosedOpr::BitOr)),
                ),
                PunctuationMapped::DotDot => todo!(),
                PunctuationMapped::Star => {
                    ResolvedToken::BinaryOpr(token_idx, BinaryOpr::Closed(BinaryClosedOpr::Mul))
                }
                PunctuationMapped::Eq => match self.env() {
                    Some(env) => match env {
                        ExprEnvironment::TypeBeforeEq => match self.last_bra() {
                            Some(_) => todo!(),
                            None => return TokenResolveResult::Break(()),
                        },
                        ExprEnvironment::WithinBracket(_) => todo!(),
                        ExprEnvironment::Condition(_) => todo!(),
                    },
                    None => ResolvedToken::BinaryOpr(token_idx, BinaryOpr::Assign),
                },
                PunctuationMapped::ForAll => todo!(),
                PunctuationMapped::Exists => todo!(),
            },
            Token::WordOpr(opr) => match opr {
                WordOpr::And => ResolvedToken::BinaryOpr(
                    token_idx,
                    BinaryOpr::ShortCircuitLogic(BinaryShortcuitLogicOpr::And),
                ),
                WordOpr::Or => ResolvedToken::BinaryOpr(
                    token_idx,
                    BinaryOpr::ShortCircuitLogic(BinaryShortcuitLogicOpr::Or),
                ),
                WordOpr::As => ResolvedToken::BinaryOpr(token_idx, BinaryOpr::As),
                WordOpr::Be => ResolvedToken::Be(token_idx),
            },
            Token::Literal(literal) => ResolvedToken::AtomicExpr(Expr::Literal(token_idx, literal)),
            Token::Error(error) => {
                ResolvedToken::AtomicExpr(Expr::Err(DerivedExprError::Token(error).into()))
            }
        })
    }

    fn resolve_prefix_or_other(
        &mut self,
        token_idx: TokenIdx,
        prefix_opr: PrefixOpr,
        other: ResolvedToken,
    ) -> ResolvedToken {
        match self.top_expr() {
            TopExprRef::Unfinished(_)
            | TopExprRef::Finished(Expr::List { .. })
            | TopExprRef::Finished(Expr::BoxColonList { .. })
            | TopExprRef::None => ResolvedToken::PrefixOpr(token_idx, prefix_opr),
            TopExprRef::Finished(_) => other,
        }
    }
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    fn resolve_ident(&mut self, token_idx: TokenIdx, ident: Ident) -> ResolvedToken {
        if let Some(opn) = self.last_unfinished_expr() {
            match opn {
                UnfinishedExpr::Binary {
                    punctuation: BinaryOpr::ScopeResolution,
                    lopd,
                    ..
                } => match lopd.base_entity_path(self.db(), &self.parser.expr_arena) {
                    BaseEntityPath::None => todo!(),
                    BaseEntityPath::Some(_) => {
                        p!(
                            lopd,
                            ident.debug(self.parser.db),
                            self.parser.path.debug(self.parser.db)
                        );
                        todo!()
                    }
                    BaseEntityPath::Uncertain { .. } => {
                        return ResolvedToken::AtomicExpr(Expr::Err(
                            OriginalExprError::UnresolvedSubentity { token_idx, ident }.into(),
                        ))
                    }
                    BaseEntityPath::Err => todo!(),
                },
                _ => (),
            }
        }
        ResolvedToken::AtomicExpr(
            match self
                .parser
                .symbol_context
                .resolve_ident(self.db(), token_idx, ident)
            {
                Some(symbol) => match symbol {
                    // Symbol::Variable(variable_idx) => Expr::Variable {
                    //     token_idx,
                    //     symbol_idx: variable_idx,
                    // },
                    Symbol::Entity(entity_path) => {
                        let (entity_path_expr, entity_path) =
                            self.parse_entity_path_expr(token_idx, ident, entity_path);
                        Expr::EntityPath {
                            entity_path_expr,
                            path: entity_path,
                        }
                    }
                    Symbol::Inherited(inherited_symbol_idx, inherited_symbol_kind) => {
                        Expr::InheritedSymbol {
                            ident,
                            token_idx,
                            inherited_symbol_idx,
                            inherited_symbol_kind,
                        }
                    }
                    Symbol::Local(current_symbol_idx, current_symbol_kind) => Expr::CurrentSymbol {
                        ident,
                        token_idx,
                        current_symbol_idx,
                        current_symbol_kind,
                    },
                    //  Expr::EntityPath(entity_path),
                },
                None => Expr::Err(OriginalExprError::UnrecognizedIdent { token_idx, ident }.into()),
            },
        )
    }
}

#[derive(Debug)]
pub(crate) enum ResolvedToken {
    AtomicExpr(Expr),
    BinaryOpr(TokenIdx, BinaryOpr),
    PrefixOpr(TokenIdx, PrefixOpr),
    SuffixOpr(TokenIdx, SuffixOpr),
    Bra(TokenIdx, Bracket),
    Ket(TokenIdx, Bracket),
    Dot(TokenIdx),
    ListItem(TokenIdx),
    Be(TokenIdx),
    ColonRightAfterLBox(TokenIdx),
}
