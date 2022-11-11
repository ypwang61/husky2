mod db;

use crate::*;
use db::*;
use husky_expect_test_utils::*;

#[test]
fn test_infer_ty_works() {
    expect_test::<String, _>("", &|text: &str| -> String {
        let db = TermPatternInferTestsDb::new();
        let (arena, expr) = db.parse_raw_expr_from_text(text);
        let mut sheet = TermPatternInferSheet::new(&arena);
        let term_menu = db.term_menu();
        TermPatternInferContext::new(&db, &arena, expr, &term_menu).write_inference(&mut sheet);
        format!(
            r#"raw expr arena:
{}
ty infer sheet:
{}"#,
            textwrap::indent(&format!("{:#?}", arena), "    "),
            textwrap::indent(&format!("{:#?}", sheet), "    "),
        )
    });
}