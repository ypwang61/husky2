mod form_body;
mod module_items;
mod trai_for_ty_items;
mod trai_items;
mod ty_items;
mod ty_variants;

pub use self::form_body::*;
pub use self::module_items::*;
pub use self::trai_for_ty_items::*;
pub use self::trai_items::*;
pub use self::ty_items::*;
pub use self::ty_variants::*;

use crate::*;
use husky_entity_path::*;
use husky_entity_taxonomy::*;
use husky_token::*;
use parsec::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum DefnBlock {
    Form {
        path: FormPath,
        body: Option<FormBody>,
    },
    Submodule {
        path: ModulePath,
    },
    Type {
        path: TypePath,
        variants: Option<TypeVariants>,
    },
    Trait {
        path: TraitPath,
        items: Option<TraitItems>,
    },
    // doesn't have a path field because the impl block might be ill-formed
    AssociatedItem {
        body: Option<FormBody>,
    },
}

impl DefnBlock {
    pub fn children(self) -> Option<AstIdxRange> {
        match self {
            DefnBlock::Form { path, body } => body.map(|v| v.ast_idx_range()),
            DefnBlock::Submodule { path } => None,
            DefnBlock::Type { path, variants } => variants.map(|v| v.ast_idx_range()),
            DefnBlock::Trait { path, items } => items.map(|items| items.ast_idx_range()),
            DefnBlock::AssociatedItem { body } => body.map(|v| v.ast_idx_range()),
        }
    }

    pub fn form_body(self) -> Option<FormBody> {
        todo!()
    }

    /// only for non-associated entities
    pub fn entity_path(self) -> Option<EntityPath> {
        match self {
            DefnBlock::Form { path, body } => Some(path.into()),
            DefnBlock::Submodule { path } => Some(path.into()),
            DefnBlock::Type { path, variants } => Some(path.into()),
            DefnBlock::Trait { path, items } => Some(path.into()),
            DefnBlock::AssociatedItem { body } => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum ImplBlockItems {
    Type(TypeItems),
    TraitForType(TraitForTypeItems),
}

impl ImplBlockItems {
    pub fn children(self) -> AstIdxRange {
        match self {
            ImplBlockItems::Type(items) => items.ast_idx_range(),
            ImplBlockItems::TraitForType(items) => items.ast_idx_range(),
        }
    }
}
