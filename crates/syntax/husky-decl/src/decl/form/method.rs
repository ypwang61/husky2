use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct MethodDecl {
    pub entity_path: EntityPath,
}