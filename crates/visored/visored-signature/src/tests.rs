use crate::*;

#[salsa::db(
    husky_coword::jar::CowordJar,
    latex_vfs::jar::LxVfsJar,
    visored_entity_path::jar::VdEntityPathJar,
    visored_term::jar::VdTermJar,
    Jar
)]
pub(crate) struct DB();
