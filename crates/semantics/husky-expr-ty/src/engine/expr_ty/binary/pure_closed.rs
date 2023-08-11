use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_closed_expr_ty(
        &mut self,
        lopd: SynExprIdx,
        ropd: SynExprIdx,
        opr: BinaryClosedOpr,
        menu: &EtherealTermMenu,
    ) -> Result<FluffyTerm, ExprTypeError> {
        let Some(lopd_ty) = self.infer_new_expr_ty(lopd, ExpectAnyOriginal) else {
            self.infer_new_expr_ty_discarded(ropd, ExpectAnyDerived);
            Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred)?
        };
        self.infer_new_expr_ty_discarded(ropd, ExpectCoersion::new_pure(self, lopd_ty));
        match lopd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path: Left(PreludeTypePath::Num(_)),
                ..
            }
            | FluffyTermData::TypeOntologyAtPlace {
                ty_path,
                refined_ty_path: Left(PreludeTypePath::Num(_)),
                ..
            } => Ok(TermEntityPath::TypeOntology(ty_path).into()),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ..
            } => todo!(),
            FluffyTermData::TypeOntologyAtPlace { .. } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(hole_kind, _) | FluffyTermData::HoleAtPlace { hole_kind, .. } => {
                match hole_kind {
                    HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => {
                        Ok(lopd_ty)
                    }
                    HoleKind::ImplicitType => todo!(),
                    HoleKind::Any => todo!(),
                }
            }
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::SymbolAtPlace { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
