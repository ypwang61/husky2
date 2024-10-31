use crate::{
    clause::VdSynClauseArena,
    expr::{VdSynExprArena, VdSynExprData, VdSynExprIdx},
    phrase::{VdSynPhraseArena, VdSynPhraseData, VdSynPhraseIdx},
    region::VdSynExprRegionData,
    sentence::{VdSynSentenceArena, VdSynSentenceData, VdSynSentenceIdx},
};

pub(crate) struct VdSemExprBuilder<'db> {
    db: &'db ::salsa::Db,
    expr_arena: VdSynExprArena,
    phrase_arena: VdSynPhraseArena,
    clause_arena: VdSynClauseArena,
    sentence_arena: VdSynSentenceArena,
}

impl<'db> VdSemExprBuilder<'db> {
    pub fn finish(self) -> VdSynExprRegionData {
        VdSynExprRegionData::new(
            self.expr_arena,
            self.phrase_arena,
            self.clause_arena,
            self.sentence_arena,
        )
    }
}
