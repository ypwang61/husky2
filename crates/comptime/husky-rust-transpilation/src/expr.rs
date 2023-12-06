pub(crate) mod pattern;
pub(crate) mod precedence;
pub(crate) mod site;
mod stmt;

pub(crate) use self::precedence::{RustPrecedence, RustPrecedenceRange};

use self::{precedence::hir_eager_expr_precedence, site::HirEagerExprSite};
use crate::{binding::RustBinding, *};
use husky_entity_kind::FugitiveKind;
use husky_entity_path::{MajorItemPath, PrincipalEntityPath};
use husky_hir_eager_expr::{
    coersion::HirEagerCoersion, HirEagerCondition, HirEagerElifBranch, HirEagerElseBranch,
    HirEagerExprData, HirEagerExprIdx, HirEagerExprRegion, HirEagerIfBranch,
    HirEagerLetVariablesPattern, HirEagerPatternExpr, HirEagerPatternExprIdx,
    HirEagerRitchieParameterArgumentMatch, HirEagerStmtData, HirEagerStmtIdx, HirEagerStmtIdxRange,
};
use husky_hir_opr::binary::HirBinaryOpr;
use husky_hir_ty::ritchie::HirEagerContract;
use husky_opr::BinaryClosedOpr;
use husky_stack_location::StackLocationIdx;
use vec_like::SmallVecMap;

impl TranspileToRustWith<HirEagerExprRegion> for (HirEagerExprIdx, HirEagerExprSite) {
    fn transpile_to_rust(mut self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let (slf, mut site) = self;
        let data = slf.data(builder.hir_eager_expr_arena());
        let precedence = hir_eager_expr_precedence(data);
        let needs_deref = false;
        if needs_deref {
            site.rust_bindings.push(RustBinding::Deref)
        }
        if !site.rust_bindings.is_empty() {
            site.rust_precedence_range = RustPrecedenceRange::Geq(RustPrecedence::Prefix)
        }
        let mut wrap_in_some_flag = false;
        for &rust_binding in &*site.rust_bindings {
            match rust_binding {
                RustBinding::Deref | RustBinding::DerefCustomed => {
                    builder.punctuation(RustPunctuation::DerefStar)
                }
                RustBinding::Reref => builder.punctuation(RustPunctuation::Ambersand),
                RustBinding::RerefMut => {
                    builder.punctuation(RustPunctuation::Ambersand);
                    builder.keyword(RustKeyword::Mut)
                }
                RustBinding::SelfValue => (),
                RustBinding::WrapInSome => builder.wrap_in_some_left(&mut wrap_in_some_flag),
            }
        }
        if !wrap_in_some_flag && !site.rust_precedence_range.include(precedence) {
            builder.bracketed_list_with(RustBracket::Par, |builder| {
                site.transpile_hir_eager_expr_to_rust(data, precedence, builder)
            })
        } else {
            site.transpile_hir_eager_expr_to_rust(data, precedence, builder)
        }
        if wrap_in_some_flag {
            builder.wrap_in_some_right()
        }
    }
}

impl HirEagerExprSite {
    // `precedence` is guaranteed to equal `hir_eager_expr_precedence(data)`
    // passed to avoid recomputing it
    fn transpile_hir_eager_expr_to_rust(
        &self,
        data: &HirEagerExprData,
        precedence: RustPrecedence,
        builder: &mut RustTranspilationBuilder<HirEagerExprRegion>,
    ) {
        let geq = |slf: &Self| slf.subexpr(RustPrecedenceRange::Geq(precedence));
        let greater = |slf: &Self| slf.subexpr(RustPrecedenceRange::Greater(precedence));
        let db = builder.db();
        match *data {
            HirEagerExprData::Literal(term_literal) => term_literal.transpile_to_rust(builder),
            HirEagerExprData::PrincipalEntityPath(principal_entity_path) => {
                principal_entity_path.transpile_to_rust(builder);
                match principal_entity_path {
                    PrincipalEntityPath::Module(_) => unreachable!(),
                    PrincipalEntityPath::MajorItem(MajorItemPath::Fugitive(path)) => {
                        match path.fugitive_kind(db) {
                            FugitiveKind::FunctionFn => (),
                            FugitiveKind::Val => builder.bracketed(RustBracket::Par, |_| ()),
                            FugitiveKind::FunctionGn => unreachable!(),
                            FugitiveKind::AliasType => unreachable!(),
                        }
                    }
                    PrincipalEntityPath::TypeVariant(_) => (),
                    PrincipalEntityPath::MajorItem(_) => (),
                }
            }
            HirEagerExprData::ConstSymbol(_) => todo!(),
            HirEagerExprData::Variable(hir_eager_runtime_symbol_idx) => {
                hir_eager_runtime_symbol_idx.transpile_to_rust(builder)
            }
            HirEagerExprData::Binary { lopd, opr, ropd } => match opr {
                HirBinaryOpr::Closed(BinaryClosedOpr::RemEuclid) => {
                    (
                        lopd,
                        self.self_expr_on_site(
                            HirEagerCoersion::TRIVIAL_TRANSIENT,
                            HirEagerContract::Pure,
                        ),
                    )
                        .transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::Dot);
                    builder.rem_eulid();
                    builder.bracketed_list_with(RustBracket::Par, |builder| {
                        (ropd, self.any_precedence()).transpile_to_rust(builder)
                    })
                }
                HirBinaryOpr::Closed(BinaryClosedOpr::Power) => {
                    (
                        lopd,
                        self.self_expr_on_site(
                            HirEagerCoersion::TRIVIAL_TRANSIENT,
                            HirEagerContract::Pure,
                        ),
                    )
                        .transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::Dot);
                    builder.pow();
                    builder.bracketed_list_with(RustBracket::Par, |builder| {
                        (ropd, self.any_precedence()).transpile_to_rust(builder)
                    })
                }
                _ => {
                    (lopd, geq(self)).transpile_to_rust(builder);
                    opr.transpile_to_rust(builder);
                    (ropd, greater(self)).transpile_to_rust(builder)
                }
            },
            HirEagerExprData::Be { src: _, target: _ } => {
                builder.macro_name(RustMacroName::Matches)
            }
            HirEagerExprData::Prefix {
                opr,
                opd_hir_expr_idx,
            } => {
                // todo: check some details
                opr.transpile_to_rust(builder);
                (opd_hir_expr_idx, geq(self)).transpile_to_rust(builder)
            }
            HirEagerExprData::Suffix {
                opd_hir_expr_idx,
                opr,
            } => {
                (opd_hir_expr_idx, geq(self)).transpile_to_rust(builder);
                opr.transpile_to_rust(builder)
            }
            HirEagerExprData::TypeConstructorFnCall {
                function_hir_eager_expr_idx,
                path,
                ref instantiation,
                ref item_groups,
            } => {
                builder.ty_constructor(path);
                builder.bracketed_comma_list(
                    RustBracket::Par,
                    item_groups.iter().map(|item_group| (item_group, self)),
                )
            }
            HirEagerExprData::TypeVariantConstructorCall {
                function_hir_eager_expr_idx,
                path,
                ref instantiation,
                ref item_groups,
            } => {
                (function_hir_eager_expr_idx, geq(self)).transpile_to_rust(builder);
                builder.bracketed_comma_list(
                    RustBracket::Par,
                    item_groups.iter().map(|item_group| (item_group, self)),
                )
            }
            HirEagerExprData::FunctionFnCall {
                function_hir_eager_expr_idx,
                path,
                ref instantiation,
                ref item_groups,
            } => {
                (function_hir_eager_expr_idx, geq(self)).transpile_to_rust(builder);
                builder.bracketed_comma_list(
                    RustBracket::Par,
                    item_groups.iter().map(|item_group| (item_group, self)),
                )
            }
            HirEagerExprData::AssociatedFunctionFnCall {
                function_hir_eager_expr_idx,
                path,
                ref instantiation,
                ref item_groups,
            } => {
                (function_hir_eager_expr_idx, geq(self)).transpile_to_rust(builder);
                builder.bracketed_comma_list(
                    RustBracket::Par,
                    item_groups.iter().map(|item_group| (item_group, self)),
                )
            }
            HirEagerExprData::PropsStructField {
                owner_hir_expr_idx,
                ident,
            } => {
                (owner_hir_expr_idx, geq(self)).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                ident.transpile_to_rust(builder)
            }
            HirEagerExprData::MemoizedField {
                owner_hir_expr_idx,
                ident,
                ..
            } => {
                (owner_hir_expr_idx, geq(self)).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                ident.transpile_to_rust(builder);
                builder.bracketed(RustBracket::Par, |_| ())
            }
            HirEagerExprData::MethodFnCall {
                self_argument,
                ident,
                path,
                ref instantiation,
                ref item_groups,
            } => {
                (self_argument, geq(self)).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                ident.transpile_to_rust(builder);
                builder.bracketed_comma_list(
                    RustBracket::Par,
                    item_groups.iter().map(|item_group| (item_group, self)),
                )
            }
            HirEagerExprData::NewTuple { items: _ } => {
                todo!()
            }
            HirEagerExprData::Index {
                owner_hir_expr_idx,
                ref items,
            } => {
                (owner_hir_expr_idx, geq(self)).transpile_to_rust(builder);
                builder.bracketed_comma_list(
                    RustBracket::Box,
                    items
                        .iter()
                        .copied()
                        .map(|item| (item, self.any_precedence())),
                )
            }
            HirEagerExprData::NewList { ref items } => {
                builder.macro_name(RustMacroName::Vec);
                builder.bracketed_comma_list(
                    RustBracket::Box,
                    items
                        .iter()
                        .copied()
                        .map(|item| (item, self.any_precedence())),
                )
            }
            HirEagerExprData::Block { stmts } => stmts.transpile_to_rust(builder),
            HirEagerExprData::EmptyHtmlTag {
                function_ident: _,
                arguments: _,
            } =>
            /* ad hoc */
            {
                ()
            }
            HirEagerExprData::Todo => {
                builder.macro_name(RustMacroName::Todo);
                builder.bracketed(RustBracket::Par, |_| ())
            }
            HirEagerExprData::Unreachable => {
                builder.macro_name(RustMacroName::Unreachable);
                builder.bracketed(RustBracket::Par, |_| ())
            }
            HirEagerExprData::AssociatedFn {
                associated_item_path,
            } => associated_item_path.transpile_to_rust(builder),
        }
    }
}
impl TranspileToRustWith<HirEagerExprRegion>
    for (&HirEagerRitchieParameterArgumentMatch, &HirEagerExprSite)
{
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let (slf, site) = self;
        match *slf {
            HirEagerRitchieParameterArgumentMatch::Regular(param, hir_eager_expr_idx, coersion) => {
                (
                    hir_eager_expr_idx,
                    site.regular_call_item(param, coersion, builder.db()),
                )
                    .transpile_to_rust(builder)
            }
            HirEagerRitchieParameterArgumentMatch::Variadic => todo!(),
            HirEagerRitchieParameterArgumentMatch::Keyed => todo!(),
        }
    }
}
