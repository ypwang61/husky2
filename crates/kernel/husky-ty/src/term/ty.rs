use super::*;

pub(crate) fn term_ty(db: &dyn TypeDb, reduced_term: ReducedTerm) -> TypeResult<ReducedTerm> {
    match reduced_term.term() {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::Entity(path) => entity_path_path_term_ty(db, path),
        Term::Category(cat) => cat
            .ty()
            .map(Into::into)
            .map(|term| calc_reduced_term(db, term))
            .map_err(|e| OriginalTypeError::Term(e).into()),
        Term::Universe(_) => todo!(),
        Term::Curry(_) => todo!(),
        Term::Ritchie(_) => todo!(),
        Term::Abstraction(_) => todo!(),
        Term::Application(term) => application_term_ty(db, term),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
    }
}

pub(crate) fn entity_path_path_term_ty(
    db: &dyn TypeDb,
    path: EntityPath,
) -> TypeResult<ReducedTerm> {
    Err(OriginalTypeError::Todo.into())
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn application_term_ty(
    db: &dyn TypeDb,
    term: TermApplication,
) -> TypeResult<ReducedTerm> {
    Err(OriginalTypeError::Todo.into())
}