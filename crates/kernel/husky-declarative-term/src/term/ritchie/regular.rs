use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeRitchieRegularParameter {
    contract: TermContract,
    ty: DeclarativeTerm,
}

impl DeclarativeRitchieRegularParameter {
    pub fn new(contract: TermContract, ty: DeclarativeTerm) -> Self {
        Self { contract, ty }
    }

    pub(crate) fn substitute_ty(self, f: impl FnOnce(DeclarativeTerm) -> DeclarativeTerm) -> Self {
        Self {
            contract: self.contract,
            ty: f(self.ty),
        }
    }

    pub fn contract(&self) -> TermContract {
        self.contract
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }

    #[inline(never)]
    pub(super) fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        self.ty.show_with_db_fmt(f, db, ctx)
    }
}

impl salsa::DisplayWithDb for DeclarativeRitchieRegularParameter {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.ty.show_with_db_fmt(f, db, &mut Default::default())
    }
}
