use husky_ethereal_signature::HasTypeMethodEtherealSignatures;

use super::*;

pub(super) fn ethereal_ty_field_disambiguation(
    db: &dyn FluffyTermDb,
    ty_term: EtherealTerm,
    ident: Ident,
) -> FluffyTermMaybeResult<&FluffyFieldDisambiguation> {
    // divide into cases for memoization
    match ty_term {
        EtherealTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
            ethereal_ty_ontology_path_ty_field_disambiguation(db, ty_path, ident).just_ok_as_ref()
        }
        EtherealTerm::Application(ty_term) => {
            ethereal_term_application_ty_field_disambiguation(db, ty_term, ident).just_ok_as_ref()
        }
        _ => Nothing,
    }
}

#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn ethereal_ty_ontology_path_ty_field_disambiguation(
    db: &dyn FluffyTermDb,
    ty_path: TypePath,
    ident: Ident,
) -> FluffyTermMaybeResult<FluffyFieldDisambiguation> {
    ethereal_ty_field_disambiguation_aux(db, ty_path, &[], ident, smallvec![])
}

#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn ethereal_term_application_ty_field_disambiguation(
    db: &dyn FluffyTermDb,
    ty_term: EtherealTermApplication,
    ident: Ident,
) -> FluffyTermMaybeResult<FluffyFieldDisambiguation> {
    let application_expansion = ty_term.application_expansion(db);
    match application_expansion.function() {
        TermFunctionReduced::TypeOntology(ty_path) => ethereal_ty_field_disambiguation_aux(
            db,
            ty_path,
            application_expansion.arguments(db),
            ident,
            smallvec![],
        ),
        TermFunctionReduced::Trait(_) | TermFunctionReduced::Other(_) => Nothing,
    }
}

fn ethereal_ty_field_disambiguation_aux<'a>(
    db: &'a dyn FluffyTermDb,
    ty_path: TypePath,
    arguments: &'a [EtherealTerm],
    ident: Ident,
    mut indirections: SmallVec<[FluffyFieldIndirection; 2]>,
) -> FluffyTermMaybeResult<FluffyFieldDisambiguation> {
    let Some(templates) = ty_path
        .ty_method_ethereal_signature_templates(db, ident)
        .into_result_option()? else {
        match ty_path.refine(db) {
            Right(PreludeTypePath::Borrow(borrow_ty_path)) => match borrow_ty_path {
                PreludeBorrowTypePath::Ref => todo!(),
                PreludeBorrowTypePath::RefMut => todo!(),
                PreludeBorrowTypePath::Leash => {
                    indirections.push(FluffyFieldIndirection::Leash);
                    if arguments.len() != 1 {
                        todo!()
                    }
                    return JustOk(ethereal_ty_field_disambiguation(db, arguments[0], ident)?.merge(indirections))
                },
            },
            // ad hoc
            // needs to consider `Deref` `DerefMut` `Carrier`
            _ => return Nothing
        }
    };
    todo!()
}
