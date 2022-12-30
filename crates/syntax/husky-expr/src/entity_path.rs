use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum EntityPathExpr {
    Root {
        ident: IdentifierToken,
    },
    Subentity {
        parent: EntityPathExprIdx,
        ident: IdentifierToken,
    },
}

pub(crate) type EntityPathExprArena = Arena<EntityPathExpr>;
pub type EntityPathExprIdx = ArenaIdx<EntityPathExpr>;