use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeAsTraitImplBlock {
    #[id]
    pub id: TypeAsTraitImplBlockId,
    pub ast_idx: AstIdx,
    pub body: AstIdxRange,
}

impl TypeAsTraitImplBlock {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        body: AstIdxRange,
        ty: TypePath,
        trai: TraitPath,
    ) -> Self {
        todo!()
    }

    pub fn module(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.id(db).module
    }

    pub fn ty(self, db: &dyn EntityTreeDb) -> TypePath {
        self.id(db).ty
    }

    pub fn trai(self, db: &dyn EntityTreeDb) -> TraitPath {
        self.id(db).trai
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct TypeAsTraitImplBlockId {
    module: ModulePath,
    ty: TypePath,
    trai: TraitPath,
    disambiguator: u8,
}

impl TypeAsTraitImplBlockId {
    pub fn module(&self) -> ModulePath {
        self.module
    }

    pub fn ty(&self) -> TypePath {
        self.ty
    }

    pub fn trai(&self) -> TraitPath {
        self.trai
    }

    pub fn disambiguator(&self) -> u8 {
        self.disambiguator
    }
}