use crate::*;
use husky_ast::AstDb;
use husky_source_path::SourcePath;
use salsa::DbWithJar;
use upcast::Upcast;

pub trait TermPatternInferDb: DbWithJar<TermPatternInferJar> + TermDb + Upcast<dyn TermDb> {
    fn term_pattern_infer_sheet(&self, file: SourcePath) -> FileResultArc<TermPatternInferSheet>;
}

impl<T> TermPatternInferDb for T
where
    T: DbWithJar<TermPatternInferJar> + TermDb + Upcast<dyn TermDb>,
{
    fn term_pattern_infer_sheet(&self, file: SourcePath) -> FileResultArc<TermPatternInferSheet> {
        todo!()
    }
}

fn term_pattern_infer_sheet(
    _db: &dyn TermPatternInferDb,
    _file: SourcePath,
) -> FileResultArc<TermPatternInferSheet> {
    todo!()
}