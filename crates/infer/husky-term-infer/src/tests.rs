mod db;

use crate::*;
use db::*;
use husky_expect_test_utils::expect_test_husky_to_rust;
use husky_expr_syntax::{parse_expr, ExprArena};
use husky_print_utils::epin;
use husky_term::TermDb;
use husky_token::*;

#[test]
fn test_decl() {
    expect_test_husky_to_rust("decl", &debug_decl);

    fn debug_decl(text: &str) -> String {
        let db = TermInferTestsDb::new();
        // let (arena, expr) = db.parse_expr_from_text(text);
        // let mut sheet = TermSheet::new(&arena);
        // let term_menu = db.term_menu();
        // let mut ctx = InferContext::new(&db, &mut sheet, &arena, expr, &term_menu);
        // let term = ctx.term_result().unwrap();
        // format!("{:?}", db.decl(term.path()))
        todo!()
    }
}

#[test]
fn test_infer_ty_works() {
    // expect_test::<String, _>("", debug_infer_ty);

    fn debug_infer_ty(text: &str) -> String {
        todo!()
        //         let db = TermInferTestsDb::new();
        //         let (arena, expr) = db.parse_expr_from_text(text);
        //         let mut sheet = TermSheet::new(&arena);
        //         let term_menu = db.term_menu();
        //         InferContext::new(&db, &mut sheet, &arena, expr, &term_menu).run();
        //         format!(
        //             r#"raw expr arena:
        // {}
        // ty infer sheet:
        // {}"#,
        //             textwrap::indent(&format!("{:#?}", arena), "    "),
        //             textwrap::indent(&format!("{:#?}", sheet), "    "),
        //         )
    }
}
