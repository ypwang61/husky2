mod associated_item;
mod attr;
mod impl_block;
mod major_item;
mod submodule;
mod ty_variant;

pub use self::associated_item::*;
pub use self::attr::*;
pub use self::major_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::*;
use husky_ast::AstIdx;
use husky_entity_syn_tree::helpers::paths::{module_item_paths, module_item_syn_node_paths};
use husky_syn_expr::helpers::block_expr::parse_defn_block_expr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum ItemSynNodeDefn {
    Submodule(SubmoduleSynNodeDefn),
    MajorItem(MajorItemSynNodeDefn),
    TypeVariant(TypeVariantSynNodeDefn),
    ImplBlock(ImplBlockSynNodeDecl),
    AssociatedItem(AssociatedItemSynNodeDefn),
    Attr(AttrSynNodeDefn),
}

impl ItemSynNodeDefn {
    pub fn syn_node_decl(self, db: &dyn SynDefnDb) -> ItemSynNodeDecl {
        match self {
            ItemSynNodeDefn::Submodule(syn_node_defn) => syn_node_defn.syn_node_decl().into(),
            ItemSynNodeDefn::MajorItem(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            ItemSynNodeDefn::TypeVariant(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            ItemSynNodeDefn::ImplBlock(syn_node_decl) => syn_node_decl.into(),
            ItemSynNodeDefn::AssociatedItem(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            ItemSynNodeDefn::Attr(syn_node_defn) => syn_node_defn.syn_node_decl().into(),
        }
    }

    pub fn body_with_syn_expr_region(
        self,
        db: &dyn SynDefnDb,
    ) -> Option<(SynExprIdx, SynExprRegion)> {
        match self {
            ItemSynNodeDefn::MajorItem(defn) => defn.body_with_syn_expr_region(db),
            ItemSynNodeDefn::AssociatedItem(defn) => defn.body_with_syn_expr_region(db),
            ItemSynNodeDefn::Submodule(_)
            | ItemSynNodeDefn::TypeVariant(_)
            | ItemSynNodeDefn::ImplBlock(_)
            | ItemSynNodeDefn::Attr(_) => None,
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        self.body_with_syn_expr_region(db).map(|v| v.1)
    }
}

pub trait HasSynNodeDefn: Copy {
    type SynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn;
}

impl HasSynNodeDefn for ItemSynNodePath {
    type SynNodeDefn = ItemSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn {
        match self {
            ItemSynNodePath::Submodule(path) => path.syn_node_defn(db).into(),
            ItemSynNodePath::MajorItem(path) => path.syn_node_defn(db).into(),
            ItemSynNodePath::TypeVariant(path) => path.syn_node_defn(db).into(),
            ItemSynNodePath::ImplBlock(path) => path.syn_node_defn(db).into(),
            ItemSynNodePath::AssociatedItem(path) => path.syn_node_defn(db).into(),
            ItemSynNodePath::Attr(path) => path.syn_node_defn(db).into(),
        }
    }
}

pub trait HasNodeDefns: Copy {
    fn node_defns(self, db: &dyn SynDefnDb) -> &[ItemSynNodeDefn];
}

impl HasNodeDefns for ModulePath {
    fn node_defns(self, db: &dyn SynDefnDb) -> &[ItemSynNodeDefn] {
        module_syn_node_defns(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar, return_ref)]
pub(crate) fn module_syn_node_defns(
    db: &dyn SynDefnDb,
    module_path: ModulePath,
) -> Vec<ItemSynNodeDefn> {
    module_item_syn_node_paths(db, module_path)
        .iter()
        .copied()
        .map(|syn_node_path| syn_node_path.syn_node_defn(db))
        .collect()
}

#[test]
fn module_node_defns_works() {
    use tests::*;

    DB::default().ast_expect_test_debug_with_db(
        |db, module_path: ModulePath| module_path.node_defns(db),
        &AstTestConfig::new("module_syn_node_defns"),
    );
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum SynDefn {
    Submodule(SubmoduleSynDefn),
    MajorItem(MajorItemSynDefn),
    TypeVariant(TypeVariantSynDefn),
    ImplBlock(ImplBlockSynDecl),
    AssociatedItem(AssociatedItemSynDefn),
}

impl SynDefn {
    pub fn syn_decl(self, db: &dyn SynDefnDb) -> SynDecl {
        match self {
            SynDefn::Submodule(defn) => SynDecl::Submodule(defn.decl()),
            SynDefn::MajorItem(defn) => defn.syn_decl(db).into(),
            SynDefn::TypeVariant(defn) => defn.decl(db).into(),
            SynDefn::ImplBlock(decl) => decl.into(),
            SynDefn::AssociatedItem(defn) => defn.decl(db).into(),
        }
    }

    pub fn ast_idx(self, _db: &dyn SynDefnDb) -> AstIdx {
        todo!()
        // self.decl(db).ast_idx(db)
    }

    pub fn template_parameters<'a>(self, db: &'a dyn SynDefnDb) -> &'a [TemplateSynParameterData] {
        self.syn_decl(db).template_parameters(db)
    }

    pub fn body_with_syn_expr_region(
        self,
        db: &dyn SynDefnDb,
    ) -> Option<(SynExprIdx, SynExprRegion)> {
        match self {
            SynDefn::Submodule(_) => None,
            SynDefn::MajorItem(defn) => defn.body_with_syn_expr_region(db),
            SynDefn::AssociatedItem(defn) => defn.body_with_syn_expr_region(db),
            SynDefn::TypeVariant(_defn) => None,
            SynDefn::ImplBlock(_) => None,
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDefnDb) -> Option<SynExprRegion> {
        self.body_with_syn_expr_region(db).map(|v| v.1)
    }
}

impl SynDefn {
    pub fn path(self, db: &dyn SynDefnDb) -> ItemPath {
        match self {
            SynDefn::Submodule(defn) => defn.path(db).into(),
            SynDefn::MajorItem(defn) => defn.path(db).into(),
            SynDefn::AssociatedItem(defn) => defn.path(db).into(),
            SynDefn::TypeVariant(defn) => defn.path(db).into(),
            SynDefn::ImplBlock(decl) => decl.path(db).into(),
        }
    }
}

pub trait HasSynDefn: Copy {
    type SynDefn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn>;
}

impl HasSynDefn for ItemPath {
    type SynDefn = SynDefn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn> {
        Ok(match self {
            ItemPath::Submodule(_, path) => path.syn_defn(db)?.into(),
            ItemPath::MajorItem(path) => path.syn_defn(db)?.into(),
            ItemPath::ImplBlock(path) => path.syn_defn(db)?.into(),
            ItemPath::AssociatedItem(path) => path.syn_defn(db)?.into(),
            ItemPath::TypeVariant(_, _) => todo!(),
            ItemPath::Attr(_, _) => todo!(),
        })
    }
}

pub trait HasDefns: Copy {
    fn defns(self, db: &dyn SynDefnDb) -> EntitySynTreeResult<&[SynDefn]>;
}

impl HasDefns for ModulePath {
    fn defns(self, db: &dyn SynDefnDb) -> EntitySynTreeResult<&[SynDefn]> {
        Ok(module_syn_defns(db, self)
            .as_ref()
            .expect("syn tree error is deprecated"))
    }
}

#[salsa::tracked(jar = SynDefnJar, return_ref)]
pub(crate) fn module_syn_defns(
    db: &dyn SynDefnDb,
    module_path: ModulePath,
) -> EntitySynTreeResult<Vec<SynDefn>> {
    Ok(module_item_paths(db, module_path)
        .iter()
        .copied()
        .filter_map(|path| path.syn_defn(db).ok())
        .collect())
}

#[test]
fn module_defns_works() {
    use tests::*;

    DB::default().ast_expect_test_debug_with_db(
        |db, module_path: ModulePath| module_path.defns(db),
        &AstTestConfig::new("module_syn_defns"),
    );
}
