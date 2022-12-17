use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookAncestry {
    crate_path: CratePath,
    paths: Vec<EntityPath>,
}

impl BookAncestry {
    pub fn crate_path(&self) -> CratePath {
        self.crate_path
    }
}

#[salsa::tracked(jar = EntityPathJar, return_ref)]
pub(crate) fn book_ancestry(db: &dyn EntityPathDb, entity_path: EntityPath) -> BookAncestry {
    match entity_path.data(db) {
        EntityPathData::CrateRoot(crate_path) => BookAncestry {
            crate_path,
            paths: vec![entity_path],
        },
        EntityPathData::Childpath { parent, ident } => {
            let mut ancestry = book_ancestry(db, parent).clone();
            ancestry.paths.push(entity_path);
            ancestry
        }
    }
}

#[test]
fn book_ancestry_works() {
    use crate::tests::*;
    fn t(db: &DB, entity_path: EntityPath) -> Vec<String> {
        book_ancestry(db, entity_path)
            .paths
            .iter()
            .map(|path| path.display(db))
            .collect()
    }

    let db = DB::default();
    let menu = db.entity_path_menu();
    expect_test::expect![[r#"
        [
            "crate",
        ]
    "#]]
    .assert_debug_eq(&t(&db, menu.core()));
    expect_test::expect![[r#"
        [
            "crate",
            "crate::num",
            "crate::num::i32",
        ]
    "#]]
    .assert_debug_eq(&t(&db, menu.i32()));
    expect_test::expect![[r#"
        [
            "crate",
            "crate::num",
            "crate::num::i64",
        ]
    "#]]
    .assert_debug_eq(&t(&db, menu.i64()));
    expect_test::expect![[r#"
        [
            "crate",
            "crate::num",
            "crate::num::b32",
        ]
    "#]]
    .assert_debug_eq(&t(&db, menu.b32()));
    expect_test::expect![[r#"
        [
            "crate",
            "crate::num",
            "crate::num::b64",
        ]
    "#]]
    .assert_debug_eq(&t(&db, menu.b64()));
}