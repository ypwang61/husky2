use super::*;
use husky_entity_tree::node::ty_variant::HasTypeVariantPaths;
use husky_hir_decl::decl::EnumHirDecl;

#[salsa::interned(constructor = pub(super) new)]
pub struct EnumHirDefn {
    pub path: TypePath,
    pub hir_decl: EnumHirDecl,
}

impl From<EnumHirDefn> for MajorItemHirDefn {
    fn from(hir_defn: EnumHirDefn) -> Self {
        MajorItemHirDefn::Type(hir_defn.into())
    }
}

impl From<EnumHirDefn> for HirDefn {
    fn from(hir_defn: EnumHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl EnumHirDefn {
    pub(super) fn deps(self, db: &::salsa::Db) -> HirDefnDeps {
        enum_hir_defn_deps(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        enum_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked]
fn enum_hir_defn_deps(db: &::salsa::Db, hir_defn: EnumHirDefn) -> HirDefnDeps {
    let mut builder = HirDefnDepsBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    for &(_, ty_variant_path) in hir_decl.path(db).ty_variant_paths(db) {
        builder.add_item_path(ty_variant_path)
    }
    builder.finish()
}

#[salsa::tracked]
fn enum_hir_defn_version_stamp(db: &::salsa::Db, hir_defn: EnumHirDefn) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
