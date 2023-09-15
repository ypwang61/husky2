use super::*;
use husky_defn_ast::{DefnAstArenaRef, DefnAstIdx};
use husky_entity_syn_tree::helpers::tokra_region::{DefnTokraRegionData, HasSynDefnTokraRegion};
use husky_print_utils::p;

pub struct SynStmtContext<'a> {
    expr_context: SynExprContext<'a>,
    defn_tokra_region_data: DefnTokraRegionData<'a>,
}

impl<'a> std::ops::Deref for SynStmtContext<'a> {
    type Target = SynExprContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.expr_context
    }
}

impl<'a> std::ops::DerefMut for SynStmtContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.expr_context
    }
}

impl<'a> SynStmtContext<'a> {
    pub fn new<P>(
        syn_node_path: P,
        decl_expr_region: SynExprRegion,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
        db: &'a dyn SynExprDb,
    ) -> Option<Self>
    where
        P: HasSynDefnTokraRegion,
    {
        let module_path = syn_node_path.module_path(db);
        let expr_context = SynExprContext::new(
            db,
            RegionPath::Defn(syn_node_path.into()),
            db.module_symbol_context(module_path).unwrap(),
            Some(decl_expr_region),
            allow_self_type,
            allow_self_value,
        );
        use salsa::DebugWithDb;
        Some(Self {
            expr_context,
            defn_tokra_region_data: syn_node_path.syn_defn_tokra_region(db)?.data(db),
        })
    }

    pub(crate) fn expr_parser<'b>(
        &'b mut self,
        token_group_idx: RegionalTokenGroupIdx,
    ) -> SynExprParser<'a, &'b mut SynExprContext<'a>>
    where
        'a: 'b,
    {
        let token_stream = self.token_group_token_stream(token_group_idx, None);
        SynExprParser::new(self, None, token_stream)
    }

    pub(crate) fn parse_root_body(&mut self) -> SynStmtIdxRange {
        let body = self.defn_tokra_region_data.root_body();
        self.parse_stmts(body)
    }

    pub fn finish(self) -> SynExprRegion {
        self.expr_context.finish()
    }

    pub(crate) fn token_group_token_stream(
        &self,
        token_group_idx: RegionalTokenGroupIdx,
        saved_stream_state: impl Into<Option<RegionalTokenStreamState>>,
    ) -> RegionalTokenStream<'a> {
        // self.token_sheet_data
        //     .token_group_token_stream(token_group_idx, saved_stream_state)
        todo!()
    }

    pub(crate) fn regional_asts(&self) -> DefnAstArenaRef<'a> {
        todo!()
    }

    pub(crate) fn regional_ast_token_idx_range(
        &self,
        defn_ast_idx: DefnAstIdx,
    ) -> RegionalTokenIdxRange {
        todo!()
    }
}
