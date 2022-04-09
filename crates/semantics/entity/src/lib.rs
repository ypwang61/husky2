mod defn;
mod dependence;
mod query;

pub use defn::*;
pub use query::*;

use defn_head::*;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use entity_syntax::*;
use file::FilePtr;
use semantics_eager::*;
use semantics_lazy::{LazyExpr, LazyExprKind, LazyOpnKind, LazyStmt, LazyStmtKind};
use std::sync::Arc;
use text::TextRange;
