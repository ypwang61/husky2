use crate::{registry::associated_trace::IsAssociatedTraceRegistry, *};
use husky_regional_token::{RegionalTokenIdxBase, RegionalTokenIdxRange};
use husky_text::{HasText, Text};
use husky_token::{RangedTokenSheet, TokenIdx, TokenIdxRange};
use husky_token_info::{TokenInfo, TokenInfoSheetRef, TokenInfoSource};
use husky_trace_protocol::view::{SeparationAfter, TraceViewLineData, TraceViewTokenData};

#[derive(Debug, PartialEq, Eq)]
pub struct TraceViewLines {
    data: Vec<TraceViewLineData>,
}

impl TraceViewLines {
    pub fn data(&self) -> &[TraceViewLineData] {
        self.data.as_ref()
    }

    pub(crate) fn new<AssociatedTraceRegistry>(
        module_path: ModulePath,
        token_idx_range: TokenIdxRange,
        registry: AssociatedTraceRegistry,
        db: &dyn TraceDb,
    ) -> Self
    where
        AssociatedTraceRegistry: IsAssociatedTraceRegistry,
    {
        let mut builder = TraceViewTokensBuilder::new(db, module_path, registry);
        builder.generate_tokens(token_idx_range);
        builder.finish()
    }
}

struct TraceViewTokensBuilder<'a, AssociatedTraceRegistry>
where
    AssociatedTraceRegistry: IsAssociatedTraceRegistry,
{
    db: &'a dyn TraceDb,
    text: Text<'a>,
    ranged_token_sheet: &'a RangedTokenSheet,
    token_info_sheet: TokenInfoSheetRef<'a>,
    lines_data: Vec<TraceViewLineData>,
    tokens_data: Vec<TraceViewTokenData>,
    associated_trace_registry: AssociatedTraceRegistry,
}

impl<'a, AssociatedTraceRegistry> TraceViewTokensBuilder<'a, AssociatedTraceRegistry>
where
    AssociatedTraceRegistry: IsAssociatedTraceRegistry,
{
    fn new(
        db: &'a dyn TraceDb,
        module_path: ModulePath,
        associated_trace_registry: AssociatedTraceRegistry,
    ) -> Self {
        // db.text
        Self {
            db,
            ranged_token_sheet: db.ranged_token_sheet(module_path).unwrap(),
            token_info_sheet: db.token_info_sheet_ref(module_path).unwrap(),
            text: module_path.text(db),
            lines_data: vec![],
            tokens_data: vec![],
            associated_trace_registry,
        }
    }

    fn generate_tokens(&mut self, token_idx_range: TokenIdxRange) {
        for token_idx in token_idx_range {
            self.generate_token(token_idx)
        }
    }

    fn generate_token(&mut self, token_idx: TokenIdx) {
        let text_range = self.ranged_token_sheet.token_text_range(token_idx);
        let text = self.text.text_within(text_range);
        let db = self.db;
        let (token_class, src) = match self.token_info_sheet[token_idx].as_ref() {
            Some(token_info) => (token_info.data().token_class(db), Some(token_info.src())),
            None => (
                self.ranged_token_sheet.token_sheet_data(db)[token_idx].default_token_class(),
                None,
            ),
        };
        let associated_trace_id = src
            .map(|src| {
                self.associated_trace_registry
                    .get_or_issue_associated_trace_id(src, db)
            })
            .flatten();
        // todo: handle inline comments
        let spaces_before: u32 = if token_idx.index() > 0 {
            let prev_text_range = self.ranged_token_sheet.token_text_range(token_idx - 1);
            if prev_text_range.start.line == text_range.end.line {
                text_range.start.col.0 - prev_text_range.end.col.0
            } else {
                self.new_line_if_tokens_data_is_empty();
                text_range.start.col.0
            }
        } else {
            text_range.start.col.0
        };
        self.tokens_data.push(TraceViewTokenData::new(
            text.to_string(),
            token_class,
            spaces_before,
            associated_trace_id,
        ));
    }

    fn new_line_if_tokens_data_is_empty(&mut self) {
        if !self.tokens_data.is_empty() {
            self.lines_data.push(TraceViewLineData::new(std::mem::take(
                &mut self.tokens_data,
            )))
        }
    }

    fn finish(mut self) -> TraceViewLines {
        assert!(self.tokens_data.len() > 0);
        self.lines_data
            .push(TraceViewLineData::new(self.tokens_data));
        TraceViewLines {
            data: self.lines_data,
        }
    }
}
