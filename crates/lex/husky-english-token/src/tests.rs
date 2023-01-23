use super::*;
use expect_test::expect_file;
use husky_entity_path::EntityPathJar;

use husky_vfs::*;
use husky_word::{WordDb, WordJar};
use salsa::Database;
use std::sync::Arc;

#[salsa::db(WordJar, VfsJar, EnglishTokenJar)]
#[derive(Default)]
struct DB {
    storage: salsa::Storage<Self>,
}

impl Database for DB {}

fn err(input: &str, err: EnglishTokenError) {
    let db = DB::default();
    let mut t = EnglishTokenIter::new(&db, input);
    let token = t.next().unwrap().variant().clone();
    assert_eq!(token, EnglishTokenVariant::Err(err));
    assert!(t.next().is_none());
}