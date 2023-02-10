use super::*;
use husky_print_utils::p;
use idx_arena::Arena;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub(crate) enum ExprTypeExpectation {
    None,
    /// expect that the type of the expression
    /// is equal to `Type u` for some universe u
    TypeType,
    CastibleAsBool,
    FrameVariableType,
    Return {
        ty: Option<ReducedTerm>,
    },
    ImplicitlyConvertibleTo {
        ty: LocalTerm,
    },
}

impl ExprTypeExpectation {
    pub(crate) fn term(self) -> Option<ReducedTerm> {
        match self {
            ExprTypeExpectation::None => None,
            ExprTypeExpectation::TypeType => None,
            ExprTypeExpectation::CastibleAsBool => None,
            ExprTypeExpectation::FrameVariableType => None,
            ExprTypeExpectation::Return { ty } => ty,
            ExprTypeExpectation::ImplicitlyConvertibleTo { ty } => ty.resolved(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LocalTermExpectationRule {
    src_expr_idx: ExprIdx,
    expectee: LocalTerm,
    variant: LocalTermExpectationRuleVariant,
    resolve_progress: LocalTermExpectationResolveProgress,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct LocalTermExpectationResolved {
    pub(crate) implicit_conversion: LocalTermImplicitConversion,
    pub(crate) local_term: LocalTerm,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LocalTermExpectationResolveProgress {
    Unresolved,
    Resolved(LocalTermExpectationResolved),
    Err(LocalTermExpectationResolveError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalTermExpectationResolveError {
    #[error("original {0}")]
    Original(#[from] OriginalLocalTermExpectationResolveError),
    #[error("derived {0}")]
    Derived(#[from] DerivedLocalTermExpectationResolveError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalLocalTermExpectationResolveError {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedLocalTermExpectationResolveError {
    #[error("target substitution failure")]
    TargetSubstitutionFailure,
    #[error("duplication")]
    Duplication(LocalTermExpectationRuleIdx),
    #[error("unresolved local term")]
    UnresolvedLocalTerm,
}

impl LocalTermExpectationResolveProgress {
    // it will use derived type error
    pub(crate) fn duplicate(&self, src: LocalTermExpectationRuleIdx) -> Self {
        match self {
            LocalTermExpectationResolveProgress::Unresolved => {
                LocalTermExpectationResolveProgress::Unresolved
            }
            LocalTermExpectationResolveProgress::Resolved(expectation_resolved) => {
                LocalTermExpectationResolveProgress::Resolved(*expectation_resolved)
            }
            LocalTermExpectationResolveProgress::Err(_) => {
                LocalTermExpectationResolveProgress::Err(
                    DerivedLocalTermExpectationResolveError::Duplication(src).into(),
                )
            }
        }
    }

    pub(crate) fn reduced_term(&self) -> Option<ReducedTerm> {
        match self {
            LocalTermExpectationResolveProgress::Unresolved => None,
            LocalTermExpectationResolveProgress::Resolved(LocalTermExpectationResolved {
                local_term,
                ..
            }) => match local_term {
                LocalTerm::Resolved(reduced_term) => Some(*reduced_term),
                LocalTerm::Unresolved(_) => todo!(),
            },
            LocalTermExpectationResolveProgress::Err(_) => None,
        }
    }
}

impl LocalTermExpectationRule {
    pub(crate) fn variant(&self) -> &LocalTermExpectationRuleVariant {
        &self.variant
    }

    pub(crate) fn resolve_progress(&self) -> &LocalTermExpectationResolveProgress {
        &self.resolve_progress
    }

    pub(crate) fn target(&self) -> LocalTerm {
        self.expectee
    }

    pub fn src_expr_idx(&self) -> ExprIdx {
        self.src_expr_idx
    }

    fn resolve(&mut self, expectation_resolved: LocalTermExpectationResolved) {
        self.resolve_progress = LocalTermExpectationResolveProgress::Resolved(expectation_resolved)
    }

    pub(super) fn err(&mut self, error: LocalTermExpectationResolveError) {
        self.resolve_progress = LocalTermExpectationResolveProgress::Err(error)
    }
}

pub(super) struct LocalTermExpectationEffect {
    expectation_resolved: LocalTermExpectationResolved,
    actions: Vec<TermResolveAction>,
}

impl LocalTermExpectationEffect {
    pub(super) fn actions(self) -> Vec<TermResolveAction> {
        self.actions
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub(crate) struct LocalTermExpectationRules {
    arena: Arena<LocalTermExpectationRule>,
    first_unresolved_expectation: usize,
}

impl std::ops::Index<LocalTermExpectationRuleIdx> for LocalTermExpectationRules {
    type Output = LocalTermExpectationRule;

    fn index(&self, index: LocalTermExpectationRuleIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl LocalTermExpectationRules {
    pub(super) fn unresolved_rule_iter(
        &self,
    ) -> impl Iterator<Item = (LocalTermExpectationRuleIdx, &LocalTermExpectationRule)> {
        self.arena
            .indexed_iter_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::Resolved(_)
                | LocalTermExpectationResolveProgress::Err(_) => false,
            })
    }

    pub(super) fn unresolved_expectation_rule_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (LocalTermExpectationRuleIdx, &mut LocalTermExpectationRule)> {
        self.arena
            .indexed_iter_mut_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::Resolved(_)
                | LocalTermExpectationResolveProgress::Err(_) => false,
            })
    }

    pub(super) fn alloc_rule(
        &mut self,
        rule: LocalTermExpectationRule,
    ) -> LocalTermExpectationRuleIdx {
        self.arena.alloc_one(rule)
    }

    pub(super) fn take_effect(
        &mut self,
        rule_idx: LocalTermExpectationRuleIdx,
        effect: &LocalTermExpectationEffect,
    ) {
        self.arena
            .update(rule_idx, |rule| rule.resolve(effect.expectation_resolved))
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn new_expectation_rule(
        &self,
        src_expr_idx: ExprIdx,
        target: LocalTerm,
        variant: LocalTermExpectationRuleVariant,
    ) -> LocalTermExpectationRule {
        match target {
            LocalTerm::Resolved(target) => {
                self.new_resolved_expectation_rule(src_expr_idx, target, variant)
            }
            LocalTerm::Unresolved(target) => {
                self.new_unresolved_expectation_rule(src_expr_idx, target, variant)
            }
        }
    }

    fn new_resolved_expectation_rule(
        &self,
        src_expr_idx: ExprIdx,
        resolved_term: ReducedTerm,
        variant: LocalTermExpectationRuleVariant,
    ) -> LocalTermExpectationRule {
        let db = self.db();
        let reduced_term_menu = self.reduced_term_menu();
        let resolve_progress = match variant {
            LocalTermExpectationRuleVariant::AsBool => {
                match resolved_term {
                    term if term == reduced_term_menu.bool() => {
                        LocalTermExpectationResolveProgress::Resolved(
                            LocalTermExpectationResolved {
                                implicit_conversion: LocalTermImplicitConversion::None,
                                local_term: term.into(),
                            },
                        )
                    }
                    // MOM
                    term if term == reduced_term_menu.i32() => todo!(),
                    term if term == reduced_term_menu.r32() => todo!(),
                    term => todo!(),
                }
            }
            LocalTermExpectationRuleVariant::ImplicitlyConvertibleTo { dst: term } => match term {
                LocalTerm::Resolved(_) => todo!(),
                LocalTerm::Unresolved(_) => LocalTermExpectationResolveProgress::Unresolved,
            },
            LocalTermExpectationRuleVariant::Type => match db.term_ty(resolved_term.term()) {
                Ok(term_ty) => match term_ty.term() {
                    Term::Category(cat) => match cat.universe().raw() {
                        0 => todo!(),
                        _ => LocalTermExpectationResolveProgress::Resolved(
                            LocalTermExpectationResolved {
                                implicit_conversion: LocalTermImplicitConversion::None,
                                local_term: resolved_term.into(),
                            },
                        ),
                    },
                    _ => todo!(),
                },
                Err(_) => todo!(),
            },
            LocalTermExpectationRuleVariant::FrameVariableType => todo!(),
        };
        LocalTermExpectationRule {
            src_expr_idx,
            expectee: resolved_term.into(),
            variant,
            resolve_progress,
        }
    }
    fn new_unresolved_expectation_rule(
        &self,
        src_expr_idx: ExprIdx,
        target: UnresolvedTermIdx,
        variant: LocalTermExpectationRuleVariant,
    ) -> LocalTermExpectationRule {
        LocalTermExpectationRule {
            src_expr_idx,
            expectee: target.into(),
            variant,
            resolve_progress: LocalTermExpectationResolveProgress::Unresolved,
        }
    }

    pub(super) fn expectation_rule_effect(
        &self,
        rule: &LocalTermExpectationRule,
        level: LocalTermResolveLevel,
    ) -> Option<LocalTermExpectationEffect> {
        let table = self.local_term_table();
        match rule.variant {
            LocalTermExpectationRuleVariant::AsBool => match rule.expectee {
                LocalTerm::Resolved(_) => todo!(),
                LocalTerm::Unresolved(expectee) => match level {
                    LocalTermResolveLevel::Weak => None,
                    LocalTermResolveLevel::Strong => match table[expectee].unresolved_term() {
                        UnresolvedTerm::ImplicitSymbol(_) => {
                            match table[expectee].unresolved_term() {
                                UnresolvedTerm::ImplicitSymbol(implicit_symbol) => {
                                    let src_expr_idx = rule.src_expr_idx();
                                    p!(self.expr_region_data()[src_expr_idx]);
                                    todo!()
                                }
                                _ => unreachable!(),
                            };
                            todo!()
                        }
                        UnresolvedTerm::TypeApplication { ty, arguments } => todo!(),
                    },
                },
            },
            LocalTermExpectationRuleVariant::ImplicitlyConvertibleTo { dst } => match rule.expectee
            {
                LocalTerm::Resolved(expectee) => match dst {
                    LocalTerm::Resolved(_) => todo!(),
                    LocalTerm::Unresolved(dst) => match table[dst].unresolved_term() {
                        UnresolvedTerm::ImplicitSymbol(_) => match level {
                            LocalTermResolveLevel::Weak => None,
                            LocalTermResolveLevel::Strong => Some(LocalTermExpectationEffect {
                                actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                                    implicit_symbol: dst,
                                    substitution: expectee.into(),
                                }],
                                expectation_resolved: LocalTermExpectationResolved {
                                    implicit_conversion: LocalTermImplicitConversion::None,
                                    local_term: expectee.into(),
                                },
                            }),
                        },
                        UnresolvedTerm::TypeApplication { ty, arguments } => todo!(),
                    },
                },
                LocalTerm::Unresolved(expectee) => match table[expectee].unresolved_term() {
                    UnresolvedTerm::ImplicitSymbol(_) => match level {
                        LocalTermResolveLevel::Weak => None,
                        LocalTermResolveLevel::Strong => Some(LocalTermExpectationEffect {
                            actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                                implicit_symbol: expectee,
                                substitution: dst,
                            }],
                            expectation_resolved: LocalTermExpectationResolved {
                                implicit_conversion: LocalTermImplicitConversion::None,
                                local_term: dst,
                            },
                        }),
                    },
                    UnresolvedTerm::TypeApplication { ty, arguments } => todo!(),
                },
            },
            LocalTermExpectationRuleVariant::Type => todo!(),
            LocalTermExpectationRuleVariant::FrameVariableType => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum LocalTermExpectationRuleVariant {
    AsBool,
    ImplicitlyConvertibleTo { dst: LocalTerm },
    Type,
    FrameVariableType,
}
