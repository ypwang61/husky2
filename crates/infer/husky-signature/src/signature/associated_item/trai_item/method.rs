use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn trai_method_signature(
    db: &dyn SignatureDb,
    decl: TraitMethodDecl,
) -> TraitMethodSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    TraitMethodSignature::new(
        db,
        todo!(),
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitMethodSignature {
    #[return_ref]
    pub output_ty: SignatureOutcome<Term>,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
