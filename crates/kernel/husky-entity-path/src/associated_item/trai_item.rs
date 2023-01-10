use super::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct TraitItemPath {
    pub trai_path: ModuleItemPath,
    pub ident: Identifier,
}