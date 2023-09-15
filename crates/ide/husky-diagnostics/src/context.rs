use crate::*;
use husky_ast::{AstIdx, AstSheet, AstTokenIdxRangeSheet, HasAstSheet};
use husky_expr_ty::*;
use husky_fluffy_term::FluffyTermRegion;
use husky_regional_token::{RegionalTokenIdx, RegionalTokenIdxBase, RegionalTokenStreamState};
use husky_syn_expr::{ExprRangeRegion, SynExprIdx, SynExprRegion, SynExprRegionData};
use husky_token::{TokenGroupIdx, TokenIdx, TokenIdxRange, TokenStreamState};

pub(crate) struct SheetDiagnosticsContext<'a> {
    db: &'a dyn DiagnosticsDb,
    token_sheet_data: &'a TokenSheetData,
    ranged_token_sheet: &'a RangedTokenSheet,
    ast_sheet: &'a AstSheet,
    ast_token_idx_range_sheet: &'a AstTokenIdxRangeSheet,
}

impl<'a> SheetDiagnosticsContext<'a> {
    pub(crate) fn new(db: &'a dyn DiagnosticsDb, module_path: ModulePath) -> Self {
        let ranged_token_sheet = db.ranged_token_sheet(module_path).unwrap();
        let token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        Self {
            db,
            token_sheet_data,
            ranged_token_sheet,
            ast_sheet: module_path.ast_sheet(db).expect("todo"),
            ast_token_idx_range_sheet: db.ast_token_idx_range_sheet(module_path).unwrap(),
        }
    }

    pub(crate) fn db(&self) -> &'a dyn DiagnosticsDb {
        self.db
    }

    pub(crate) fn token_sheet_data(&self) -> &TokenSheetData {
        self.token_sheet_data
    }

    pub(crate) fn token_idx_text_range(&self, token_idx: TokenIdx) -> TextRange {
        self.ranged_token_sheet.token_idx_text_range(token_idx)
    }

    pub(crate) fn token_idx_range_text_range(&self, token_idx_range: TokenIdxRange) -> TextRange {
        self.ranged_token_sheet
            .token_idx_range_text_range(token_idx_range)
    }

    pub(crate) fn token_stream_state_text_range(
        &self,
        token_stream_state: TokenStreamState,
    ) -> TextRange {
        self.ranged_token_sheet
            .token_stream_state_text_range(token_stream_state)
    }

    pub(crate) fn ast_token_idx_range_sheet(&self) -> &AstTokenIdxRangeSheet {
        self.ast_token_idx_range_sheet
    }

    pub(crate) fn ast_text_range(&self, ast_idx: AstIdx) -> TextRange {
        self.ranged_token_sheet
            .tokens_text_range(self.ast_token_idx_range_sheet[ast_idx])
    }

    pub(crate) fn token_group_text_range(&self, token_group_idx: TokenGroupIdx) -> TextRange {
        let token_idx_range = self
            .token_sheet_data()
            .token_group_token_idx_range(token_group_idx);
        self.ranged_token_sheet.tokens_text_range(token_idx_range)
    }

    pub(crate) fn ast_sheet(&self) -> &'a AstSheet {
        self.ast_sheet
    }
}

pub(crate) struct RegionDiagnosticsContext<'a> {
    db: &'a dyn DiagnosticsDb,
    token_sheet_data: &'a TokenSheetData,
    ranged_token_sheet: &'a RangedTokenSheet,
    expr_region_data: &'a SynExprRegionData,
    expr_ty_region: &'a ExprTypeRegion,
    expr_range_region: &'a ExprRangeRegion,
}

impl<'a> RegionDiagnosticsContext<'a> {
    pub(crate) fn new(db: &'a dyn DiagnosticsDb, syn_expr_region: SynExprRegion) -> Self {
        let expr_region_data = &syn_expr_region.data(db);
        let module_path = expr_region_data.path().module_path(db);
        let ranged_token_sheet = db.ranged_token_sheet(module_path).unwrap();
        let token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        let expr_ty_region = db.expr_ty_region(syn_expr_region);
        let expr_range_region = db.expr_range_region(syn_expr_region);
        Self {
            db,
            token_sheet_data,
            ranged_token_sheet,
            expr_region_data,
            expr_ty_region,
            expr_range_region,
        }
    }

    pub(crate) fn db(&self) -> &'a dyn DiagnosticsDb {
        self.db
    }

    pub(crate) fn regional_token_idx_base(&self) -> RegionalTokenIdxBase {
        todo!()
    }

    pub(crate) fn token_sheet_data(&self) -> &TokenSheetData {
        self.token_sheet_data
    }

    pub(crate) fn ranged_token_sheet(&self) -> &RangedTokenSheet {
        self.ranged_token_sheet
    }

    pub(crate) fn expr_ty_region(&self) -> &ExprTypeRegion {
        self.expr_ty_region
    }

    pub(crate) fn fluffy_term_region(&self) -> &FluffyTermRegion {
        self.expr_ty_region.fluffy_term_region()
    }

    pub(crate) fn expr_text_range(&self, expr_idx: SynExprIdx) -> TextRange {
        todo!()
        // self.text_range(self.expr_range_region[expr_idx])
    }

    fn text_range(&self, token_idx_range: TokenIdxRange) -> TextRange {
        assert!(token_idx_range.start().token_idx() < token_idx_range.end().token_idx());
        let first = self
            .ranged_token_sheet
            .token_idx_text_range(token_idx_range.start().token_idx());
        let last = self
            .ranged_token_sheet
            .token_idx_text_range(token_idx_range.end().token_idx() - 1);
        first.join(last)
    }

    pub(crate) fn token_text_range(&self, regional_token_idx: RegionalTokenIdx) -> TextRange {
        self.ranged_token_sheet()
            .token_idx_text_range(regional_token_idx.token_idx(self.regional_token_idx_base()))
    }

    pub(crate) fn token_stream_state_text_range(
        &self,
        regional_token_stream_state: RegionalTokenStreamState,
    ) -> TextRange {
        self.ranged_token_sheet.token_stream_state_text_range(
            regional_token_stream_state.token_stream_state(self.regional_token_idx_base()),
        )
    }
}
