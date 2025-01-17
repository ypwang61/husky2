use super::*;
use husky_term_prelude::Variance;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirLifetimeTemplateVariable {
    Explicit {
        attrs: HirTemplateVariableAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    SelfLifetime,
}
