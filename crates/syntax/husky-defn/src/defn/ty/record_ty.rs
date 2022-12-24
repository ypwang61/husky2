use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct RecordTypeDefn {
    #[id]
    pub module_item_path: ModuleItemPath,
    #[return_ref]
    pub expr_arena: ExprArena,
    pub decl: RecordTypeDecl,
}