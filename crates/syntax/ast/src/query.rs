use entity_route_query::{EntityRouteQueryGroup, ScopeResultArc};
use file::FilePtr;
use fold::Transformer;
use fold::{FoldStorage, FoldedList};
use std::sync::Arc;
use upcast::Upcast;

use crate::*;
use atom::symbol_proxy::{Symbol, SymbolProxy};

#[salsa::query_group(AstQueryGroupStorage)]
pub trait AstSalsaQueryGroup: EntityRouteQueryGroup + Upcast<dyn EntityRouteQueryGroup> {
    fn ast_text(&self, file: FilePtr) -> ScopeResultArc<AstText>;
}

pub trait AstQueryGroup: AstSalsaQueryGroup {
    // fn ast(&self, file: file::FilePtr, token_group_index: usize) -> ScopeResult<&AstResult<Ast>> {
    //     todo!()
    //     // Ok(self
    //     //     .ast_text(file)?
    //     //     .folded_results
    //     //     .fold_iter(token_group_index)
    //     //     .next()
    //     //     .unwrap()
    //     //     .value)
    // }
}

fn ast_text(this: &dyn AstSalsaQueryGroup, id: FilePtr) -> ScopeResultArc<AstText> {
    let tokenized_text = this.tokenized_text(id)?;
    let mut parser = AstTransformer::new(this, this.module(id)?);
    parser.transform_all(tokenized_text.fold_iter(0));
    Ok(Arc::new(parser.finish()))
}

// fn parse_ty(db: &dyn AstSalsaQueryGroup, code: &'static str) -> AstResult<EntityRoutePtr> {
//     let tokens = db.tokenize(code);
//     let symbols = fold::LocalStack::<Symbol>::new();
//     let proxy = SymbolProxy {
//         main: None,
//         db,
//         this_ty: None,
//         symbols: &symbols,
//     };
//     atom::parser::parse_ty(proxy, &tokens, None)
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstText {
    pub file: FilePtr,
    pub arena: RawExprArena,
    pub folded_results: FoldedList<AstResult<Ast>>,
}

impl AstText {
    pub fn errors(&self) -> Vec<&AstError> {
        self.folded_results
            .nodes
            .iter()
            .filter_map(|node| node.value.as_ref().err())
            .collect()
    }
}
