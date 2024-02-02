use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumTupleVariantEthTemplate {
    pub parent_ty_template: EnumTypeEthTemplate,
    pub instance_constructor_ritchie_ty: RitchieEthTerm,
}

impl EnumTupleVariantEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypeVariantPath,
        tmpl: EnumTupleVariantDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let TypeEthTemplate::Enum(parent_ty_template) = path.parent_ty_path(db).eth_template(db)?
        else {
            unreachable!()
        };
        let instance_constructor_ty =
            RitchieEthTerm::from_declarative(db, tmpl.instance_constructor_ty(db))?;
        Ok(Self::new(db, parent_ty_template, instance_constructor_ty))
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EthTerm {
        self.instance_constructor_ritchie_ty(db).into()
    }
}
