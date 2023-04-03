use crate::*;
use husky_entity_taxonomy::TypeKind;
use husky_word::IdentPairMap;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct TypeVariant {
    #[id]
    pub path: TypeVariantPath,
    pub ast_idx: AstIdx,
    pub ident: Ident,
}

pub trait HasVariants: Copy {
    type Variant;
    fn variants<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreeResultRef<'a, &'a [(Ident, Self::Variant)]>;
}

impl HasVariants for TypePath {
    type Variant = TypeVariant;

    fn variants<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreeResultRef<'a, &'a [(Ident, Self::Variant)]> {
        ty_path_variants(db, self).as_ref().map(|v| v as &[_])
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_path_variants(
    db: &dyn EntityTreeDb,
    path: TypePath,
) -> EntityTreeResult<IdentPairMap<TypeVariant>> {
    let module_path = path.module_path(db);
    let ast_sheet = db.ast_sheet(module_path)?;
    match path.ty_kind(db) {
        TypeKind::Enum | TypeKind::Inductive => (),
        _ => return Ok(Default::default()),
    }
    let variants = ast_sheet
        .iter()
        .find_map(|ast| match ast {
            Ast::Defn {
                block:
                    DefnBlock::Type {
                        path: path0,
                        variants,
                    },
                ..
            } if *path0 == path => Some(variants.expect("guaranteed by husky-ast")),
            _ => None,
        })
        .ok_or(OriginalEntityTreeError::InvalidTypePath(path))?;
    Ok(ast_sheet
        .indexed_iter(variants.ast_idx_range())
        .map(|(ast_idx, variant_ast)| match variant_ast {
            Ast::TypeVariant {
                token_group_idx,
                path,
                ident_token,
                ..
            } => {
                let ident = ident_token.ident();
                (ident, TypeVariant::new(db, *path, ast_idx, ident))
            }
            _ => unreachable!(),
        })
        .collect())
}
