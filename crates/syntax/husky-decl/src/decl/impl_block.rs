mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use trai_for_ty_impl_block::*;
pub use ty_impl_block::*;

use super::*;

use husky_token::ImplToken;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum ImplDecl {
    Type(TypeImplBlockDecl),
    TypeAsTrait(TraitForTypeImplBlockDecl),
}

impl ImplDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            ImplDecl::Type(decl) => decl.ast_idx(db),
            ImplDecl::TypeAsTrait(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        _db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        todo!()
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            ImplDecl::Type(decl) => decl.expr_region(db),
            ImplDecl::TypeAsTrait(decl) => decl.expr_region(db),
        }
    }
}
