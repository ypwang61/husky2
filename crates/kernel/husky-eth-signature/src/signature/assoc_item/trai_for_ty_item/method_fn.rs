use super::*;
use husky_eth_term::term::ritchie::{EthRitchieSimpleParameter, EtherealRitchieParameter};

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar, constructor = new)]
pub struct TraitForTypeMethodFnEthTemplate {
    pub path: TraitForTypeItemPath,
    pub self_ty: EthTerm,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub self_value_parameter: EthRitchieSimpleParameter,
    #[return_ref]
    pub parenate_parameters: EtherealParenateParameters,
    pub return_ty: EthTerm,
}

impl TraitForTypeMethodFnEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        dec_sig_tmpl: TraitForTypeMethodFnDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let self_ty = EthTerm::ty_from_dec(db, dec_sig_tmpl.self_ty(db))?;
        let template_parameters =
            EthTemplateParameters::from_dec(db, dec_sig_tmpl.template_parameters(db))?;
        let self_value_parameter =
            EthRitchieSimpleParameter::from_dec(db, dec_sig_tmpl.self_value_parameter(db))?;
        let parenate_parameters =
            EtherealParenateParameters::from_dec(db, dec_sig_tmpl.parenate_parameters(db))?;
        let return_ty = EthTerm::ty_from_dec(db, dec_sig_tmpl.return_ty(db))?;
        Ok(TraitForTypeMethodFnEthTemplate::new(
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
        db: &::salsa::Db,
        impl_block_signature_builder: EthTraitForTypeImplBlockSignatureBuilder,
    ) -> TraitForTypeMethodFnEtherealSignatureBuilder {
        let instantiation_builder = impl_block_signature_builder
            .instantiation_builder(db)
            .merge_with_item_template_parameters(self.template_parameters(db));
        TraitForTypeMethodFnEtherealSignatureBuilder::new(db, self, instantiation_builder)
    }
}

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TraitForTypeMethodFnEtherealSignatureBuilder {
    pub template: TraitForTypeMethodFnEthTemplate,
    #[return_ref]
    pub instantiation_builder: EtherealInstantiationBuilder,
}

impl TraitForTypeMethodFnEtherealSignatureBuilder {
    pub fn try_finish(self, db: &::salsa::Db) -> Option<&TraitForTypeMethodFnEtherealSignature> {
        trai_for_ty_method_fn_ethereal_signature_signature_builder_try_into_signature(db, self)
            .as_ref()
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn trai_for_ty_method_fn_ethereal_signature_signature_builder_try_into_signature(
    db: &::salsa::Db,
    signature_builder: TraitForTypeMethodFnEtherealSignatureBuilder,
) -> Option<TraitForTypeMethodFnEtherealSignature> {
    // todo: deal with dependent type
    let instantiation = signature_builder
        .instantiation_builder(db)
        .try_into_instantiation()?;
    let template = signature_builder.template(db);
    Some(TraitForTypeMethodFnEtherealSignature {
        path: template.path(db),
        self_value_parameter: template
            .self_value_parameter(db)
            .instantiate(db, &instantiation)
            .into(),
        parenate_parameters: template
            .parenate_parameters(db)
            .iter()
            .map(|param| -> EtherealRitchieParameter {
                param.instantiate(db, &instantiation).into()
            })
            .collect(),
        return_ty: template.return_ty(db).instantiate(db, &instantiation),
        instantiation,
    })
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitForTypeMethodFnEtherealSignature {
    pub path: TraitForTypeItemPath,
    pub instantiation: EthInstantiation,
    pub self_value_parameter: EthRitchieSimpleParameter,
    pub parenate_parameters: SmallVec<[EtherealRitchieParameter; 4]>,
    pub return_ty: EthTerm,
}

impl TraitForTypeMethodFnEtherealSignature {
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
