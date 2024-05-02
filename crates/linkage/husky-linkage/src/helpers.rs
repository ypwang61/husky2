use crate::*;
use husky_entity_path::{region::RegionPath, ItemPath};
use husky_hir_defn::defn::HasHirDefn;
use husky_place::PlaceRegistry;

impl Linkage {
    pub fn path_and_instantiation_for_definition<'db>(
        self,
        db: &'db salsa::Db,
    ) -> Option<(ItemPath, &LinInstantiation)> {
        Some(match *self.data(db) {
            LinkageData::MajorFunctionRitchie {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::MajorVal {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::MemoizedField {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::MethodRitchie {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::AssocRitchie {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::UnveilAssocRitchie {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::StructConstructor { .. }
            | LinkageData::EnumVariantConstructor { .. }
            | LinkageData::EnumVariantDiscriminator { .. }
            | LinkageData::StructDestructor { .. }
            | LinkageData::EnumVariantDestructor { .. }
            | LinkageData::StructField { .. }
            | LinkageData::EnumVariantField { .. }
            | LinkageData::Index
            | LinkageData::VecConstructor { .. }
            | LinkageData::TypeDefault { .. }
            | LinkageData::EnumUnitToJsonValue { .. } => return None,
        })
    }

    pub fn place_registry(self, db: &::salsa::Db) -> Option<&PlaceRegistry> {
        use husky_sem_expr::helpers::region::sem_expr_region_from_region_path;

        let (path, _) = self.path_and_instantiation_for_definition(db)?;
        Some(
            sem_expr_region_from_region_path(RegionPath::Defn(path), db)
                .data(db)
                .place_registry(),
        )
    }
}
