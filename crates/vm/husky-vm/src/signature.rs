use husky_coword::{Ident, IdentMap};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TySignature {
    Enum,
    Struct { fields: IdentMap<(Ident, Binding)> },
    Vec,
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct VMMembValDeclarativeSignatureTemplate {
//     pub ident: Ident,
//     pub contract: MembVarContract,
// }
