use self::signature::impl_block::trai_for_ty_impl_block::EthTraitForTypeImplBlockSignatureBuilderItd;
use super::*;
use husky_dec_signature::signature::assoc_item::trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieDecTemplate;
use husky_eth_term::term::ritchie::{EthRitchieSimpleParameter, EtherealRitchieParameter};

#[salsa::interned(constructor = new)]
pub struct TraitForTypeMethodRitchieEthTemplate {
    pub path: TraitForTypeItemPath,
    pub self_ty: EthTerm,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub self_value_parameter: EthRitchieSimpleParameter,
    #[return_ref]
    pub parenate_parameters: EtherealParenateParameters,
    pub return_ty: EthTerm,
}

impl TraitForTypeMethodRitchieEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        dec_template: TraitForTypeMethodRitchieDecTemplate,
    ) -> EthSignatureResult<Self> {
        let self_ty = EthTerm::ty_from_dec(db, dec_template.self_ty(db))?;
        let template_parameters =
            EthTemplateParameters::from_dec(db, dec_template.template_parameters(db))?;
        let self_value_parameter =
            EthRitchieSimpleParameter::from_dec(db, dec_template.self_value_parameter(db))?;
        let parenate_parameters =
            EtherealParenateParameters::from_dec(db, dec_template.parenate_parameters(db))?;
        let return_ty = EthTerm::ty_from_dec(db, dec_template.return_ty(db))?;
        Ok(Self::new(
            db,
            path,
            self_ty,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
        ))
    }

    pub(super) fn inherit_instantiation_builder(
        self,
        impl_block_signature_builder: EthTraitForTypeImplBlockSignatureBuilderItd,
        db: &::salsa::Db,
    ) -> TraitForTypeMethodRitchieEtherealSignatureBuilder {
        let instantiation_builder = impl_block_signature_builder
            .instantiation_builder(db)
            .merge_with_item_template_parameters(self.template_parameters(db));
        TraitForTypeMethodRitchieEtherealSignatureBuilder::new(
            db,
            self,
            instantiation_builder,
            impl_block_signature_builder.context_itd(db),
        )
    }
}

#[salsa::interned]
pub struct TraitForTypeMethodRitchieEtherealSignatureBuilder {
    pub template: TraitForTypeMethodRitchieEthTemplate,
    #[return_ref]
    pub instantiation_builder: EthInstantiationBuilder,
    pub context_itd: EthSignatureBuilderContextItd,
}

impl TraitForTypeMethodRitchieEtherealSignatureBuilder {
    pub fn context(self, db: &::salsa::Db) -> &EthSignatureBuilderContext {
        self.context_itd(db).context(db)
    }
}

impl TraitForTypeMethodRitchieEtherealSignatureBuilder {
    pub fn try_finish(
        self,
        db: &::salsa::Db,
    ) -> Option<&TraitForTypeMethodRitchieEtherealSignature> {
        trai_for_ty_method_ritchie_ethereal_signature_signature_builder_try_into_signature(db, self)
            .as_ref()
    }
}

#[salsa::tracked(return_ref)]
fn trai_for_ty_method_ritchie_ethereal_signature_signature_builder_try_into_signature(
    db: &::salsa::Db,
    signature_builder: TraitForTypeMethodRitchieEtherealSignatureBuilder,
) -> Option<TraitForTypeMethodRitchieEtherealSignature> {
    // todo: deal with dependent type
    let instantiation = signature_builder
        .instantiation_builder(db)
        .try_into_instantiation()?;
    let template = signature_builder.template(db);
    let ctx = signature_builder.context(db);
    Some(TraitForTypeMethodRitchieEtherealSignature {
        path: template.path(db),
        self_value_parameter: template
            .self_value_parameter(db)
            .instantiate(&instantiation, ctx, db)
            .into(),
        parenate_parameters: template
            .parenate_parameters(db)
            .iter()
            .map(|param| -> EtherealRitchieParameter {
                param.instantiate(&instantiation, ctx, db).into()
            })
            .collect(),
        return_ty: template.return_ty(db).instantiate(&instantiation, ctx, db),
        instantiation,
    })
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitForTypeMethodRitchieEtherealSignature {
    pub path: TraitForTypeItemPath,
    pub instantiation: EthInstantiation,
    pub self_value_parameter: EthRitchieSimpleParameter,
    pub parenate_parameters: SmallVec<[EtherealRitchieParameter; 4]>,
    pub return_ty: EthTerm,
}

impl TraitForTypeMethodRitchieEtherealSignature {
    pub fn parenate_parameters(&self) -> &[EtherealRitchieParameter] {
        &self.parenate_parameters
    }

    pub fn return_ty(&self) -> EthTerm {
        self.return_ty
    }

    pub fn path(&self) -> TraitForTypeItemPath {
        self.path
    }

    pub fn instantiation(&self) -> &EthInstantiation {
        &self.instantiation
    }
}
