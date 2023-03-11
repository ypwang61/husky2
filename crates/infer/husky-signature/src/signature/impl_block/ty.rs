use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_impl_block_signature(
    db: &dyn SignatureDb,
    decl: TypeImplDecl,
) -> SignatureResult<TypeImplSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let _implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db)?,
        &signature_term_region,
        raw_term_menu,
    );
    let ty = decl.ty(db);
    let ty = match signature_term_region.expr_term(ty.expr()) {
        Ok(ty) => ty,
        Err(_) => todo!(),
    };
    Ok(TypeImplSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(
            decl.implicit_parameters(db)?,
            signature_term_region,
            raw_term_menu,
        ),
        ty,
    ))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeImplSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    pub ty: RawTerm,
}